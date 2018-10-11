use specs::System;
use shrev::ReaderId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Input {
    Button(Button),
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
    state: ButtonState,
}

pub enum Orientation {
    FacingRight, FacingLeft,
}

pub enum Command {
    QuarterCircle(Button, Orientation),
    DP(Button, Orientation),
    HalfCircle(Button, Orientation),
    DownDown(Button),
    Button(Button),
    Direction(Direction),
}

pub struct ControlSystem {
    reader_id: ReaderId<InputEvent>,
}

impl<'a> System<'a> for ControlSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {}
}
