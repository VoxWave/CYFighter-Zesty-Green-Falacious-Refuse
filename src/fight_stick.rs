use amethyst::{
    core::transform::Transform,
    ecs::{DenseVecStorage, Component, Join, Read, ReadStorage, WriteStorage, System},
    input::InputHandler,
};

use crate::{
    game::{VIEW_HEIGHT, VIEW_WIDTH},
    parser::{Command, InputBuffer, parse_button, parse_directional, parse_236_button},
};

#[derive(Clone, PartialEq, Debug)]
pub struct Button(pub ButtonType, pub bool);

#[derive(Clone, PartialEq, Debug)]
pub enum ButtonType {
    A, B, C, D, E,
}

#[derive(Clone, PartialEq, Debug)]
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

pub struct FightStick {
    pub a: Button,
    pub b: Button,
    pub c: Button,
    pub d: Button,
    pub e: Button,
    pub stick: Stick,
}

pub struct P1FightStick(pub FightStick);
pub struct P2FightStick(pub FightStick);

impl FightStick {
    pub fn new() -> Self {
        FightStick {
            a: Button(ButtonType::A, false),
            b: Button(ButtonType::A, false),
            c: Button(ButtonType::A, false),
            d: Button(ButtonType::A, false),
            e: Button(ButtonType::A, false),
            stick: Stick(AxisPosition::Neutral, AxisPosition::Neutral),
        }
    }
}

// this system is for the visualisation of the stick and buttons.
// it has some functionality that overlaps with POCParseSystem but
// Refactoring it is a bit too big of an ordeal at the moment so
// I'll leave it ugly for now.
pub struct FightStickSystem;

impl<'s> System<'s> for FightStickSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>
    );

    fn run(&mut self, (mut transforms, mut sticks, mut buttons, inputs): Self::SystemData) {
        use AxisPosition::*;
        let p1_up_down = inputs.axis_value("p1_up_down").unwrap();
        let p1_left_right = inputs.axis_value("p1_left_right").unwrap();
        let p1_a = inputs.action_is_down("p1_a").unwrap();
        let p1_b = inputs.action_is_down("p1_b").unwrap();
        let p1_c = inputs.action_is_down("p1_c").unwrap();
        let p1_d = inputs.action_is_down("p1_d").unwrap();
        let p1_e = inputs.action_is_down("p1_e").unwrap();
        
        let p2_up_down = inputs.axis_value("p2_up_down").unwrap();
        let p2_left_right = inputs.axis_value("p2_left_right").unwrap();
        let p2_a = inputs.action_is_down("p2_a").unwrap();
        let p2_b = inputs.action_is_down("p2_b").unwrap();
        let p2_c = inputs.action_is_down("p2_c").unwrap();
        let p2_d = inputs.action_is_down("p2_d").unwrap();
        let p2_e = inputs.action_is_down("p2_e").unwrap();

        // println!("{:?}, {:?}", p1_up_down, p1_left_right);

        let h_axis = get_axis_position(p1_left_right, 0.5);
        let v_axis = get_axis_position(p1_up_down, 0.5);
    }
}