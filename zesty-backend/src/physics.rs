use specs::prelude::*;
use na::Vector2;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub pos: Vector2<f32>,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub dir: Vector2<f32>,
}

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (mut positions, mut velocities):Self::SystemData) {
        (&mut positions, &mut velocities).join().for_each( |(position,velocity)| { 
            position.pos += velocity.dir;
         });
    }
}