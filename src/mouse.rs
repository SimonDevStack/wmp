use std::env;
use std::error::Error;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::str::from_utf8;
pub fn location() -> Result<(i32, i32), Box<dyn Error>> {
    match detect_compositor() {
        "hyprland" => hyprland_location(),
        _ => Ok((0, 0)),
    }
}

fn detect_compositor() -> &'static str {
    "hyprland"
}

fn hyprland_location() -> Result<(i32, i32), Box<dyn Error>> {
    // Hyprland Instance Signature (HIS)
    let his = std::env::var("HYPRLAND_INSTANCE_SIGNATURE")?;
    let xdg_runtime = std::env::var("XDG_RUNTIME_DIR")?;
    let path = format!("{}/hypr/{}/.socket.sock", xdg_runtime, his);
    let mut stream = UnixStream::connect(path)?;
    stream.write_all(b"/cursorpos")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    parse(response)
}

fn parse(data: String) -> Result<(i32, i32), Box<dyn Error>> {
    let (x, y) = data.split_once(',').ok_or("Failed to split")?;
    let y = y.trim();

    let x = x.parse::<i32>()?;
    let y = y.parse::<i32>()?;
    Ok((x, y))
}
