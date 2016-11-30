extern crate sdl2;

use std::thread;
use std::time::{Duration};

use sdl2::event::{Event};
use sdl2::rect::{Rect};
use sdl2::keyboard::{Keycode};
use sdl2::render::{Texture, TextureAccess};
use sdl2::pixels::{PixelFormatEnum};

mod complex;
use complex::{c64};

struct Scene {
	pos: c64,
	zoom: c64
}

fn trace(c: c64, n: u32) -> u8 {
	let mut k: u32 = 0;
	let mut z = c64::new(0.0, 0.0);
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

fn ttos_rel(scene: &Scene, x: u32, y: u32, w: u32, h: u32) -> c64 {
	scene.zoom*c64::new(
		(2.0*x as f64 - w as f64 + 1.0)/h as f64,
		(2.0*y as f64 - h as f64 + 1.0)/h as f64
	)
}

fn ttos(scene: &Scene, x: u32, y: u32, w: u32, h: u32) -> c64 {
	scene.pos + ttos_rel(scene, x, y, w, h)
}

fn render(scene: &Scene, texture: &mut Texture) {
	let query = texture.query();
	let width = query.width;
	let height = query.height;
	texture.with_lock(None, |pixels: &mut [u8], pitch: usize| {
		for y in 0..height {
			for x in 0..width {
				let t = trace(ttos(&scene, x, y, width, height), 36);

				let offset = pitch*(y as usize) + 4*(x as usize);
				pixels[offset + 0] = t;
				pixels[offset + 1] = t;
				pixels[offset + 2] = t;
				pixels[offset + 3] = 255;
			}
		}
	}).unwrap();
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

	let mut scene = Scene { pos: c64::from(0.0), zoom: c64::from(2.0) };
	let mut redraw = true;

	let mut events = ctx.event_pump().unwrap();
	'main : loop {
		for event in events.poll_iter() {
			match event {
				Event::Quit{..} => break 'main,
				Event::KeyDown{keycode, ..} => 
					if keycode.unwrap() == Keycode::Escape { break 'main; },
				Event::MouseWheel{y, ..} => {
					if y != 0 {
						// let s = events.mouse_state();
						scene.zoom *= c64::from((1.2 as f64).powi(-y));
						redraw = true;
					}
				},
				_ => continue,
			}
		}

		if redraw {
			render(&scene, &mut texture);
			renderer.copy(&texture, None, Some(screen_rect)).unwrap();
			renderer.present();
			redraw = false;
		}

		thread::sleep(Duration::from_millis(40));
	}
}
