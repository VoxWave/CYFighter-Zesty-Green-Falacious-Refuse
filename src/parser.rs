use crate::fight_stick::{Button, ButtonType, Stick, AxisPosition::{self, *}, FightStickInput};

type InputBuffer<'a> = &'a[FightStickInput];

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

// use std::{
//     slice,
//     iter::Enumerate,
//     ops::{Range, RangeFrom, RangeTo, RangeFull},
// };

// use nom::{AtEof, Compare, CompareResult, Slice, InputIter, named, named_args, one_of, tag};

// use crate::fight_stick::{Button, ButtonType, Stick, AxisPosition::{self, *}, FightStickInput};

// #[derive(Clone, PartialEq)]
// struct InputBuffer<'a>(&'a[FightStickInput]);

// impl<'a> Slice<Range<usize>> for InputBuffer<'a> {
//     fn slice(&self, range: Range<usize>) -> Self {
//         InputBuffer(&self.0[range])
//     }
// }

// impl<'a> Slice<RangeTo<usize>> for InputBuffer<'a> {
//     fn slice(&self, range: RangeTo<usize>) -> Self {
//         self.slice(0..range.end)
//     }
// }

// impl<'a> Slice<RangeFrom<usize>> for InputBuffer<'a> {
//     fn slice(&self, range: RangeFrom<usize>) -> Self {
//         self.slice(range.start..self.0.len())
//     }
// }

// impl<'a> Slice<RangeFull> for InputBuffer<'a> {
//     fn slice(&self, range: RangeFull) -> Self {
//         self.clone()
//     }
// }

// impl<'a> InputIter for InputBuffer<'a> {
//     type Item = &'a FightStickInput;
//     type RawItem = FightStickInput;
//     type Iter = Enumerate<slice::Iter<'a, FightStickInput>>;
//     type IterElem = slice::Iter<'a, FightStickInput>;

//     #[inline]
//     fn iter_indices(&self) -> Self::Iter {
//         self.0.iter().enumerate()
//     }
//     #[inline]
//     fn iter_elements(&self) -> Self::IterElem {
//         self.0.iter()
//     }
//     #[inline]
//     fn position<P>(&self, predicate: P) -> Option<usize>
//     where
//         P: Fn(Self::RawItem) -> bool,
//     {
//         self.0.iter().position(|b| predicate(b.clone()))
//     }

//     #[inline]
//     fn slice_index(&self, count: usize) -> Option<usize> {
//         if count <= self.0.len() {
//             Some(count)
//         } else {
//             None
//         }
//     }
// }

// impl<'a> AtEof for InputBuffer<'a> {
//     fn at_eof(&self) -> bool {
//         true
//     }
// }

// impl<'a> Compare<InputBuffer<'a>> for InputBuffer<'a> {
//     fn compare(&self, t: InputBuffer<'a>) -> CompareResult {
//         if self == &t {
//             CompareResult::Ok
//         } else {
//             CompareResult::Error
//         }
//     }

//     fn compare_no_case(&self, t: InputBuffer<'a>) -> CompareResult {
//         self.compare(t)
//     }
// }

// fn convert_fsi_to_command(fsi: FightStickInput) -> Command {
//     match fsi {
//         FightStickInput::Button(b) => Command::Button(b),
//         FightStickInput::StickPosition(s) => Command::Direction(s.0, s.1),
//     }
// }

// fn stick_to_fsi(stick: Stick) -> FightStickInput {
//     FightStickInput::StickPosition(stick)
// }

// named_args!(parse_fsi(fsi: FightStickInput)<InputBuffer, FightStickInput>,
//     tag!(InputBuffer(&[fsi,]))
// );

// named!(parse_directional<InputBuffer, Command>,
//     one_of!(&[
//         stick_to_fsi(Stick(Positive, Positive)), 
//         stick_to_fsi(Stick(Positive, Neutral)),
//         stick_to_fsi(Stick(Positive, Negative)),
//         stick_to_fsi(Stick(Neutral, Positive)),
//         stick_to_fsi(Stick(Neutral, Neutral)),
//         stick_to_fsi(Stick(Neutral, Negative)),
//         stick_to_fsi(Stick(Negative, Positive)),
//         stick_to_fsi(Stick(Negative, Neutral)),
//         stick_to_fsi(Stick(Negative, Negative)),
//     ][..])
// );

// enum Command {
//     QuaterCircle(ButtonType, Facing),
//     DP(ButtonType, Facing),
//     HalfCircle(ButtonType, Facing),
//     Direction(AxisPosition, AxisPosition),
//     Button(Button),
// }

// enum Facing {
//     Left, Right,
// }