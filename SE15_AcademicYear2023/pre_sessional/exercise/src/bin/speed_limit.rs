fn main() {
    // Input
    let args: Vec<_> = std::env::args().collect();
    let speed = args[1].parse().unwrap_or(0);

    let mut _speed = 0;
    if speed > 120 {
        _speed = speed - 120;
        println!("You are driving above the speed limit by {_speed} km/hr.");
    } else if speed < 40 {
        _speed = 40 - _speed;
        println!("You are driving below the speed limit by {_speed} km/hr.");
    } else {
        println!("You are driving within the speed limit.");
    }
}