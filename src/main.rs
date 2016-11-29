extern crate sdl2;

use sdl2::event::{Event};
use sdl2::rect::{Rect};
use sdl2::keyboard::{Keycode};
use sdl2::render::{TextureAccess};
use sdl2::pixels::{Color, PixelFormatEnum};

mod complex;
use complex::{c64};

fn trace(c: c64, n: u32) -> u8 {
	let mut k: u32 = 0;
	let mut z = c64::from((0.0, 0.0));
	for i in 0..n {
		z = z*z + c;
		if z.abs2() > 4.0 {
			k = i; 
			break;
		}
	}
	let f = (k as f64)/((n - 1) as f64);
	((255 as f64)*f) as u8
}

fn main() {
	let ctx = sdl2::init().unwrap();
	let video_ctx = ctx.video().unwrap();
	
	let width = 800;
	let height = 600;
	let screen_rect = Rect::new(0, 0, width, height);
	let window = video_ctx.window("SDL2", width, height).position_centered().opengl().build().unwrap();

	let mut renderer = window.renderer().build().unwrap();

	let mut texture = renderer.create_texture(PixelFormatEnum::ARGB8888, TextureAccess::Streaming, width, height).unwrap();

	texture.with_lock(Some(screen_rect), |pixels: &mut [u8], pitch: usize| {
		for y in 0..height {
			for x in 0..width {
				let pos = c64::from((
					(x as f64 - 0.5*width as f64)/(height - 1) as f64,
					(y as f64 - 0.5*height as f64)/(height - 1) as f64
				));
				let t = trace(c64::from(4.0)*pos, 36);

				let offset = pitch*(y as usize) + 4*(x as usize);
				pixels[offset + 0] = t;
				pixels[offset + 1] = t;
				pixels[offset + 2] = t;
				pixels[offset + 3] = 255;
			}
		}
	}).unwrap();

	let mut events = ctx.event_pump().unwrap();
	'main : loop {
		for event in events.poll_iter() {
			match event {
				Event::Quit{..} => break 'main,
				Event::KeyDown{keycode, ..} => 
					if keycode.unwrap() == Keycode::Escape { break 'main; },
				_ => continue,
			}
		}

		renderer.set_draw_color(Color::RGB(0, 0, 0));
		renderer.clear();

		renderer.copy(&texture, None, Some(screen_rect)).unwrap();

		renderer.present();
	}
}
