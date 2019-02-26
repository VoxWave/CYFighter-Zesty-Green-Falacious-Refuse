use crate::fight_stick::{Button, ButtonType, Stick, AxisPosition::{self, *}, FightStickInput};

type InputBuffer<'a> = &'a[FightStickInput];

type ParseResult<'a, T> = Result<(T, InputBuffer<'a>), ()>;

fn parse_button(input: InputBuffer, wanted: Button) -> ParseResult<Button> {
    if let Some(FightStickInput::Button(button)) = input.last() {
        if *button == wanted {
            return Ok((wanted, &input[0..input.len()-1]))
        }
    }
    Err(())
}

#[test]
fn button_press_is_parsed() {
    let buffer = [
        FightStickInput::StickPosition(Stick(Positive, Neutral)), 
        FightStickInput::Button(Button(ButtonType::A, true)),
    ];
    let result = parse_button(&buffer, Button(ButtonType::A, true));
    match result {
        Ok((Button(ButtonType::A, true), &[FightStickInput::StickPosition(Stick(Positive, Neutral))])) => {},
        _ => panic!("parsing didn't work. {:?} was parsed instead of the A button press", result),
    }
}

#[test]
fn wrong_input_is_not_parsed_as_button_press() {
    let buffer = [
        FightStickInput::Button(Button(ButtonType::A, true)),
        FightStickInput::StickPosition(Stick(Positive, Neutral)), 
    ];
    let result = parse_button(&buffer, Button(ButtonType::A, true));
    match result {
        Ok(wrong) => panic!("{:?} was parsed even though parsing was supposed to fail", wrong),
        Err(()) => {},
    }
}

fn parse_direction(input: InputBuffer, wanted: Stick) -> ParseResult<Stick> {
    if let Some(FightStickInput::StickPosition(stick)) = input.last() {
        if *stick == wanted {
            return Ok((wanted, &input[0..input.len()-1]))
        }
    }
    Err(())
}

#[test]
fn right_stick_input_is_parsed() {
    let buffer = [
        FightStickInput::Button(Button(ButtonType::A, true)),
        FightStickInput::StickPosition(Stick(Positive, Neutral)), 
    ];
    let result = parse_direction(&buffer, Stick(Positive, Neutral));
    match result {
        Ok((Stick(Positive, Neutral), &[FightStickInput::Button(Button(ButtonType::A, true))])) => {},
        _ => panic!("parsing didn't work expect a stick input towards the right but {:?} was the result instead.", result),
    }
}

#[test]
fn wrong_input_is_not_parsed_as_stick_input() {
    let buffer = [
        FightStickInput::StickPosition(Stick(Positive, Neutral)), 
        FightStickInput::Button(Button(ButtonType::A, true)),
    ];
    let result = parse_direction(&buffer, Stick(Positive, Neutral));
    match result {
        Ok(wrong) => panic!("{:?} was parsed even though parsing was supposed to fail", wrong),
        _ => {},
    }
}

macro_rules! sequence {
    () => {
        
    };
}

macro_rules! choice {
    () => {
        
    };
}

enum Command {
    QuaterCircle(ButtonType, Facing),
    DP(ButtonType, Facing),
    HalfCircle(ButtonType, Facing),
    Direction(AxisPosition, AxisPosition),
    Button(Button),
}

enum Facing {
    Left, Right,
}