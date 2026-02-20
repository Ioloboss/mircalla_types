use std::collections::btree_map::Values;

use crate::units::Pixels;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
	Vertical,
	Horizontal,
}


// ===================================================
// POSITION
// ===================================================

#[derive(Debug, Clone, Copy)]
pub struct Position<T> {
	pub x: T,
	pub y: T,
}

impl <A, B> From<(A, A)> for Position<B>
where
	A: Into<B>
{
	fn from(value: (A, A)) -> Self {
		Self { x: value.0.into(), y: value.1.into() }
	}
}

impl <A, B> std::ops::Add<Position<B>> for Position<A>
where
	A: std::ops::Add<B>,
{
	type Output = Position<<A as std::ops::Add<B>>::Output>;
	fn add(self, rhs: Position<B>) -> Self::Output {
		Self::Output {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Axis {
	X,
	Y,
}

impl From<Dimension> for Axis {
	fn from(value: Dimension) -> Self {
		match value {
			Dimension::Width => Self::X,
			Dimension::Height => Self::Y,
		}
	}
}

impl <T> Position<T>
where
	T: Copy,
{
	pub fn get(&self, axis: Axis) -> T {
		match axis {
			Axis::X => self.x,
			Axis::Y => self.y,
		}
	}

	pub fn set(&mut self, axis: Axis, value: T) {
		match axis {
			Axis::X => self.x = value,
			Axis::Y => self.y = value,
		}
	}
}

impl <T> Position<Option<T>> {
	pub fn unwrap_contents(self) -> Position<T> {
		Position {
			x: self.x.unwrap(),
			y: self.y.unwrap(),
		}
	}

	pub fn none() -> Self {
		Self {x : None, y: None }
	}
}

impl From<Position<Pixels<u16>>> for Position<Pixels<f32>> {
	fn from(value: Position<Pixels<u16>>) -> Self {
		Position {
			x: value.x.into(),
			y: value.y.into(),
		}
	}
}






// ===================================================
// SIZE
// ===================================================

#[derive(Debug, Clone, Copy)]
pub struct Size<T> {
	pub width: T,
	pub height: T,
}

impl <A, B> From<(A, A)> for Size<B>
where
	A: Into<B>
{
	fn from(value: (A, A)) -> Self {
		Self { width: value.0.into(), height: value.1.into() }
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Dimension {
	Width,
	Height,
}

impl From<Direction> for Dimension {
	fn from(value: Direction) -> Self {
		match value {
			Direction::Horizontal => Dimension::Width,
			Direction::Vertical => Dimension::Height,
		}
	}
}

impl Dimension {
	pub fn opposite(&self) -> Self {
		match self {
			Dimension::Width => Dimension::Height,
			Dimension::Height => Dimension::Width,
		}
	}
}


impl <T> Size<T>
where
	T: Copy,
{
	pub fn get(&self, dimension: Dimension) -> T {
		match dimension {
			Dimension::Width => self.width,
			Dimension::Height => self.height,
		}
	}

	pub fn set(&mut self, dimension: Dimension, value: T) {
		match dimension {
			Dimension::Width => self.width = value,
			Dimension::Height => self.height = value,
		}
	}
}

impl <T> Size<Option<T>> {
	pub fn unwrap_contents(self) -> Size<T> {
		Size {
			width: self.width.unwrap(),
			height: self.height.unwrap(),
		}
	}

	pub fn none() -> Self {
		Size { width: None, height: None}
	}
}

impl From<Size<Pixels<u16>>> for Size<Pixels<f32>> {
	fn from(value: Size<Pixels<u16>>) -> Self {
		Size {
			width: value.width.into(),
			height: value.height.into(),
		}
	}
}



// ===================================================
// COLOUR
// ===================================================

fn to_linear_rgb(color_chanel: u8) -> f32 {
	let value = color_chanel as f32 / 255.0;
	if value > 0.04045 {
		((value + 0.055) / 1.055).powf(2.4)
	} else {
		value / 12.92
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Colour {
	pub r: f32,
	pub g: f32,
	pub b: f32,
}

impl Colour {
	pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
		Self {
			r: to_linear_rgb(r),
			g: to_linear_rgb(g),
			b: to_linear_rgb(b),
		}
	}
}

impl Colour {
	pub fn black() -> Self {
		Colour { r: 0.0, g: 0.0, b: 0.0 }
	}

	pub fn red() -> Self {
		Colour { r: 1.0, g: 0.0, b: 0.0 }
	}

	pub fn green() -> Self {
		Colour { r: 0.0, g: 1.0, b: 0.0 }
	}

	pub fn blue() -> Self {
		Colour { r: 0.0, g: 0.0, b: 1.0 }
	}
}

impl Into<[f32; 3]> for Colour {
	fn into(self) -> [f32; 3] {
		[
			self.r,
			self.g,
			self.b,
		]
	}
}