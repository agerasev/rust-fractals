extern crate sdl2;

use std::thread;
use std::sync::{Arc, Mutex};
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};

use sdl2::event::{Event};
use sdl2::rect::{Rect};
use sdl2::keyboard::{Keycode};
use sdl2::render::{TextureAccess};
use sdl2::pixels::{PixelFormatEnum};

mod complex;
mod camera;
mod render;

use complex::{c64};
use camera::{Camera};
use render::{Tube};

fn main() {
	let ctx = sdl2::init().unwrap();
	let video_ctx = ctx.video().unwrap();
	
	let width = 800;
	let height = 600;
	let screen_rect = Rect::new(0, 0, width, height);
	let window = video_ctx.window("SDL2", width, height).position_centered().opengl().build().unwrap();

	let mut renderer = window.renderer().build().unwrap();

	let mut texture = renderer.create_texture(PixelFormatEnum::ARGB8888, TextureAccess::Streaming, width, height).unwrap();
	
	let redraw = Arc::new(Mutex::new(true));
	let redraw_ref = redraw.clone();

	let mut camera = Arc::new(Mutex::new(Camera { pos: c64::from(0.0), zoom: c64::from(2.0) }));
	let camera_ref = camera.clone();

	let tube = Arc::new(Mutex::new(Tube::new(c64::new(0.0, 0.0), 2.01, 1024, 1.0 + 1e-2, 32, 0)));
	let mut tube_ref = tube.clone();

	let mut done = Arc::new(Mutex::new(false));
	let mut done_ref = done.clone(); 
	let mut thread_handle = thread::spawn(move || {
		let mut time = Instant::now();
		while !done_ref.lock().unwrap().deref() {
			tube_ref.lock().unwrap().deref_mut().render();
			if time.elapsed() > Duration::from_millis(200) {
				*redraw_ref.lock().unwrap().deref_mut() = true;
				thread::park();
				time = Instant::now();
			}
		}
	});

	let mut events = ctx.event_pump().unwrap();
	'main : loop {
		for event in events.poll_iter() {
			match event {
				Event::Quit{..} => break 'main,
				Event::KeyDown{keycode, ..} => 
					if keycode.unwrap() == Keycode::Escape { break 'main; },
				Event::MouseWheel{y, ..} => {
					if y != 0 {
						// let s = events.mouse_state().x();
						camera.lock().unwrap().deref_mut().zoom *= c64::from((1.2 as f64).powi(-y));
						*redraw.lock().unwrap().deref_mut() = true;
					}
				},
				_ => continue,
			}
		}

		if *redraw.lock().unwrap().deref() {
			camera.lock().unwrap().deref().draw(tube.lock().unwrap().deref(), &mut texture);
			renderer.copy(&texture, None, Some(screen_rect)).unwrap();
			renderer.present();
			*redraw.lock().unwrap().deref_mut() = false;
			thread_handle.thread().unpark();
		}

		thread::sleep(Duration::from_millis(40));
	}

	*done.lock().unwrap().deref_mut() = true;
	thread_handle.thread().unpark();
	thread_handle.join().unwrap();
}
