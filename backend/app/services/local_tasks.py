"""Small in-process task runner for the desktop-local Cuepoint backend.

Task payloads are kept only in memory and expire quickly.  This replaces the
Celery/Redis pair that made the old desktop build depend on Docker.
"""

from concurrent.futures import ThreadPoolExecutor
from dataclasses import dataclass
from threading import Lock
import time
import uuid
from typing import Any, Callable


_RESULT_TTL_SECONDS = 5 * 60
_MAX_TASKS = 64
_executor = ThreadPoolExecutor(max_workers=2, thread_name_prefix="cuepoint-agent")
_lock = Lock()


@dataclass
class _Task:
    status: str
    result: Any = None
    updated_at: float = 0.0


_tasks: dict[str, _Task] = {}


def _prune(now: float) -> None:
    expired = [task_id for task_id, task in _tasks.items() if now - task.updated_at > _RESULT_TTL_SECONDS]
    for task_id in expired:
        _tasks.pop(task_id, None)

    if len(_tasks) > _MAX_TASKS:
        oldest = sorted(_tasks, key=lambda task_id: _tasks[task_id].updated_at)
        for task_id in oldest[: len(_tasks) - _MAX_TASKS]:
            _tasks.pop(task_id, None)


def submit(fn: Callable[..., Any], *args: Any) -> str:
    task_id = str(uuid.uuid4())
    now = time.monotonic()
    with _lock:
        _prune(now)
        _tasks[task_id] = _Task(status="PENDING", updated_at=now)

    def run() -> None:
        with _lock:
            task = _tasks.get(task_id)
            if task:
                task.status = "STARTED"
                task.updated_at = time.monotonic()
        try:
            result = fn(*args)
        except Exception as exc:
            with _lock:
                task = _tasks.get(task_id)
                if task:
                    task.status = "FAILURE"
                    task.result = str(exc)
                    task.updated_at = time.monotonic()
        else:
            with _lock:
                task = _tasks.get(task_id)
                if task:
                    task.status = "SUCCESS"
                    task.result = result
                    task.updated_at = time.monotonic()

    _executor.submit(run)
    return task_id


def get(task_id: str) -> tuple[str, Any]:
    now = time.monotonic()
    with _lock:
        _prune(now)
        task = _tasks.get(task_id)
        if task is None:
            return "PENDING", None
        return task.status, task.result
