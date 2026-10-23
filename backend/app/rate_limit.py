import time
from collections import defaultdict, deque
from threading import Lock
from fastapi import HTTPException, Request
from app.config import settings

_requests: dict[str, deque[float]] = defaultdict(deque)
_lock = Lock()


def _client_ip(request: Request) -> str:
    # Behind Caddy, request.client.host is the proxy's own docker-network
    # IP for every request - the real client IP is in X-Forwarded-For
    # (which Caddy's reverse_proxy sets by default).
    xff = request.headers.get("x-forwarded-for")
    if xff:
        return xff.split(",")[0].strip()
    return request.client.host if request.client else "unknown"


async def rate_limit(request: Request):
    """Sliding one-minute in-memory limit for model-backed routes."""
    ip = _client_ip(request)
    now = time.monotonic()
    with _lock:
        entries = _requests[ip]
        while entries and now - entries[0] >= 60:
            entries.popleft()
        limited = len(entries) >= settings.rate_limit_per_minute
        if not limited:
            entries.append(now)
    if limited:
        raise HTTPException(status_code=429, detail="Too many requests - slow down and try again in a moment.")
