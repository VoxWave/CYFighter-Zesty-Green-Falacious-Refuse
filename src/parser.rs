use amethyst::{
    ecs::System,
}

use crate::fight_stick::{Button, ButtonType, Stick, AxisPosition::{self, *}, FightStickInput};

pub type InputBuffer<'a> = &'a[FightStickInput];

#[derive(Debug, Clone)]
pub enum Command {
    QuaterCircle(ButtonType, Facing),
    DP(ButtonType, Facing),
    HalfCircle(ButtonType, Facing),
    Direction(AxisPosition, AxisPosition),
    Button(Button),
}

#[derive(Debug, Clone)]
pub enum Facing {
    Left, Right,
}

pub fn parse_button(input: InputBuffer) -> Option<Command> {
    if let Some(FightStickInput::Button(button)) = input.last() {
        Some(Command::Button(button.clone()))
    } else {
        None
    }
}

pub fn parse_directional(input: InputBuffer) -> Option<Command> {
    if let Some(FightStickInput::StickPosition(stick)) = input.last() {
        Some(Command::Direction(stick.0.clone(), stick.1.clone()))
    } else {
        None
    }
}

pub fn parse_236_button(input: InputBuffer) -> Option<Command> {
    if let Some(FightStickInput::Button(Button(button_type, true))) = input.last() {
        if let Some(facing) = find_236_or_214(&input[0..input.len()-1]) {
            Some(Command::QuaterCircle(button_type.clone(), facing))
        } else {
            None
        }
    } else {
        None
    }
}

fn find_236_or_214(input: InputBuffer) -> Option<Facing> {
    if input.len() >= 3 {
        let len = input.len();
        match (&input[len-1], &input[len-2], &input[len-3]) {
            (
                &FightStickInput::StickPosition(Stick(Neutral, Negative)),
                &FightStickInput::StickPosition(Stick(Positive, Negative)),
                &FightStickInput::StickPosition(Stick(Positive, Neutral)),
            ) => Some(Facing::Right),
            (
                &FightStickInput::StickPosition(Stick(Neutral, Negative)),
                &FightStickInput::StickPosition(Stick(Negative, Negative)),
                &FightStickInput::StickPosition(Stick(Negative, Neutral)),
            ) => Some(Facing::Left),
            (
                &FightStickInput::StickPosition(_),
                &FightStickInput::StickPosition(_),
                &FightStickInput::StickPosition(_),
            ) => find_236_or_214(&input[0..input.len()-1]),
            _ => None,
        }
    } else {
        None
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

pub struct ParserSystem {
    pub input_buffer: Vec<FightStickInput>,
}

impl<'s> System<'s> for POCParseSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        
    }
}