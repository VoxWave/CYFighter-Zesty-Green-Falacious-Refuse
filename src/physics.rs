use Vector2;
use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Vector2);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity(pub Vector2);
pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (mut positions, mut velocities): Self::SystemData) {
        (&mut positions, &mut velocities)
            .join()
            .for_each(|(position, velocity)| {
                position.0 += velocity.0;
            });
    }
}
