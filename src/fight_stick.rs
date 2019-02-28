use amethyst::{
    core::transform::Transform,
    ecs::{DenseVecStorage, Component, Join, Read, ReadStorage, WriteStorage, System},
    input::InputHandler,
};

use crate::{
    game::{VIEW_HEIGHT, VIEW_WIDTH},
    parser::{Command, InputBuffer, parse_button, parse_directional, parse_236_button},
};

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

struct FightStick {
    a: Button,
    b: Button,
    c: Button,
    d: Button,
    e: Button,
    stick: Stick,
}

impl FightStick {
    fn new() -> Self {
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

fn parse(input: InputBuffer) -> Option<Command> {
    match (
        parse_directional(input), 
        parse_236_button(input),
        parse_button(input),
    ) {
        (Some(command), _ , _) => Some(command),
        (_, Some(command), _) => Some(command),
        (_, _, Some(command)) => Some(command),
        _ => None,
    }
}

pub struct POCParseSystem {
    stick: FightStick,
    input_buffer: Vec<FightStickInput>,
}

impl<'s> System<'s> for POCParseSystem {
    type SystemData = Read<'s, InputHandler<String, String>>;

    fn run(&mut self, inputs: Self::SystemData) {
        let p1_up_down = inputs.axis_value("p1_up_down").unwrap();
        let p1_left_right = inputs.axis_value("p1_left_right").unwrap();
        let p1_a_button = inputs.action_is_down("A").unwrap();
        let h_axis = get_axis_position(p1_left_right, 0.5);
        let v_axis = get_axis_position(p1_up_down, 0.5);
        let mut parsed_commands = Vec::new();
        if self.stick.stick.0 != h_axis || self.stick.stick.1 != v_axis {
            self.stick.stick = Stick(h_axis, v_axis);
            self.input_buffer.push(FightStickInput::StickPosition(Stick(h_axis, v_axis)));
            parsed_commands.push(parse(&self.input_buffer));
        }
        if self.stick.a.1 != p1_a_button {
            self.stick.a = Button(self.stick.a.0, p1_a_button);
            self.input_buffer.push(FightStickInput::Button(Button(self.stick.a.0, p1_a_button)));
            parsed_commands.push(&self.input_buffer);
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
        WriteStorage<'s, Stick>,
        WriteStorage<'s, Button>,
        Read<'s, InputHandler<String, String>>
    );

    fn run(&mut self, (mut transforms, mut sticks, mut buttons, inputs): Self::SystemData) {
        use AxisPosition::*;
        let p1_up_down = inputs.axis_value("p1_up_down").unwrap();
        let p1_left_right = inputs.axis_value("p1_left_right").unwrap();
        let the_A_button = inputs.action_is_down("A").unwrap();
        // println!("{:?}, {:?}", p1_up_down, p1_left_right);

        let h_axis = get_axis_position(p1_left_right, 0.5);
        let v_axis = get_axis_position(p1_up_down, 0.5);
        (&mut transforms, &mut sticks).join().for_each(|(mut transform, stick)| {
            stick.0 = h_axis.clone();
            stick.1 = v_axis.clone();
            // println!("{:?}", stick.0);
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