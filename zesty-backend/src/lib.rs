extern crate specs;
#[macro_use]
extern crate specs_derive;
extern crate nalgebra as na;

use specs::VecStorage;

pub mod physics;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}