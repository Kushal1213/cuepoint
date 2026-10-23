# Cuepoint 0.3.1

Cuepoint now runs as a self-contained local Windows application.

## What is included

- Ask, Guide, and Act modes for screen-aware help.
- Direct questions and selected-region questions from any Windows app.
- On-device or OpenAI voice transcription.
- Visual walkthroughs with points, boxes, and arrows.
- Reviewed or automatic multi-step actions.
- Optional read-only GitHub access and Tavily web search.
- Local Task History and opt-in anonymous diagnostics.
- A fully remapped Cuepoint interface, website, setup guide, privacy guide, and project documentation.
- A bundled loopback-only FastAPI service that starts and stops with the desktop app.
- In-memory task execution and throttling with no Celery, Redis, Docker, or hosted Cuepoint server.

## Before installing

Install Cuepoint and connect a Gemini or OpenAI API key in Settings. The local service is included and starts automatically. Model, Tavily, GitHub, update, and optional diagnostics requests still require internet access when those features are used.

Requires 64-bit Windows 10 version 1903 or later, or Windows 11.
