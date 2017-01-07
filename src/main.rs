extern crate sdl2;

use std::thread;
use std::sync::{Arc, Mutex};
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};

use sdl2::event::{Event};
use sdl2::rect::{Rect};
use sdl2::keyboard::{Keycode};
use sdl2::mouse::{MouseButton};
use sdl2::render::{TextureAccess};
use sdl2::pixels::{PixelFormatEnum};

mod complex;
mod view;
mod render;

use complex::{c64};
use view::{View};
use render::{Tube, Status};

struct Shared {
	redraw: bool,
	done: bool
}

impl Shared {
	fn new() -> Self {
		Shared { redraw: true, done: false }
	}
}

struct Control {
	lmb: bool,
	x: i32,
	y: i32,
	dx: i32,
	dy: i32
}

impl Control {
	fn new() -> Self {
		Control { 
			lmb: false,
			x: 0, y: 0,
			dx: 0, dy: 0
		}
	}
}

fn main() {
	let ctx = sdl2::init().unwrap();
	let video_ctx = ctx.video().unwrap();
	
	let width = 800;
	let height = 600;
	let screen_rect = Rect::new(0, 0, width, height);
	let window = video_ctx.window("Rust Fractals", width, height).position_centered().build().unwrap();

	let mut renderer = window.renderer().build().unwrap();

	let mut texture = renderer.create_texture(PixelFormatEnum::ARGB8888, TextureAccess::Streaming, width, height).unwrap();
	
	let shared = Arc::new(Mutex::new(Shared::new()));
	let shared_ref = shared.clone();

	let mut view = View::new(c64::from(0.0), c64::from(2.0));

	let tube = Arc::new(Mutex::new(Tube::new(2.01, 1024, 1.0 + 1e-2, 32)));
	let tube_ref = tube.clone();
	
	tube.lock().unwrap().deref_mut().put(c64::new(0.0, 0.0), 0);

	let rth = thread::spawn(move || {
		while !shared_ref.lock().unwrap().deref().done {
			let status = tube_ref.lock().unwrap().deref_mut().render(1000, Duration::from_millis(40));
			match status {
				Status::Timeout | Status::Done => shared_ref.lock().unwrap().deref_mut().redraw = true,
				_ => {}
			}
			match status {
				Status::Done | Status::Idle => thread::park(),
				_ => {}
			}
		}
	});

	let mut blit = false;
	let mut control = Control::new();
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
						view.zoom *= c64::from((1.2 as f64).powi(-y));
						shared.lock().unwrap().deref_mut().redraw = true;
						rth.thread().unpark();
					}
				},
				Event::MouseButtonDown{mouse_btn, x, y, ..} => {
					match mouse_btn {
						MouseButton::Left => {
							control.lmb = true;
							control.x = x;
							control.y = y;
						},
						_ => {}
					}
				},
				Event::MouseButtonUp{mouse_btn, ..} => {
					match mouse_btn {
						MouseButton::Left => {
							let pos = view.pos - view.pix_dev(control.dx, control.dy, width, height);
							view.pos = pos;
							tube.lock().unwrap().deref_mut().put(pos, 0);

							rth.thread().unpark();
							shared.lock().unwrap().deref_mut().redraw = true;

							control.lmb = false;
							control.dx = 0;
							control.dy = 0;
						},
						_ => {}
					}
				},
				Event::MouseMotion{x, y, ..} => {
					if control.lmb {
						control.dx = x - control.x;
						control.dy = y - control.y;
						blit = true;
					}
				}
				_ => continue,
			}
		}

		if shared.lock().unwrap().deref().redraw {
			view.draw(tube.lock().unwrap().deref(), &mut texture);
			shared.lock().unwrap().deref_mut().redraw = false;
			blit = true;
		}

		if blit {
			renderer.clear();
			renderer.copy(&texture, None, Some(Rect::new(control.dx, control.dy, width, height))).unwrap();
			renderer.present();
			blit = false;
		}

		thread::sleep(Duration::from_millis(40));
	}

	shared.lock().unwrap().deref_mut().done = true;
	rth.thread().unpark();
	rth.join().unwrap();
}
