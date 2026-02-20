// ===================================================
// PIXELS
// ===================================================


#[derive(Debug, PartialEq, Copy, Clone, PartialOrd)]
pub struct Pixels<T> {
	pub value: T,
}

impl <T> Pixels<T>
where
	T: Into<f32> + Copy
{
	pub fn to_screen_space(&self, screen_dimension: Pixels<f32>) -> ScreenSpace {
		(((self.value.into() / screen_dimension.value) * 2.0) - 1.0).into()
	}

	pub fn to_screen_space_length(&self, screen_dimension: Pixels<f32>) -> ScreenSpace {
		((self.value.into() / screen_dimension.value) * 2.0).into()
	}
}

impl <T> From<T> for Pixels<T> {
	fn from(value: T) -> Self {
		Self { value }
	}
}

impl <A, B> std::ops::Add<Pixels<B>> for Pixels<A>
where
	A: std::ops::Add + Copy,
	B: Into<A> + Copy,
	<A as std::ops::Add>::Output: Copy,
{
	type Output = Pixels<<A as std::ops::Add>::Output>;

	fn add(self, rhs: Pixels<B>) -> Self::Output {
		Self::Output { value: self.value + rhs.value.into() }
	}
}

impl<A, B> std::ops::AddAssign<Pixels<B>> for Pixels<A>
where
	Pixels<A>: std::ops::Add<Pixels<B>, Output = Pixels<A>>,
	B: Copy,
	A: Copy,
{	
	fn add_assign(&mut self, rhs: Pixels<B>) {
		*self = *self + rhs;
	}
}

impl <A, B> std::ops::Sub<Pixels<B>> for Pixels<A>
where
	A: std::ops::Sub + Copy,
	B: Into<A> + Copy,
	<A as std::ops::Sub>::Output: Copy,
{
	type Output = Pixels<<A as std::ops::Sub>::Output>;
	fn sub(self, rhs: Pixels<B>) -> Self::Output {
		Self::Output { value: self.value - rhs.value.into() }
	}
}

impl std::ops::Mul<usize> for Pixels<u16> {
	type Output = Pixels<u16>;
	fn mul(self, rhs: usize) -> Self::Output {
		(self.value * rhs as u16).into()
	}
}

impl std::ops::Div<usize> for Pixels<u16> {
	type Output = Pixels<u16>;
	fn div(self, rhs: usize) -> Self::Output {
		(self.value / rhs as u16).into()
	}
}

impl From<Pixels<u16>> for Pixels<f32> {
	fn from(value: Pixels<u16>) -> Self {
		Pixels { value: value.value.into() }
	}
}


// ===================================================
// SCREEN SPACE
// ===================================================


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ScreenSpace {
	pub value: f32,
}

impl <T> From<T> for ScreenSpace
where
	T: Into<f32>
{
	fn from(value: T) -> Self {
		Self { value: value.into() }
	}
}