mod resty_controller;

use resty_controller::RestyController;

fn main() {
    println!("Hello, world!");

    let resty_con = RestyController::new(31337);
    resty_con.listen();
}
