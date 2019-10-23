use math_util::feq;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    pub fn new_from_array(a: [f64; 2]) -> Point {
        Point { x: a[0], y: a[1] }
    }

    ///Operator : equals
    pub fn equals(&self, other: &Point) -> bool {
        feq(self.x, other.x) && feq(self.y, other.y)
    }

    ///As array
    pub fn as_array(&self) -> [f64; 2] {
        [self.x, self.y]
    }
    ///As tuple
    pub fn as_tuple(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn test_point() {
        let pa = Point::new_from_array([3.0, 4.0]);
        let pb = Point::new(3.0, 4.0);
        let pc = Point::new(5.0, 4.0);
        assert_eq!(pa.x, 3.0);
        assert_eq!(pa.y, 4.0);
        assert_eq!(pa, pb);
        assert_ne!(pa, pc);
        assert_ne!(pb, pc);
        assert!(pb != pc);
        assert_eq!(pa.as_array(), [3.0, 4.0]);
        assert_eq!(pa.as_tuple(), (3.0, 4.0));
        assert!(pa.equals(&pb));
    }
}
