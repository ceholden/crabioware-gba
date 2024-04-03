use agb::fixnum::FixedWidthUnsignedInteger;
use agb::fixnum::Number as AGBNumber;
use agb::fixnum::Rect as AGBRect;
use agb::fixnum::Vector2D;
use agb::fixnum::{FixedNum, Num};
use alloc::vec::Vec;

pub type Number = agb::fixnum::FixedNum<8>;

// ========================================================================== //
// TODO: need our own Vector2D...?
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

#[derive(Debug, Eq, PartialEq)]
pub struct Rect<T: AGBNumber>(pub AGBRect<T>);
impl<T: AGBNumber> Rect<T> {
    pub fn new(position: Vector2D<T>, size: Vector2D<T>) -> Self {
        Self(AGBRect { position, size })
    }

    pub fn touches(&self, other: &Self) -> bool {
        self.0.touches(other.0)
    }
}
impl<const N: usize> RectMath<FixedNum<N>> for Rect<FixedNum<N>> {
    fn translate(&self, offset: Vector2D<FixedNum<N>>) -> Self {
        Rect(AGBRect {
            position: self.0.position + offset,
            size: self.0.size,
        })
    }

    fn centroid(&self) -> Vector2D<FixedNum<N>> {
        Vector2D {
            x: self.0.position.x + self.0.size.x / 2,
            y: self.0.position.y + self.0.size.y / 2,
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
