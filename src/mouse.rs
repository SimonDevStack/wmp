use std::process::Command;

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
    Command::new("hyprctl").arg("cursorpos").output?;
}
