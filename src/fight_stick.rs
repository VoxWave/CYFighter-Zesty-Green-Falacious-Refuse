use amethyst::{
    core::transform::Transform,
    ecs::{DenseVecStorage, Component, Join, Read, ReadStorage, WriteStorage, System},
    input::InputHandler,
};

#[derive(Component, PartialEq, Debug)]
pub struct Button(pub ButtonType, pub bool);

#[derive(PartialEq, Debug)]
pub enum ButtonType {
    A, B, C, D, E,
}

#[derive(Component, PartialEq, Debug)]
pub struct Stick(pub StickState);

#[derive(PartialEq, Debug)]
pub enum StickState {
    Neutral, Up, Down, Left, Right, UpLeft, UpRight, DownLeft, DownRight,
}

#[derive(PartialEq, Debug)]
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
        let p1_up_down = inputs.axis_value("p1_up_down").unwrap();
        let p1_left_right = inputs.axis_value("p1_left_right").unwrap();
        println!("{:?}, {:?}", p1_up_down, p1_left_right);

        let v_axis = get_thresholded_sign(p1_up_down, 0.5);
        let h_axis = get_thresholded_sign(p1_left_right, 0.5);
        {
            let new_stick_state = map_axes_to_stick_state(h_axis, v_axis);
            let mut p1_stick = (&mut sticks).join().next().unwrap();
            p1_stick.0 = new_stick_state;
        }
        // (&mut transforms, sticks).join().for_each(|(mut stick_transform, stick)| {
        //     stick_transform.translate_xy();
        // });
    }
}

fn map_axes_to_stick_state(h_axis: ThresholdSign, v_axis: ThresholdSign) -> StickState {
    use ThresholdSign::*;
    match (h_axis, v_axis) {
        (Positive, Positive) => StickState::UpRight,
        (Positive, Neutral) => StickState::Right,
        (Positive, Negative) => StickState::DownRight,
        (Neutral, Positive) => StickState::Up,
        (Neutral, Neutral) => StickState::Neutral,
        (Neutral, Negative) => StickState::Down,
        (Negative, Positive) => StickState::UpLeft,
        (Negative, Neutral) => StickState::Left,
        (Negative, Negative) => StickState::DownLeft,
    }
}

enum ThresholdSign {
    Positive, Neutral, Negative,
}

fn get_thresholded_sign(scalar: f64, threshold: f64) -> ThresholdSign {
    use ThresholdSign::*;
    if scalar > threshold.abs() {
        Positive
    } else if scalar < -threshold.abs() {
        Negative
    } else {
        Neutral
    }
}

