mod mouse;
use mouse::location;
fn main() {
    let (mut a, mut b) = location().unwrap();
    loop {
        let (x, y) = location().unwrap();
        if (x, y) != (a, b) {
            continue;
        }
        println!("x: {} y: {}", x, y);
        (a, b) = (x, y)
    }
}
