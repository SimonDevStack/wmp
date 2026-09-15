use std::error::Error;
use std::process::Command;
use std::str::from_utf8;
pub fn location() -> Result<(i32, i32), Box<dyn Error>> {
    match recognize_compositor() {
        "hyprland" => Ok(hyprland_location()?),
        _ => Ok((0, 0)),
    }
}

fn recognize_compositor() -> &'static str {
    "hyprland"
}

fn hyprland_location() -> Result<(i32, i32), Box<dyn Error>> {
    let result = Command::new("hyprctl").arg("cursorpos").output().unwrap();
    Ok(parse(result.stdout)?)
}

// todo: make this idiomatic and safe

fn parse(data: Vec<u8>) -> Result<(i32, i32), Box<dyn Error>> {
    // convert bytes to slices
    let data = from_utf8(&data[0..]);
    let (x, y) = data?.split_once(',').ok_or("Failed to split")?;
    let y = &y[1..].trim_end();

    println!("{x}");
    println!("{y}");
    let x = x.parse::<i32>()?;
    let y = y.parse::<i32>()?;
    Ok((x, y))
}
