extern crate sdl2;

use std::vec::{Vec};
use std::thread;
use std::sync::{Arc, Mutex};
use std::ops::{Deref, DerefMut};
use std::time::{Duration};
use std::f64::consts::PI;

use sdl2::event::{Event};
use sdl2::rect::{Rect};
use sdl2::keyboard::{Keycode};
use sdl2::render::{Texture, TextureAccess};
use sdl2::pixels::{PixelFormatEnum};

mod complex;
mod canvas;

use complex::{c64};
use canvas::*;

struct Scene {
	pos: c64,
	zoom: c64
}

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

fn ttos_rel(scene: &Scene, x: u32, y: u32, w: u32, h: u32) -> c64 {
	scene.zoom*c64::new(
		(2.0*x as f64 - w as f64 + 1.0)/h as f64,
		(2.0*y as f64 - h as f64 + 1.0)/h as f64
	)
}

fn ttos(scene: &Scene, x: u32, y: u32, w: u32, h: u32) -> c64 {
	scene.pos + ttos_rel(scene, x, y, w, h)
}

fn stor(scene: &Scene, dir: c64) -> (i32, usize) {
	let d = -dir.abs().log(RING_STEP).round() as i32;
	let a = dir/scene.zoom;
	let mut p = a.im.atan2(a.re);
	p += if p < 0.0 {2.0*PI} else {0.0};
	p *= (RING_SIZE as f64)/(2.0*PI);
	(d, p.floor() as usize)
}

fn render(scene: &Scene, canvas: &mut Canvas) {
	canvas.rings.clear();
	for d in 0..100 {
		let mut ring = Ring::new(d);
		let mut rs = scene.zoom*c64::from(RING_START*RING_STEP.powi(-(d as i32)));
		let ra = 2.0*PI/RING_SIZE as f64;
		for p in 0..RING_SIZE {
			let pos = scene.pos + rs*c64::new((ra*p as f64).cos(), (ra*p as f64).sin());
			ring.points[p].depth = trace(pos, 36);
		}
		canvas.rings.push(ring);
	}
}

fn draw(scene: &Scene, canvas: &Canvas, texture: &mut Texture) {
	let query = texture.query();
	let width = query.width;
	let height = query.height;
	texture.with_lock(None, |pixels: &mut [u8], pitch: usize| {
		for y in 0..height {
			for x in 0..width {
				let (mut d, mut p) = stor(scene, ttos_rel(scene, x, y, width, height));
				if d < 0 { d = 0; }
				if d >= canvas.rings.len() as i32 { d = canvas.rings.len() as i32 - 1; }

				let t = canvas.rings[d as usize].points[p].depth % 8;
				let offset = pitch*(y as usize) + 4*(x as usize);
				pixels[offset + 0] = 255*((t>>2) & 1) as u8;
				pixels[offset + 1] = 255*((t>>1) & 1) as u8;
				pixels[offset + 2] = 255*((t>>0) & 1) as u8;
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
	
	let redraw = Arc::new(Mutex::new(true));
	let redraw_ref = redraw.clone();

	let mut scene = Arc::new(Mutex::new(Scene { pos: c64::from(0.0), zoom: c64::from(2.0) }));
	let scene_ref = scene.clone();

	let canvas = Arc::new(Mutex::new(Canvas::new(0)));
	let mut canvas_ref = canvas.clone();

	let mut done = Arc::new(Mutex::new(false));
	let mut done_ref = done.clone(); 
	let mut thread_handle = thread::spawn(move || {
		while !done_ref.lock().unwrap().deref() {
			render(scene_ref.lock().unwrap().deref(), canvas_ref.lock().unwrap().deref_mut());
			*redraw_ref.lock().unwrap().deref_mut() = true;
			thread::park();
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
						scene.lock().unwrap().deref_mut().zoom *= c64::from((1.2 as f64).powi(y));
						thread_handle.thread().unpark();
					}
				},
				_ => continue,
			}
		}

		if *redraw.lock().unwrap().deref() {
			draw(scene.lock().unwrap().deref(), canvas.lock().unwrap().deref(), &mut texture);
			renderer.copy(&texture, None, Some(screen_rect)).unwrap();
			renderer.present();
			*redraw.lock().unwrap().deref_mut() = false;
		}

		thread::sleep(Duration::from_millis(40));
	}

	*done.lock().unwrap().deref_mut() = true;
	thread_handle.thread().unpark();
	thread_handle.join().unwrap();
}
