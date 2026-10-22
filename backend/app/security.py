from fastapi import Header, HTTPException
from app.config import settings


async def verify_client_key(x_cuepoint_client_key: str | None = Header(default=None)):
    """Gate on a shared secret baked into the desktop client. Not real
    per-user auth (one key for every install, extractable from the shipped
    binary) - just a deterrent against random scraping/bots hitting a
    publicly-reachable, quota-metered backend. A no-op locally: if
    CUEPOINT_CLIENT_KEY isn't set (dev's .env), every request passes."""
    if not settings.cuepoint_client_key:
        return
    if x_cuepoint_client_key != settings.cuepoint_client_key:
        raise HTTPException(status_code=401, detail="Invalid or missing client key")
