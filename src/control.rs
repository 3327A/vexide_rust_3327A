mod bot;

use vexide::prelude::*;
use crate::bot::Bot;


impl Compete for Bot {
    fn toggle_piston_a(&mut self) { self.toggle(); }
    fn toggle_piston_b(&mut self) { self.toggle(); }

    async fn autonomous(&mut self) {

    }
    async fn drive(&mut self) {

    }
}
