use vexide::prelude::*;

struct Robot {
    controller: Controller,
    // drive train
    left_motors: [Motor; 3],
    right_motors: [Motor; 3],

    // bot io
    in_take: Motor,
    out_take: Motor,

    // solenoids (pneumatics)
    match_loader: AdiDigitalOut,
    match_loader_state: bool,
    output_ramp: AdiDigitalOut,
    output_ramp_state: bool,
}


impl Robot {
    async fn toggle_piston_a(&mut self) {
        self.match_loader_state = !match_loader_state;
        _ = self.match_loader.toggle();
    }

    async fn toggle_piston_b(&mut self) {
        self.output_ramp_state = !output_ramp_state;
        _ = self.output_ramp.toggle();
    }
}

impl Compete for Robot {
    async fn autonomous(&mut self) {
        println!("We haven't got to developing this yet.");
    }

    
    async fn driver(&mut self) { 
        // TODO: print that I am tiking it in all caps (good luck);
        loop {
            let controller_state = self
                                    .controller
                                    .state()
                                    .unwrap_or_default();

            // - R sticks vertical motion dictates the robot's forward voltage.
            // - L sticks sideways motion dictates the robot's turning voltage.
            let forward = controller_state.right_stick.x();
            let turn = controller_state.left_stick.y();

            // move the left motors.
            for motor in self.left_motors.iter_mut() {
                _ = motor.set_voltage((forward + turn) * Motor::V5_MAX_VOLTAGE);
            }

            // move the right motors.
            for motor in self.right_motors.iter_mut() {
                _ = motor.set_voltage((forward - turn) * Motor::V5_MAX_VOLTAGE);
            }
            
            if controller_state.button_x.is_now_pressed() {
                self.toggle_piston_a();
            }
            if controller_state.button_up.is_now_pressed() {
                self.toggle_piston_b();
            }

            sleep(Controller::UPDATE_INTERVAL).await;
        }
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    Robot {
        controller: peripherals.primary_controller,
        left_motors: [
            Motor::new(peripherals.port_16, Gearset::Blue, Direction::Forward),
            Motor::new(peripherals.port_18, Gearset::Blue, Direction::Forward),
            Motor::new(peripherals.port_17, Gearset::Blue, Direction::Forward),
        ],
        right_motors: [
            Motor::new(peripherals.port_13, Gearset::Blue, Direction::Reverse),
            Motor::new(peripherals.port_15, Gearset::Blue, Direction::Reverse),
            Motor::new(peripherals.port_15, Gearset::Blue, Direction::Reverse),
        ],
        in_take: Motor::new(peripherals.port_20, Gearset::Blue, Direction::Forward),
        out_take: Motor::new(peripherals.port_10, Gearset::Green, Direction::Forward),
        match_loader: AdiDigitalOut::new(peripherals.adi_a),
        output_ramp: AdiDigitalOut::new(peripherals.adi_b),
        // Off by default.
        match_loader_state: false,  
        output_ramp_state: false,
    }
    .compete()
    .await;
}
