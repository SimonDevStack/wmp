struct Mouse {
    y: i32,
    x: i32,
}

impl Mouse {
    fn new() -> Self {
        Mouse { x: 0, y: 0 }
    }
}
