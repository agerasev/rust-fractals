use complex::{c64};
use std::collections::{LinkedList};

pub struct Point {
	pub depth: u64,
	pub frac: f64
}

impl Point {
	pub fn new() -> Self {
		Point { depth: 0, frac: 0.0 }
	}
}

impl Clone for Point {
	fn clone(&self) -> Self {
		Point { depth: self.depth, frac: self.frac }
	}
}

impl Copy for Point {}


pub static RING_SIZE: usize = 256;
pub static RING_STEP: f64 = 1.04;
pub static RING_START: f64 = 2.0;

pub struct Ring {
	pub depth: u64,
	pub points: Vec<Point>
}

impl Ring {
	pub fn new(depth: u64) -> Self {
		Ring { depth: depth, points: vec![Point::new(); RING_SIZE] }
	}
}

pub struct Canvas {
	pub start: u64,
	pub rings: Vec<Ring>
}

impl Canvas {
	pub fn new(start: u64) -> Self {
		Canvas { start: start, rings: Vec::<Ring>::new() }
	}
}