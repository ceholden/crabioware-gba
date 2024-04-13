use agb::fixnum::FixedWidthUnsignedInteger;
use agb::fixnum::Number as AGBNumber;
pub use agb::fixnum::Rect;
pub use agb::fixnum::Vector2D;
use agb::fixnum::{FixedNum, Num};
use alloc::vec::Vec;

pub type Number = agb::fixnum::FixedNum<8>;

// ========================================================================== //
// Vector2D
// ========================================================================== //
pub trait VecMath<N> {
    fn dot(&self, other: Self) -> N;
}
impl<N> VecMath<N> for Vector2D<N>
where
    N: AGBNumber,
{
    fn dot(&self, other: Self) -> N {
        self.x * other.x + self.y * other.y
    }
}

// FIXME: move to AABB { center: ..., half_width: ...}

// ========================================================================== //
// Rect
// ========================================================================== //
pub trait RectMath<T>
where
    T: AGBNumber,
{
    fn translate(&self, offset: Vector2D<T>) -> Self;
    fn centroid(&self) -> Vector2D<T>;
    fn dot(&self, other: Self) -> T;
}

impl<const N: usize> RectMath<FixedNum<N>> for Rect<FixedNum<N>> {
    fn translate(&self, offset: Vector2D<FixedNum<N>>) -> Self {
        Rect {
            position: self.position + offset,
            size: self.size,
        }
    }

    fn centroid(&self) -> Vector2D<FixedNum<N>> {
        Vector2D {
            x: self.position.x + self.size.x / 2,
            y: self.position.y + self.size.y / 2,
        }
    }

    fn dot(&self, other: Self) -> Num<i32, N> {
        self.centroid().dot(other.centroid())
    }
}

// ========================================================================== //
// Polygon
// ========================================================================== //
pub struct Polygon<N>
where
    N: FixedWidthUnsignedInteger,
{
    pub vertices: Vec<Vector2D<N>>,
}

impl<N> IntoIterator for Polygon<N>
where
    N: FixedWidthUnsignedInteger,
{
    type Item = Vector2D<N>;
    type IntoIter = PolygonIntoIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        let n = self.vertices.len();
        PolygonIntoIter {
            polygon: self,
            index: 0,
            size: n,
        }
    }
}

pub struct PolygonIntoIter<N>
where
    N: FixedWidthUnsignedInteger,
{
    polygon: Polygon<N>,
    index: usize,
    size: usize,
}
impl<N> Iterator for PolygonIntoIter<N>
where
    N: FixedWidthUnsignedInteger,
{
    type Item = Vector2D<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = match self.index < self.size {
            true => Some(self.polygon.vertices[self.index]),
            false => match self.index == self.size {
                true => Some(self.polygon.vertices[0]),
                false => None,
            },
        };
        self.index += 1;
        out
    }
}
