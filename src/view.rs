use complex::{c64};
use std::time::{Duration, Instant};

use sdl2::render::{Texture};
use render::{Tube};

pub struct View {
	pub pos: c64,
	pub mag: c64,
	
	pub det: i32,
	pub part: i32
}

impl View {
	pub fn new(pos: c64, mag: c64) -> Self {
		View { pos: pos, mag: mag, det: 0, part: 0 }
	}

	pub fn put(&mut self, pos: c64) {
		self.pos = pos;
	}

	pub fn zoom(&mut self, mag: c64) {
		self.mag = mag;
	}

	pub fn pix_dev(&self, x: i32, y: i32, w: u32, h: u32) -> c64 {
		self.mag*c64::new(
			(2.0*x as f64)/h as f64,
			(2.0*y as f64)/h as f64
		)
	}

	pub fn pix_pos(&self, x: u32, y: u32, w: u32, h: u32) -> c64 {
		self.pos + self.mag*c64::new(
			(2.0*x as f64 - w as f64 + 1.0)/h as f64,
			(2.0*y as f64 - h as f64 + 1.0)/h as f64
		)
	}

	pub fn draw(&self, tube: &Tube, texture: &mut Texture, ) {
		if tube.rings.len() > 0 {
			let query = texture.query();
			let width = query.width;
			let height = query.height;
			texture.with_lock(None, |pixels: &mut [u8], pitch: usize| {
				for y in 0..height {
					for x in 0..width {
						let (r, p) = tube.rad_pos(self.pix_pos(x, y, width, height));
						let t = tube.rings[r - tube.begin].points[p].depth;
						let offset = pitch*(y as usize) + 4*(x as usize);
						let p = 16;
						let mut n = (t % (2*p));
						if n > p {
							n = 2*p - n;
						}
						let val = (255.0*(n as f64/p as f64)) as u8;
						pixels[offset + 0] = val;
						pixels[offset + 1] = val;
						pixels[offset + 2] = val;
						pixels[offset + 3] = 255;
					}
				}
			}).unwrap();
		}
	}
}
