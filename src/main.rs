use std::ops::Add;
use std::ops::Mul;

struct Complex {
	re: f64,
	im: f64,
}

impl Copy for Complex {}

impl Clone for Complex {
    fn clone(&self) -> Complex { *self }
}

impl Add for Complex {
	type Output = Complex;
	fn add(self, other: Complex) -> Complex {
		Complex { re: self.re + other.re, im: self.im + other.im }
	}
}

impl Mul for Complex {
	type Output = Complex;
	fn mul(self, other: Complex) -> Complex {
		Complex { re: self.re*other.re - self.im*other.im, im: self.re*other.im + self.im*other.re }
	}
}

fn abs2(var: Complex) -> f64 {
	var.re*var.re + var.im*var.im
}

fn main() {
	let xrange = 32;
	let yrange = 32;
	let depth = 32;
	for iy in (1 - yrange)..yrange {
		for ix in (1 - xrange)..xrange {
			let c = Complex { re: 2.0/(xrange as f64)*(ix as f64), im: 1.5/(yrange as f64)*(iy as f64) };
			let mut a = c;
			let mut d = -1;
			for i in 0 .. depth {
				a = a*a + c;
				if abs2(a) > 2.0 {
					d = i;
					break;
				}
			}
			if d < 0 {
				print!(" ");
			} else if (d as f64) < 0.5*(depth as f64) {
				print!(".");
			} else {
				print!("#");
			}
		}
		println!("");
	}
}
