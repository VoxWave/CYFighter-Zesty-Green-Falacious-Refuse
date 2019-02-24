use amethyst::ecs::{DenseVecStorage, Component};

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
    Neutral, Up, Down, Left, Right, Ul, Ur, Dl, Dr,
}

#[derive(PartialEq, Debug)]
pub enum FightStickInput {
    Button(Button),
    StickPosition(Stick)
}