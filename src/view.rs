use std::time::{Duration, Instant};
use std::cmp::{min, max};

use complex::{c64};

use sdl2::rect::{Rect};
use sdl2::render::{Texture};
use render::{Tube};

pub struct View {
	pub pos: c64,
	pub mag: c64,
	
	pub part: (u32, u32),
	pub block: u32
}

impl View {
	pub fn new(pos: c64, mag: c64) -> Self {
		View { pos: pos, mag: mag, part: (0, 0), block: 16 }
	}

	pub fn put(&mut self, pos: c64) {
		self.pos = pos;
		self.part = (0, 0);
	}

	pub fn zoom(&mut self, mag: c64) {
		self.mag = mag;
		self.part = (0, 0);
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

	pub fn draw(&mut self, tube: &Tube, texture: &mut Texture, timeout: Duration) -> bool {
		let time = Instant::now();
		while !self.draw_block(tube, texture) {
			if time.elapsed() > timeout {
				return false;
			}
		}
		true
	}

	pub fn draw_block(&mut self, tube: &Tube, texture: &mut Texture) -> bool {
		let query = texture.query();
		let width = query.width;
		let height = query.height;

		let (x, y) = (self.part.0*self.block, self.part.1*self.block);
		let (w, h) = (min(width - x, self.block), min(height - y, self.block));

		let rect = Rect::new(x as i32, y as i32, w, h);
		texture.with_lock(Some(rect), |pixels: &mut [u8], pitch: usize| {
			for iy in 0..h {
				for ix in 0..w {
					let (r, p) = tube.rad_pos(self.pix_pos(x + ix, y + iy, width, height));
					let t = tube.get(r, p);

					let offset = pitch*(iy as usize) + 4*(ix as usize);

					let p = 16;
					let mut n = t % (2*p);
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

		self.part.0 += 1;
		if self.part.0*self.block >= width {
			self.part.0 = 0;
			self.part.1 += 1;
		}
		self.part.1*self.block >= height
	}
}
