mod mouse;
use mouse::location;
fn main() {
    let (mut a, mut b) = location();
    loop {
        let (x, y) = location();
        if (x, y) != (a, b) {
            continue;
        }
        println!("x: {} y: {}", x, y);
        (a, b) = (x, y)
    }
}
