#[derive(Copy, Clone , Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y}
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn test_point() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(p.x, 3.0);
        assert_eq!(p.y, 4.0);
    }
}
