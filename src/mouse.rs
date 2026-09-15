use std::process::Command;
use std::str::from_utf8;
pub fn location() -> (i32, i32) {
    match recognize_compositor() {
        "hyprland" => hyprland_location(),
        _ => (0, 0),
    }
}

fn recognize_compositor() -> &'static str {
    "hyprland"
}

fn hyprland_location() -> (i32, i32) {
    let result = Command::new("hyprctl").arg("cursorpos").output().unwrap();
    parse(result.stdout)
}

// todo: make this idiomatic and safe

fn parse(data: Vec<u8>) -> (i32, i32) {
    // convert bytes to slices
    let data = from_utf8(&data[0..]);
    let (x, y) = data.unwrap().split_once(',').unwrap();
    let y = &y[1..4];
    println!("{x}");
    println!("{y}");
    let x = x.parse::<i32>().unwrap();
    let y = y.parse::<i32>().unwrap();
    (x, y)
}
