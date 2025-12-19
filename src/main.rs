mod bot;

use vexide::prelude::*;
use crate::bot::Bot;

#[vexide::main]
async fn main(_peripherals: Peripherals) {
    let bot = Bot {};
    bot.compete().await; // start the auton
}
