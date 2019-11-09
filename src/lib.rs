use coordinate::Coordinate;
use std::fmt::Debug;
use math_util::{Feq, Flt, const_tau, const_pi, EPSILON};
use std::ops::{Index, IndexMut};

/// Point is a 2D (x:float, y:float) point type.
/// float : f32 & f64 - required for most computations
/// requiring area, distance, trigonometry, etc.
#[derive(Copy, Clone, PartialOrd, Debug)]
pub struct Point<T>
    where T: Flt {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
    where T: Flt {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
    pub fn new_from_array(a: &[T; 2]) -> Point<T> {
        Point { x: a[0], y: a[1] }
    }

    ///Operator : equals
    pub fn equals(&self, other: &Point<T>) -> bool {
        self.x.feq(other.x) && self.y.feq(other.y)
    }

    ///As array
    pub fn as_array(&self) -> [T; 2] {
        [self.x, self.y]
    }

    ///As tuple
    pub fn as_tuple(&self) -> (T, T) {
        (self.x, self.y)
    }

    ///Computes vector magnitude given x an dy component
    pub fn magnitude(&self) -> T {
        self.x.hypot(self.y)
    }

    ///Point from magnitude and direction
    pub fn component(m: T, d: T) -> Point<T> {
        Point::new(m * d.cos(), m * d.sin())
    }

    ///Dot Product of two points as vectors
    pub fn dot_product(&self, o: Point<T>) -> T {
        (self.x * o.x) + (self.y * o.y)
    }

    ///2D cross product of AB and AC vectors,
    ///i.e. z-component of their 3D cross product.
    ///negative cw and positive if ccw
    pub fn cross_product(&self, b: Point<T>) -> T {
        return (self.x * b.y) - (self.y * b.x);
    }

    ///Deflect_vector computes vector deflection given deflection angle and
    /// side of vector to deflect from (from_end)
    pub fn deflect(&self, mag: T, defl_angle: T, from_end: bool) -> Point<T> {
        return self.extend(mag, T::PI() - defl_angle, from_end);
    }

    ///kproduct scales x and y components by constant  k
    pub fn kproduct(&self, k: T) -> Point<T> {
        self.mult(k)
    }

    ///Dir computes direction in radians - counter clockwise from x-axis.
    pub fn direction(&self) -> T {
        let mut d = self.y.atan2(self.x);
        if d < T::zero() {
            d += const_tau()
        }
        return d;
    }

    ///Revdir computes the reversed direction from a foward direction
    pub fn reverse_direction(d: T) -> T {
        let pi = T::PI();
        let mut r = d - pi;
        if d < pi {
            r = d + pi;
        }
        return r;
    }

    ///deflection angle
    pub fn deflection_angle(bearing1: T, bearing2: T) -> T {
        let mut a = bearing2 - Point::reverse_direction(bearing1);
        if a < T::zero() {
            a = a + const_tau();
        }
        return T::PI() - a;
    }


    ///Unit vector of point
    pub fn unit_vector(&self) -> Point<T> {
        let mut m = self.magnitude();
        if m.feq(T::zero()) {
            m = T::from(EPSILON).unwrap();
        }
        Point::new(self.x / m, self.y / m)
    }

    ///Projects self on to v
    pub fn project(&self, v: Point<T>) -> T {
        return self.dot_product(v.unit_vector());
    }

    ///2D cross product of AB and AC vectors given A, B, and Pnts as points,
    ///i.e. z-component of their 3D cross product.
    ///Returns a positive value, if ABC makes a counter-clockwise turn,
    ///negative for clockwise turn, and zero if the points are collinear.
//    fn orientation2d(a:Point<T>, b:Point<T>, c:Point<T>) -> T{
//        return robust.Orientation2D(a[0..2], b[0..2], c[0..2])
//    }

    ///Extends vector from the end or beginning based on `from_end`.
    pub fn extend(&self, magnitude: T, angle: T, from_end: bool) -> Point<T> {
        //from a of v back direction initiates as fwd v direction anticlockwise
        //bb - back bearing
        //fb - forward bearing
        let mut bb = self.direction();
        if from_end {
            bb += const_pi();
        }
        let mut fb = bb + angle;
        let tau: T = const_tau();
        if fb > tau {
            fb -= tau;
        }
        return Point::component(magnitude, fb);
    }
}

impl<T> Eq for Point<T>
    where T: Flt {}

impl<T> PartialEq for Point<T>
    where T: Flt {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}

impl<T> Coordinate for Point<T> where
    T: Flt {
    type Scalar = T;
    const DIM: usize = 2;

    fn gen(dim_val: impl Fn(usize) -> Self::Scalar) -> Self {
        Point {
            x: dim_val(0),
            y: dim_val(1),
        }
    }

    fn nth(&self, i: usize) -> Self::Scalar {
        match i {
            0 => self.x,
            1 => self.y,
            _ => unreachable!(),
        }
    }

    fn nth_mut(&mut self, i: usize) -> &mut Self::Scalar {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => unreachable!(),
        }
    }
}

impl<T> Index<usize> for Point<T> where T: Flt {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => unreachable!(),
        }
    }
}

impl<T> IndexMut<usize> for Point<T> where T: Flt {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.nth_mut(i)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let pa = Point::new_from_array(&[3.0, 4.0]);
        let mut m_pa = Point::new_from_array(&[3.0, 4.0]);
        let pb = Point::new(3.0, 4.0);
        let pc = Point::new(5.0, 4.0);

        assert_eq!(pa.as_tuple(), (3., 4.));
        assert_eq!(pa.as_array(), [3.0, 4.0]);
        assert_eq!((pa[0], pa[1]), (3., 4.));
        assert_eq!((pa.nth(0), pa.nth(1)), (3., 4.));
        assert_eq!((m_pa.nth(0), m_pa.nth(1)), (3., 4.));
        m_pa[0] = 0.;
        m_pa[1] = 5.;
        assert_eq!((m_pa[0], m_pa[1]), (0., 5.));
        assert!(m_pa.square_length().feq(25.0));

        assert_eq!(pa, pb);
        assert_ne!(pa, pc);
        assert_ne!(pb, pc);
        assert!(pb != pc);
        assert!(pa.equals(&pb));

        let cb = pb.comp(&pc);
        assert_eq!(cb.as_tuple(), (-2.0, 0.0));
    }
}
