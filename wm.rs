use std::{env, io};

pub fn get_wm() -> io::Result<String> {
    Ok(env::var("XDG_DESKTOP_SESSION")
        .or_else(|_| env::var("XDG_CURRENT_DESKTOP"))
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| "N/A".to_string()))
}
