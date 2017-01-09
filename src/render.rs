use complex::{c64};
use std::f64::consts::PI;

use std::time::{Duration, Instant};


fn trace(c: c64, n: usize) -> usize {
	let mut z = c;
	for i in 1..n {
		z = z*z + c;
		if z.abs2() > 4.0 {
			return i;
		}
	}
	0
}

fn check_cardio(c: c64) -> bool {
	let re_4 = c.re - 0.25;
	let im2 = c.im*c.im;
	let p = (re_4*re_4 + im2).sqrt();
	let re1 = c.re + 1.0;
	return re_4 < p - 2.0*p*p || re1*re1 + im2 < 0.0625;
}

pub struct Point {
	pub depth: usize,
	pub frac: f64
}

impl Point {
	pub fn new() -> Self {
		Point { depth: 0, frac: 0.0 }
	}

	pub fn trace(&mut self, c: c64, n: usize) {
		self.depth = if check_cardio(c) {
			0
		} else {
			trace(c, n)
		};
	}
}

impl Clone for Point {
	fn clone(&self) -> Self {
		Point { depth: self.depth, frac: self.frac }
	}
}

impl Copy for Point {}

pub struct Ring {
	pub done: bool,
	pub points: Vec<Point>
}

impl Clone for Ring {
	fn clone(&self) -> Self {
		Ring { done: self.done, points: self.points.clone() }
	}
}

impl Ring {
	pub fn new() -> Self {
		Ring { done: false, points: Vec::new() }
	}

	pub fn render(&mut self, size: usize, pos: c64, zoom: c64, depth: usize) -> bool {
		self.points.resize(size, Point::new());
		let a = 2.0*PI/(size as f64);
		let mut zero = true;
		for i in 0..size {
			let ra = a*(i as f64);
			let rot = c64::new(ra.cos(), ra.sin());

			let ptr = &mut self.points[i];
			ptr.trace(pos + zoom*rot, depth);
			if ptr.depth > 0 {
				zero = false;
			}
		}
		self.done = true;
		zero
	}
}

pub struct Tube {
	pub pos: c64,
	pub rad: f64,
	pub seg: usize,
	pub step: f64,
	pub depth: usize,
	pub rings: Vec<Ring>,
	pub zero: bool
}

pub enum Status {
	Done,
	Timeout,
	Idle
}

impl Tube {
	pub fn new(rad: f64, seg: usize, step: f64, depth: usize) -> Self {
		Tube {
			pos: c64::from(0.0),
			rad: rad,
			seg: seg,
			step: step,
			depth: depth,
			rings: Vec::new(),
			zero: false
		}
	}

	pub fn put(&mut self, pos: c64) {
		self.pos = pos;
		self.rings.clear();
		self.zero = false;
	}

	pub fn render(&mut self, begin: usize, end: usize, timeout: Duration) -> Status {
		if self.rings.len() < end {
			self.rings.resize(end, Ring::new());
		}

		let mut complete = true;
		for i in begin..end {
			if !self.rings[i].done { complete = false; break; }
		}
		if complete {
			return Status::Idle;
		}

		let time = Instant::now();
		for i in begin..end {
			if !self.rings[i].done {
				let zoom = c64::from(self.rad*self.step.powi(-(i as i32)));
				self.rings[i].render(self.seg, self.pos, zoom, self.depth);
				/*
				if zero {
					self.zero = zero;
					return Status::Done;
				}
				*/
			}
			if time.elapsed() > timeout {
				return Status::Timeout;
			}
		}
		Status::Done
	}

	pub fn get(&self, ir: usize, ip: usize) -> usize {
		if ip < self.seg && ir < self.rings.len() && self.rings[ir].done {
			self.rings[ir].points[ip].depth
		} else {
			0
		}
	}

	pub fn rad_pos(&self, pos: c64) -> (usize, usize) {
		let dev = pos - self.pos;

		let ir = (self.rad/dev.abs()).log(self.step).max(0.0).round() as usize;

		let mut p = dev.im.atan2(dev.re);
		p += if p < 0.0 {2.0*PI} else {0.0};
		p *= (self.seg as f64)/(2.0*PI);
		let ip = p.floor() as usize;
		
		(ir, ip)
	}
}