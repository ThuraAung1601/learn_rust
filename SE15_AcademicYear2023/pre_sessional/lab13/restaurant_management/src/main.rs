mod restaurant;

use restaurant::front_of_house::{hosting, serving};
use restaurant::back_of_house::{kitchen, inventory};

fn main() {
    println!("Restaurant Management System");
    
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    kitchen::cook_order();
    serving::serve_order();
    inventory::check_inventory();
}