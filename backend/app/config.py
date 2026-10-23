from pydantic_settings import BaseSettings, SettingsConfigDict
from pathlib import Path

class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_file=".env", env_file_encoding="utf-8")

    gemini_model: str = "gemini-3.1-flash-lite"
    gemini_api_key: str = ""
    marker_mode: str = "burned_in" # "burned_in" | "coords_only"
    image_max_long_edge: int = 2048
    request_timeout_secs: int = 20
    # Read-only file tools default to the current Windows user's Desktop.
    # Override this environment variable to narrow or move the allowed root.
    cuepoint_fs_root: str = str(Path.home() / "Desktop")
    # Tavily's own remote-hosted MCP server key - a single project-owned key,
    # not per-user like the GitHub token, so it lives in .env like the Gemini
    # key rather than being entered in Settings.
    tavily_api_key: str = ""
    # Per-process cap for model-backed routes. Cheap polling is not gated.
    rate_limit_per_minute: int = 30

settings = Settings()
