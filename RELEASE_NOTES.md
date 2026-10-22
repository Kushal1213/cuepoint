# Cuepoint 0.3.0

The first Cuepoint release under [@Kushal1213](https://github.com/Kushal1213).

## What is included

- Ask, Guide, and Act modes for screen-aware help.
- Direct questions and selected-region questions from any Windows app.
- On-device or OpenAI voice transcription.
- Visual walkthroughs with points, boxes, and arrows.
- Reviewed or automatic multi-step actions.
- Optional read-only GitHub access and Tavily web search.
- Local Task History and opt-in anonymous diagnostics.
- A fully remapped Cuepoint interface, website, setup guide, privacy guide, and project documentation.

## Before installing

This first package is the safe self-hosted build and connects to `http://localhost:8000`. Start the backend from the source archive with:

```powershell
docker compose up -d
```

Then install Cuepoint and connect a Gemini or OpenAI API key in Settings.

Requires 64-bit Windows 10 version 1903 or later, or Windows 11.
