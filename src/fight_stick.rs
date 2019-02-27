use amethyst::{
    core::transform::Transform,
    ecs::{DenseVecStorage, Component, Join, Read, ReadStorage, WriteStorage, System},
    input::InputHandler,
};

use crate::game::{VIEW_HEIGHT, VIEW_WIDTH};

#[derive(Clone, Component, PartialEq, Debug)]
pub struct Button(pub ButtonType, pub bool);

#[derive(Clone, PartialEq, Debug)]
pub enum ButtonType {
    A, B, C, D, E,
}

#[derive(Clone, Component, PartialEq, Debug)]
pub struct Stick(pub AxisPosition, pub AxisPosition);

#[derive(Clone, Debug, PartialEq)]
pub enum AxisPosition {
    Positive, Neutral, Negative,
}

fn get_axis_position(scalar: f64, threshold: f64) -> AxisPosition {
    use AxisPosition::*;
    if scalar > threshold.abs() {
        Positive
    } else if scalar < -threshold.abs() {
        Negative
    } else {
        Neutral
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum FightStickInput {
    Button(Button),
    StickPosition(Stick)
}

pub struct FightStickSystem;

impl<'s> System<'s> for FightStickSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Stick>,
        WriteStorage<'s, Button>,
        Read<'s, InputHandler<String, String>>
    );

    fn run(&mut self, (mut transforms, mut sticks, mut buttons, inputs): Self::SystemData) {
        use AxisPosition::*;
        let p1_up_down = inputs.axis_value("p1_up_down").unwrap();
        let p1_left_right = inputs.axis_value("p1_left_right").unwrap();
        let the_A_button = inputs.action_is_down("A").unwrap();
        println!("{:?}, {:?}", p1_up_down, p1_left_right);

        let h_axis = get_axis_position(p1_left_right, 0.5);
        let v_axis = get_axis_position(p1_up_down, 0.5);
        (&mut transforms, &mut sticks).join().for_each(|(mut transform, stick)| {
            stick.0 = h_axis.clone();
            stick.1 = v_axis.clone();
            println!("{:?}", stick.0);
            let transform_x = match h_axis {
                Positive => 10., 
                Neutral => 0.,
                Negative => -10.,
            };
            let transform_y = match v_axis {
                Positive => 10.,
                Neutral => 0.,
                Negative => -10.,
            };
            transform.set_xyz(VIEW_WIDTH/4. + transform_x, VIEW_HEIGHT/2. + transform_y, 0.);
        });
        (&mut transforms, &mut buttons).join().for_each(|(mut transform, mut button)|{
            match button {
                Button(ButtonType::A, pressed) => {
                    *pressed = the_A_button;
                    if *pressed {
                        transform.set_y(VIEW_HEIGHT/2. + 10.);
                    } else {
                        transform.set_y(VIEW_HEIGHT/2.);
                    }
                }
                _ => {}, 
            }
        });
    }
}