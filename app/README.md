# Cuepoint desktop

The Windows client is a Tauri 2 application with a vanilla HTML, CSS, and JavaScript interface backed by Rust.

```powershell
npm install
npm run tauri dev
```

The app targets the local backend by default. Production builds read `CUEPOINT_API_BASE_URL` and `CUEPOINT_CLIENT_KEY` from the repository root `.env`.
