use crate::types::{Rect, RectMath};
use agb::fixnum::num;
use agb::fixnum::FixedNum;
use agb::fixnum::Number as AGBNumber;
use agb::fixnum::Vector2D;
use agb::println;

// FIXME: we want to support,
//  1. Circle
//  2. AABB
//  3. Rect
//  4. Arbitrary polygon
//  5. Rays
// in at least collision detection

pub struct SeparationResult<N>
where
    N: AGBNumber,
{
    pub separation: Vector2D<N>,
    pub normal: Vector2D<N>,
    pub distance: N,
}

// This trait extends shapes with maths (RectMath) to define
// intersection areas and collision normals.
pub trait Intersects<N>: RectMath<N>
where
    N: AGBNumber,
{
    type Shape;
    fn intersection(&self, other: &Self::Shape) -> Option<Self::Shape>;
    fn separation(&self, other: &Self::Shape) -> Option<SeparationResult<N>>;
}

impl<const N: usize> Intersects<FixedNum<N>> for Rect<FixedNum<N>> {
    type Shape = Rect<FixedNum<N>>;

    fn intersection(&self, other: &Self::Shape) -> Option<Self::Shape> {
        if let Some(overlap) = self.0.overlapping_rect(other.0) {
            Some(Self::Shape { 0: overlap })
        } else {
            None
        }
    }

    fn separation(&self, other: &Self::Shape) -> Option<SeparationResult<FixedNum<N>>> {
        if let Some(mut intersection) = self.intersection(other) {
            // Unless equal, only consider the largest axis of separation
            // for AABBs (rectangles)
            println!("Found intersection of size {}/{}", intersection.0.size.x, intersection.0.size.y);
            if intersection.0.size.x.abs() > intersection.0.size.y.abs() {
                intersection.0.size.y = num!(0.)
            } else {
                intersection.0.size.x = num!(0.)
            }

            let distance = intersection.0.size.magnitude();
            Some(match distance == num!(0.) {
                // Assume separation in x axis if they're on top of one another
                true => SeparationResult {
                    separation: intersection.0.size,
                    normal: Vector2D {
                        x: self.0.size.x,
                        y: num!(0.0),
                    },
                    distance: num!(1.0),
                },
                false => {
                    if self.0.position.x > other.0.position.x {
                        intersection.0.size.x = -intersection.0.size.x;
                    }
                    if self.0.position.y > other.0.position.y {
                        intersection.0.size.y = -intersection.0.size.y;
                    }

                    let normal = intersection.0.size / distance;
                    SeparationResult {
                        separation: intersection.0.size,
                        normal,
                        distance,
                    }
                }
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Number, Rect};
    use agb::fixnum::{num, Vector2D};

    use super::Intersects;

    #[test_case]
    fn test_rect_non_intersects(_gba: &mut agb::Gba) {
        let rect_a = Rect::<Number>::new(
            Vector2D::new(num!(0.), num!(0.)),
            Vector2D::new(num!(4.), num!(4.)),
        );
        let rect_b = Rect::<Number>::new(
            Vector2D::new(num!(5.), num!(5.)),
            Vector2D::new(num!(6.), num!(7.)),
        );

        let test_intersects = rect_a.intersection(&rect_b);
        assert!(test_intersects == None);
    }

    #[test_case]
    fn test_rect_intersects(_gba: &mut agb::Gba) {
        let rect_a = Rect::<Number>::new(
            Vector2D::new(num!(0.), num!(0.)),
            Vector2D::new(num!(4.), num!(4.)),
        );
        let rect_b = Rect::<Number>::new(
            Vector2D::new(num!(2.), num!(1.)),
            Vector2D::new(num!(5.), num!(6.)),
        );

        let intersection = Rect::<Number>::new(
            Vector2D::new(num!(2.), num!(1.)),
            Vector2D::new(num!(2.), num!(3.)),
        );

        let test_intersects = rect_a.intersection(&rect_b).unwrap();
        assert_eq!(intersection, test_intersects);
    }
}

// TODO: for a rotated rectangle, we can use Separating Axis Theorem
// https://stackoverflow.com/questions/10962379/how-to-check-intersection-between-2-rotated-rectangles
// (not SAT) https://stackoverflow.com/questions/62028169/how-to-detect-when-rotated-rectangles-are-colliding-each-other
