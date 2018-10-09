extern crate nalgebra as na;
extern crate piston_window;
extern crate specs;
#[macro_use]
extern crate specs_derive;

use physics::{PhysicsSystem, Position, Velocity};
use piston_window::*;
use specs::prelude::*;
use specs::{Builder, DispatcherBuilder, World};

type Vector2 = na::Vector2<f64>;

mod physics;

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(PhysicsSystem, "physics_system", &[])
        .build();

    let p1 = world
        .create_entity()
        .with(Position(Vector2::new(0.0, 0.0)))
        .with(Velocity(Vector2::new(0.5, 0.0)))
        .build();

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        dispatcher.dispatch(&mut world.res);
        world.maintain();
        world.exec(|(positions,): (ReadStorage<Position>,)| {
            (&positions).join().for_each(|pos| {
                window.draw_2d(&event, |context, graphics| {
                    clear([1.0; 4], graphics);
                    rectangle(
                        [1.0, 0.0, 0.0, 1.0], // red
                        [pos.0.x, pos.0.y, 100.0, 100.0],
                        context.transform,
                        graphics,
                    );
                });
            });
        });
    }
}
