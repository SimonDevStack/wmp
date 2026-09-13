use std::process::Command;
use std::str::from_utf8;
use std::str::split_once;
pub fn location() -> Result<(i32, i32), Error> {
    match recognize_compositor() {
        "hyprland" => hyprland_location()?,
        _ => (),
    }
}

fn recognize_compositor() -> &str {
    "hyprland"
}

fn hyprland_location() -> Result<(i32, i32), Error> {
    let result = Command::new("hyprctl").arg("cursorpos").output();
}

fn parse(data: Vec<u8>) {
    // convert bytes to slices
    let data = from_utf8(&data[0..]);
    let (x, y) = data.split_once(',').unwrap();
    let x = &x[1..];
}
