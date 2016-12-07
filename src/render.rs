use complex::{c64};
use std::f64::consts::PI;

fn trace(c: c64, n: u64) -> u64 {
	let mut z = c;
	for i in 1..n {
		z = z*z + c;
		if z.abs2() > 4.0 {
			return i;
		}
	}
	0
}

pub struct Point {
	pub depth: u64,
	pub frac: f64
}

impl Point {
	pub fn new() -> Self {
		Point { depth: 0, frac: 0.0 }
	}

	pub fn trace(&mut self, c: c64, n: u64) {
		self.depth = trace(c, n);
	}
}

impl Clone for Point {
	fn clone(&self) -> Self {
		Point { depth: self.depth, frac: self.frac }
	}
}

impl Copy for Point {}

pub struct Ring {
	pub points: Vec<Point>
}

impl Ring {
	pub fn new() -> Self {
		Ring { points: Vec::new() }
	}

	pub fn render(&mut self, size: usize, pos: c64, zoom: c64, depth: u64) {
		self.points.resize(size, Point::new());
		let a = 2.0*PI/(size as f64);
		for i in 0..size {
			let ra = a*(i as f64);
			let rot = c64::new(ra.cos(), ra.sin());
			self.points[i].trace(pos + zoom*rot, depth);
		}
	}
}

pub struct Tube {
	pub pos: c64,
	pub rad: f64,
	pub seg: usize,
	pub step: f64,
	pub depth: u64,
	pub start: usize,
	pub rings: Vec<Ring>
}

impl Tube {
	pub fn new(pos: c64, rad: f64, seg: usize, step: f64, depth: u64, start: usize) -> Self {
		Tube {
			pos: pos,
			rad: rad,
			seg: seg,
			step: step,
			depth: depth,
			start: start, 
			rings: Vec::new(),
		}
	}

	pub fn render(&mut self) {
		let mut ring = Ring::new();
		let d = self.rings.len();
		let zoom = c64::from(self.rad*self.step.powi(-(d as i32)));
		ring.render(self.seg, self.pos, zoom, self.depth);
		self.rings.push(ring);
	}

	pub fn rad_pos(&self, pos: c64) -> (usize, usize) {
		let dev = pos - self.pos;

		let mut d = -(dev.abs()/self.rad).log(self.step).round() as i64;
		if d < 0 { d = 0; }
		if d >= self.rings.len() as i64 { d = self.rings.len() as i64 - 1; }
		let ir = d as usize;

		let mut p = dev.im.atan2(dev.re);
		p += if p < 0.0 {2.0*PI} else {0.0};
		p *= (self.rings[ir].points.len() as f64)/(2.0*PI);
		let ip = p.floor() as usize;
		
		(ir, ip)
	}
}