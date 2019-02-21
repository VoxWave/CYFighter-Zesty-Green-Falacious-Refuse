use amethyst::ecs::{DenseVecStorage, Component};

#[derive(Component)]
pub struct Button(ButtonType, bool);

pub enum ButtonType {
    A, B, C, D, E,
}

#[derive(Component)]
pub struct Stick(StickState);

pub enum StickState {
    Neutral, Up, Down, Left, Right, Ul, Ur, Dl, Dr,
}