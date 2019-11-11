# point {x, y}
cartesian point type. Implements [`coordinate`](https://github.com/intdxdt/coordinate) trait for `f64` types. Point geometric operations make sense in 
floating point (more precision preferred ~ `f64`).

Point can be treated as point with `x` and `y` or vector with `x` and `y` components.

## examples 
```rust
//distance and magnitude
let a = pt![0, 0];
let b = pt!(4, 3);
let z = Point{x:0, y:0};
let o = Point::new(1., 1.);
assert_eq!(o.distance(&z), SQRT_2);
assert_eq!(pt!(3, 4).magnitude(), 5.0);
let x = pt!(3, 4);
assert_eq!(x.distance(&z), a.distance(&b));

let a: Point = (3, 0).into();
let b: Point = [0., 4.].into();
let c = Point::component(5., 53.13010235415598f64.to_radians()); //from magnitude & direction
assert_eq!(a.add(&b), c);

let pv = b.sub(&a);
let nv = pv.neg();
assert_eq!(nv, pv.kproduct(-1.));

//distance from point to line segment
let a = pt!(16.82295, 10.44635);
let b = pt!(28.99656, 15.76452);
let tp = pt!(30., 0.);
tp.distance_to_segment(a, b);
```

```rust
//robust orientation
let a = pt!(237, 289);
let b = pt!(404.25, 357.25);
let c = pt!(460, 380);
let d = pt!(297.13043478260863, 339.30434782608694);
let e = pt!(445.8260869565217, 350.17391304347825);

/// = 0 if a, b, and c are coplanar
/// < 0 if ccw - self is on left of segment
/// > 0 if cw - c is on right of segment

assert_eq!(c.orientation2d(a, b), 0.); //orientation of c relative segment a-b
assert!(d.orientation2d(a, c) < 0.);   //orientation of d relative segment a-c
assert!(e.orientation2d(a, c) > 0.);   //orientation of e relative segment a-c
```

```rust
///See tests.rs for more examples on Point API
```

## coverage 
```bash
[INFO tarpaulin] Coverage Results:
|| Uncovered Lines:
|| Tested/Total Lines:
|| src/lib.rs: 115/115
|| src/tests.rs: 212/212
|| 
100.00% coverage, 327/327 lines covered

```

## lic 
`MIT`
