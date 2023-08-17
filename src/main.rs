extern crate ev3dev_lang_rust;
use ev3dev_lang_rust::Ev3Result;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort, MediumMotor};

use std::time::Duration;

fn main() -> Ev3Result<()> {

    let driver = LargeMotor::list();

    if driver.is_err() {
        let motor1 = MediumMotor::get(MotorPort::OutA)?;
        let motor2 = MediumMotor::get(MotorPort::OutB)?;
        let motor3 = MediumMotor::get(MotorPort::OutC)?;
        let motor4 = MediumMotor::get(MotorPort::OutD)?;

        let mut time = Some(Duration::new(2, 500000000));


        motor1.run_timed(time)?;
        motor2.run_timed(time)?;
        motor3.run_timed(time)?;
        motor4.run_timed(time)?;

        // Run motor.
        motor1.set_duty_cycle_sp(50)?;
        motor2.set_duty_cycle_sp(50)?;
        motor3.set_duty_cycle_sp(50)?;
        motor4.set_duty_cycle_sp(50)?;

        Ok(())
    } else {
        let motor1 = LargeMotor::get(MotorPort::OutA)?;
        let motor2 = LargeMotor::get(MotorPort::OutB)?;
        let motor3 = LargeMotor::get(MotorPort::OutC)?;
        let motor4 = LargeMotor::get(MotorPort::OutD)?;


        let mut time = Some(Duration::new(2, 500000000));

        // Set command "run-direct".
        motor1.run_timed(time)?;
        motor2.run_timed(time)?;
        motor3.run_timed(time)?;
        motor4.run_timed(time)?;

        // Run motor.
        motor1.set_duty_cycle_sp(50)?;
        motor2.set_duty_cycle_sp(50)?;
        motor3.set_duty_cycle_sp(50)?;
        motor4.set_duty_cycle_sp(50)?;

        Ok(())
    }
}
