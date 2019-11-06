use coordinate::Coordinate;
use std::fmt::Debug;
use math_util::{Numeric, Feq};


#[derive(Copy, Clone, PartialOrd, Debug)]
pub struct Point<T>
    where T: Numeric + Feq {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
    where T: Numeric + Feq {
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
}

impl<T> Eq for Point<T>
    where T: Numeric + Feq {}

impl<T> PartialEq for Point<T>
    where T: Numeric + Feq {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}

impl<T> Coordinate for Point<T> where
    T: Numeric + Feq {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let i32pa = Point::new_from_array(&[3, 4]);
        let i64pa = Point::new_from_array(&[3i64, 4]);
        let pa = Point::new_from_array(&[3.0, 4.0]);
        let mut m_pa = Point::new_from_array(&[3.0, 4.0]);
        let pb = Point::new(3.0, 4.0);
        let pc = Point::new(5.0, 4.0);

        assert_eq!(i32pa.as_tuple(), (3, 4));
        assert_eq!(i64pa.as_tuple(), (3i64, 4));
        assert_eq!(i32pa.as_array(), [3, 4]);
        assert_eq!(i64pa.as_array(), [3i64, 4]);

        assert_eq!(pa.as_tuple(), (3., 4.));
        assert_eq!(pa.as_array(), [3.0, 4.0]);
        assert_eq!((pa.nth(0), pa.nth(1)), (3., 4.));
        assert_eq!((m_pa.nth(0), m_pa.nth(1)), (3., 4.));
        *m_pa.nth_mut(0) = 0.;
        *m_pa.nth_mut(1) = 5.;
        assert_eq!((m_pa.nth(0), m_pa.nth(1)), (0., 5.));
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
