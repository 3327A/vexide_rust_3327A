mod bot;

use vexide::prelude::*;
use crate::bot::Bot;

#[vexide::main]
async fn main(_peripherals: Peripherals) {
    let bot = Bot {};
    bot.compete().await; // start the auton then go to driver controll.
    sleep(vexide::adi::ADI_UPDATE_INTERVAL).await;
}
