use std::net::{SocketAddr, TcpStream};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::time::Duration;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

const LOCAL_BACKEND_ADDR: &str = "127.0.0.1:43127";

pub struct LocalBackend(pub Mutex<Option<Child>>);

impl LocalBackend {
    pub fn new() -> Self {
        Self(Mutex::new(None))
    }
}

impl Drop for LocalBackend {
    fn drop(&mut self) {
        if let Ok(child) = self.0.get_mut() {
            if let Some(child) = child.as_mut() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }
}

fn backend_is_listening() -> bool {
    let address: SocketAddr = LOCAL_BACKEND_ADDR
        .parse()
        .expect("valid local backend address");
    TcpStream::connect_timeout(&address, Duration::from_millis(100)).is_ok()
}

#[cfg(windows)]
fn hide_window(command: &mut Command) {
    use std::os::windows::process::CommandExt;
    command.creation_flags(0x08000000); // CREATE_NO_WINDOW
}

#[cfg(not(windows))]
fn hide_window(_command: &mut Command) {}

pub fn start(app: &AppHandle) -> Result<(), String> {
    if backend_is_listening() {
        return Ok(());
    }

    let bundled = app
        .path()
        .resolve(
            "resources/backend/cuepoint-backend.exe",
            BaseDirectory::Resource,
        )
        .map_err(|error| format!("Failed to resolve the local backend: {error}"))?;

    let mut command = if bundled.exists() {
        Command::new(bundled)
    } else {
        // Developer fallback: release installers always carry the packaged
        // executable, while `tauri dev` can use the checked-out Python app.
        let script =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../backend/local_server.py");
        let mut command = Command::new("python");
        command.arg(script);
        command
    };

    command
        .args(["--port", "43127", "--parent-pid"])
        .arg(std::process::id().to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    hide_window(&mut command);
    let child = command
        .spawn()
        .map_err(|error| format!("Failed to start Cuepoint's local service: {error}"))?;
    *app.state::<LocalBackend>()
        .0
        .lock()
        .map_err(|_| "Local backend state is unavailable")? = Some(child);

    // PyInstaller's one-file package needs a moment to unpack on first run.
    for _ in 0..80 {
        if backend_is_listening() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Err("Cuepoint's local service did not become ready in time".to_string())
}
