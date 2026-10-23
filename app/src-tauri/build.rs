use std::fs;

fn main() {
    // PostHog project API key, same root .env, same compile-time baking.
    // A PostHog *project* key is designed to be public in client apps (it
    // can only write events, not read them), so shipping it inside the
    // binary is the intended usage, not a leaked secret. Empty when unset,
    // which makes telemetry a no-op - a fresh clone never sends anything.
    let read_key = |name: &str| -> String {
        fs::read_to_string("../../.env")
            .ok()
            .and_then(|contents| {
                let prefix = format!("{}=", name);
                contents.lines().find_map(|line| {
                    line.trim()
                        .strip_prefix(&prefix)
                        .map(|v| v.trim().trim_matches('"').to_string())
                })
            })
            .unwrap_or_default()
    };
    let posthog_key = read_key("POSTHOG_API_KEY");
    let posthog_host = {
        let h = read_key("POSTHOG_HOST");
        if h.is_empty() {
            "https://us.i.posthog.com".to_string()
        } else {
            h
        }
    };
    println!("cargo:rustc-env=POSTHOG_API_KEY={}", posthog_key);
    println!("cargo:rustc-env=POSTHOG_HOST={}", posthog_host);

    // Was hardcoded here directly until 2026-09-28 - that meant the real
    // secret sat in plaintext in a tracked, public file. Now read from the
    // same gitignored .env as everything else above, and rotated after the
    // exposure (old value revoked; see EC2's own .env, which must carry the
    // same new value for the client and server to agree).
    println!("cargo:rustc-env=CUEPOINT_API_BASE_URL=http://127.0.0.1:43127");
    println!("cargo:rustc-env=CUEPOINT_CLIENT_KEY=");
    println!("cargo:rerun-if-changed=../../.env");

    tauri_build::build()
}
