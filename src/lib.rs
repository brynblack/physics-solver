#![deny(missing_docs, unsafe_code)]

//! Module for a physics solver.

use std::ops::{AddAssign, SubAssign};

/// A three-dimensional vector.
#[derive(Clone, Copy, Debug)]
pub struct Vec3<T> {
    /// The x-coordinate.
    pub x: T,
    /// The y-coordinate.
    pub y: T,
    /// The z-coordinate.
    pub z: T,
}

impl<T> AddAssign for Vec3<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

/// An arbitrary point in the world.
#[derive(Clone, Copy, Debug)]
pub struct Point<T> {
    /// The position of the point.
    pub pos: Vec3<T>,
    /// The velocity of the point.
    pub vel: Vec3<T>,
}

/// A physics solver.
pub struct Solver<T> {
    gravity: T,
}

impl<T> Solver<T>
where
    T: AddAssign + Copy + SubAssign,
{
    /// Constructs a new solver with the given constraints.
    pub fn new(gravity: T) -> Self {
        Self { gravity }
    }

    /// Solves an iteration of the simulation using Euler's method.
    ///
    /// Takes in a collection of type `Point` and mutates them.
    pub fn solve(&self, points: &mut [Point<T>]) {
        points.iter_mut().for_each(|p| {
            p.vel.y -= self.gravity;
            p.pos += p.vel;
        });
    }
}
