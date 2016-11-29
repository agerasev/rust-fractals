extern crate num;

use std::ops::{Add, Sub, Mul, Div, Neg};
use std::convert::From;

#[derive(Copy, Clone)]
pub struct Complex<T> {
	pub re: T,
	pub im: T,
}

impl<T> Complex<T> where T: num::Float {
	pub fn conj(self) -> Self {
		Complex::<T> { re: self.re, im: -self.im }
	}

	pub fn abs2(self) -> T {
		self.re*self.re + self.im*self.im
	}

	#[allow(dead_code)]
	pub fn abs(self) -> T {
		self.abs2().sqrt()
	} 
}

impl<T> From<T> for Complex<T> where T: num::Float + num::Zero {
	fn from(v: T) -> Self {
		Complex::<T> { re: v, im: T::zero() }
	}
}

impl<T> From<(T,T)> for Complex<T> where T: num::Float {
	fn from(t: (T,T)) -> Self {
		Complex::<T> { re: t.0, im: t.1 }
	}
}

impl<T> From<[T;2]> for Complex<T> where T: num::Float {
	fn from(a: [T;2]) -> Self {
		Complex::<T> { re: a[0], im: a[1] }
	}
}

impl<T> Add<Complex<T>> for Complex<T> where T: num::Float {
	type Output = Self;
	fn add(self, other: Complex<T>) -> Self::Output {
		Complex::<T> { re: self.re + other.re, im: self.im + other.im }
	}
}

impl<T> Sub<Complex<T>> for Complex<T> where T: num::Float {
	type Output = Self;
	fn sub(self, other: Complex<T>) -> Self::Output {
		Complex::<T> { re: self.re - other.re, im: self.im - other.im }
	}
}

impl<T> Mul<T> for Complex<T> where T: num::Float {
	type Output = Self;
	fn mul(self, v: T) -> Self::Output {
		Complex::<T> { re: self.re*v, im: self.re*v }
	}
}

impl<T> Mul<Complex<T>> for Complex<T> where T: num::Float {
	type Output = Self;
	fn mul(self, other: Complex<T>) -> Self::Output {
		Complex::<T> {
			re: self.re*other.re - self.im*other.im, 
			im: self.re*other.im + self.im*other.re
		}
	}
}

impl<T> Div<T> for Complex<T> where T: num::Float {
	type Output = Self;
	fn div(self, v: T) -> Self::Output {
		Complex::<T> { re: self.re/v, im: self.im/v }
	}
}

impl<T> Div<Complex<T>> for Complex<T> where T: num::Float {
	type Output = Self;
	fn div(self, other: Complex<T>) -> Self::Output {
		(self*other.conj())/other.abs2()
	}
}

impl<T> Neg for Complex<T> where T: num::Float + num::Signed {
	type Output = Complex<T>;
	fn neg(self) -> Self::Output {
		Complex::<T> { re: -self.re, im: -self.im }
	}
}

#[allow(non_camel_case_types)]
pub type c32 = Complex<f32>;
#[allow(non_camel_case_types)]
pub type c64 = Complex<f64>;

