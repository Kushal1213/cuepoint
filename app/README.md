# Cuepoint desktop

The Windows client is a Tauri 2 application with a vanilla HTML, CSS, and JavaScript interface backed by Rust.

```powershell
npm install
npm run tauri dev
```

Cuepoint 0.3.1 always talks to the bundled loopback-only service at `127.0.0.1:43127`. Release installers include that service and manage its lifecycle automatically.
