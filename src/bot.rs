use vexide::prelude::*;

pub struct Bot {
    // Motors
    let front_motor_right = Motor::new(peripherals.port_16, Gearset::Blue, Direction::Forward);
    let back_motor_right = Motor::new(peripherals.port_18, Gearset::Blue, Direction::Forward);
    let middle_motor_right = Motor::new(peripherals.port_17, Gearset::Blue, Direction::Forward);

    let back_motor_left = Motor::new(peripherals.port_13, Gearset::Blue, Direction::Reverse);
    let front_motor_left = Motor::new(peripherals.port_15, Gearset::Blue, Direction::Reverse);
    let middle_motor_left = Motor::new(peripherals.port_14, Gearset::Blue, Direction::Reverse);
    
    // Pneumatics
    let piston_a = AdiDigitalOut::new(peripherals.adi_a);
    let piston_a = AdiDigitalOut::new(peripherals.adi_b);
}
