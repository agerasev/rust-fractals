use complex::{c64};

use sdl2::render::{Texture};
use render::{Tube};

pub struct View {
	pub pos: c64,
	pub zoom: c64
}

impl View {
	pub fn new(pos: c64, zoom: c64) -> Self {
		View { pos: pos, zoom: zoom }
	}

	pub fn pix_dev(&self, x: i32, y: i32, w: u32, h: u32) -> c64 {
		self.zoom*c64::new(
			(2.0*x as f64)/h as f64,
			(2.0*y as f64)/h as f64
		)
	}

	pub fn pix_pos(&self, x: u32, y: u32, w: u32, h: u32) -> c64 {
		self.pos + self.zoom*c64::new(
			(2.0*x as f64 - w as f64 + 1.0)/h as f64,
			(2.0*y as f64 - h as f64 + 1.0)/h as f64
		)
	}

	pub fn draw(&self, tube: &Tube, texture: &mut Texture) {
		if tube.rings.len() > 0 {
			let query = texture.query();
			let width = query.width;
			let height = query.height;
			texture.with_lock(None, |pixels: &mut [u8], pitch: usize| {
				for y in 0..height {
					for x in 0..width {
						let (r, p) = tube.rad_pos(self.pix_pos(x, y, width, height));
						let t = tube.rings[r].points[p].depth % 8;
						let offset = pitch*(y as usize) + 4*(x as usize);
						pixels[offset + 0] = 255*((t>>2) & 1) as u8;
						pixels[offset + 1] = 255*((t>>1) & 1) as u8;
						pixels[offset + 2] = 255*((t>>0) & 1) as u8;
						pixels[offset + 3] = 255;
					}
				}
			}).unwrap();
		}
	}
}
