use num_traits::Float;
use std::convert::From;
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
#[derive(Debug, Copy, Clone)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2D { x, y }
    }

    pub fn zero() -> Self
    where
        T: Default,
    {
        Vector2D {
            x: T::default(),
            y: T::default(),
        }
    }
}
impl<T: Div<Output = T> + Copy + Into<f32>> Vector2D<T> {
    pub fn length(&self) -> f32 {
        let length_squared = self.x.into().powi(2) + self.y.into().powi(2);
        length_squared.sqrt()
    }

    pub fn normalize(&self) -> Vector2D<f32> {
        let length = self.length();
        Vector2D {
            x: self.x.into() / length,
            y: self.y.into() / length,
        }
    }
}
impl<T: fmt::Display> fmt::Display for Vector2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/*impl<T, U> From<Vector2D<T>> for Vector2D<U>
where
    T: Into<U>,
{
    fn from(vec: Vector2D<T>) -> Vector2D<U> {
        Vector2D::new(vec.x.into(), vec.y.into())
    }
}*/

impl<T> Add for Vector2D<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector2D<T>;

    fn add(self, other: Vector2D<T>) -> Vector2D<T> {
        Vector2D::new(self.x + other.x, self.y + other.y)
    }
}
impl<T: AddAssign> AddAssign for Vector2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Vector2D<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Vector2D<T>;

    fn sub(self, other: Vector2D<T>) -> Vector2D<T> {
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

impl<T> Mul<T> for Vector2D<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vector2D<T>;

    fn mul(self, scalar: T) -> Vector2D<T> {
        Vector2D::new(self.x * scalar, self.y * scalar)
    }
}
impl<T: Neg<Output = T>> Neg for Vector2D<T> {
    type Output = Vector2D<T>;

    fn neg(self) -> Self::Output {
        Vector2D::new(-self.x, -self.y)
    }
}

impl<T> Div<T> for Vector2D<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vector2D<T>;

    fn div(self, scalar: T) -> Vector2D<T> {
        Vector2D::new(self.x / scalar, self.y / scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let vec1 = Vector2D::new(3.0, 4.0);
        assert_eq!(vec1.length(), 5.0);

        let vec2 = Vector2D::new(0.0, 0.0);
        assert_eq!(vec2.length(), 0.0);
    }

    #[test]
    fn test_addition() {
        let vec1 = Vector2D::new(1, 2);
        let vec2 = Vector2D::new(3, 4);
        let result = vec1 + vec2;
        assert_eq!(result.x, 4);
        assert_eq!(result.y, 6);
    }

    #[test]
    fn test_subtraction() {
        let vec1 = Vector2D::new(5, 7);
        let vec2 = Vector2D::new(2, 3);
        let result = vec1 - vec2;
        assert_eq!(result.x, 3);
        assert_eq!(result.y, 4);
    }

    #[test]
    fn test_multiplication() {
        let vec = Vector2D::new(2, 3);
        let result = vec * 3;
        assert_eq!(result.x, 6);
        assert_eq!(result.y, 9);
    }
}
