use shrev::EventChannel;
use specs::Write;
use std::sync::mpsc::Receiver;
use std::collections::VecDeque;

use shrev::ReaderId;

use specs::System;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Input {
    Button(Button, ButtonState),
    Direction(Direction),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Button {
    A,
    B,
    C,
    D,
    E,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UR,
    UL,
    DR,
    DL,
    Neutral,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ButtonState {
    Pressed,
    Depressed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Player {
    P1,
    P2,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputEvent {
    frame: u64,
    player: Player,
    button: Input,
}

pub enum Orientation {
    Rightwards,
    Leftwards,
}

pub enum JumpType {
    Normal,
    Super,
}

pub enum JumpDirection {
    Backward,
    Neutral,
    Forward,
}

// pub enum Command {
//     QuarterCircle(Button, Orientation),
//     DP(Button, Orientation),
//     HalfCircle(Button, Orientation),
//     DownDown(Button),
//     Charge(),
//     Dash(Orientation),
//     Walk(Orientation),
//     Crouch(),
//     Jump(JumpType, JumpDirection),
//     Button(Button, Direction),
//     ButtonRelease(Button),
// }

pub struct P1ControlChannel(pub EventChannel<InputEvent>);

pub struct P2ControlChannel(pub EventChannel<InputEvent>);

pub struct InputSystem {
    input: Receiver<Input>,
}

impl InputSystem {
    pub fn new(input: Receiver<Input>) -> Self {
        InputSystem {
            input,
        }
    }
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (Write<'a, P1ControlChannel>, Write<'a, P2ControlChannel>);

    fn run(&mut self, (p1_control_channel, p2_control_channel): Self::SystemData) {
        for input in self.input.try_iter() {
            mat
        }
    }
}
