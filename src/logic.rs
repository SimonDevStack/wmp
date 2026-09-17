use crate::error::Error;
use crate::error::Result;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

/// Returns x and y coordinates
///
/// # Examples
/// ```
/// use wmp::location;
///
/// fn main() {
///     let (x, y) = location().unwrap();
///     if (x, y) == (100, 100) {
///         println!("Your cursor is at perfect 100 on x and y axis");
///     }
/// }
pub fn location() -> Result<(i32, i32)> {
    match detect_compositor() {
        "hyprland" => hyprland_location(),
        _ => Err(Error::CompositorUnsupported),
    }
}

fn detect_compositor() -> &'static str {
    "hyprland"
}

fn hyprland_location() -> Result<(i32, i32)> {
    let his = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").map_err(|_| Error::EnvVarNotFound)?;
    let xdg_runtime = std::env::var("XDG_RUNTIME_DIR").map_err(|_| Error::EnvVarNotFound)?;
    let path = format!("{}/hypr/{}/.socket.sock", xdg_runtime, his);
    let mut stream = UnixStream::connect(path).map_err(|_| Error::SocketErr)?;
    stream
        .write_all(b"/cursorpos")
        .map_err(|_| Error::SocketErr)?;
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .map_err(|_| Error::SocketErr)?;
    parse(response)
}

fn parse(data: String) -> Result<(i32, i32)> {
    let (x, y) = data.split_once(',').ok_or(Error::ParseError)?;
    let y = y.trim();

    let x = x.parse::<i32>().map_err(|_| Error::ParseError)?;
    let y = y.parse::<i32>().map_err(|_| Error::ParseError)?;
    Ok((x, y))
}
