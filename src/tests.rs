use super::*;
use math_util::{round, Feq, SQRT_2, FRAC_PI_4};
use rstar::Point as RStarPoint;

#[test]
fn test_point() {
    let a = pt![3, 4];
    assert_eq!(a.val(0), 3.);
    assert_eq!(a.val(1), 4.);
    let mut b = pt![4, 5];
    *b.nth_mut(0) = 2.0;
    *b.nth_mut(1) = 1.5;
    assert_eq!(b.nth(0), 2.);
    assert_eq!(b.nth(1), 1.5);
    let c  = Point::generate(|_| 0.3);
    assert_eq!(c.as_tuple(), (0.3, 0.3));

    let pa = pt![3, 4];
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

#[test]
fn test_distance_magnitude() {
    let pt: Point = [3., 0.].into();
    assert_eq!(pt.distance(&Point { x: 0., y: 4. }), 5.0);
    assert_eq!(pt.square_distance(&Point { x: 0., y: 4. }), 25.0);
    let pt: Point = [3., 4.].into();
    assert_eq!(pt.distance(&Point { x: 4., y: 5. }), SQRT_2);
    assert_eq!(pt.square_distance(&Point { x: 4., y: 5. }), 2.0);

    let a = pt![0, 0];
    let b = pt![4, 3];
    let z = pt![0, 0];
    let o = pt![1, 1];
    assert_eq!(o.distance(&z), SQRT_2);
    let x = pt!(-3, 2);
    assert_eq!(round(x.distance(&z), 8),
               round(3.605551275463989, 8));

    assert_eq!(pt!(3, 4).magnitude(), 5.0);
    let x = pt!(3, 4);
    assert_eq!(x.distance(&z), 5.0);
    assert_eq!(a.distance(&b), 5.0);
    let x = pt!(3, 4);
    assert_eq!(x.square_distance(&z), 25.0);
    assert_eq!(pt!(3, 4).square_magnitude(), 25.0);
    assert_eq!(a.square_distance(&b), 25.0);
    let x = pt!(4.587, 0.);
    assert_eq!(x.distance(&z), 4.587);
}

#[test]
fn test_component_negation() {
    let a: Point = (3, 0).into();
    let b: Point = [0., 4.].into();
    let c = Point::component(5., 53.13010235415598f64.to_radians());
    assert_eq!(a.add(&b), c);

    let a: Point = [3., 4.].into();
    let b: Point = [4., 5.].into();
    let subpt = b.neg();
    let c = a.sub(&b);
    assert_eq!(subpt, Point { x: -4., y: -5. });
    assert_eq!(c, Point { x: -1., y: -1. });

    let ar = vec![10., 150., 6.5];
    let er = vec![280., 280., 12.8];
    let a: Point = (&ar).into();
    let b: Point = er[..2].into();

    let pv = b.sub(&a);
    let nv = pv.neg();
    assert_eq!(nv, pv.kproduct(-1.));
    let neg_a: Point = [-10f64, -150.].into();
    assert_eq!(a.neg(), neg_a)
}


#[test]
fn test_orient_dot_cross_product() {
    let a = pt!(1.2, -4.2);
    let b = pt!(1.2, -4.2);
    assert_eq!(19.08, round(a.dot_product(b), 8));

    let a = pt!(237, 289);
    let b = pt!(404.25, 357.25);
    let c = pt!(460, 380);
    let d = pt!(297.13043478260863, 339.30434782608694);
    let e = pt!(445.8260869565217, 350.17391304347825);

    assert_eq!(c.orientation2d(a, b), 0.);
    assert!(d.orientation2d(a, c) < 0.);
    assert!(e.orientation2d(a, c) > 0.);

    let ab = b.sub(&a);
    let ac = c.sub(&a);
    let ad = d.sub(&a);
    let ae = e.sub(&a);
    assert_eq!(ab.cross_product(ac), 0.);
    assert!(ac.cross_product(ad) > 0.);
    assert!(ac.cross_product(ae) < 0.);

    let k = pt!(-0.887, -1.6128);
    let u = pt!(4.55309, 1.42996);
    let testpoints = vec![pt!(2, 2), pt!(0, 2), pt!(0, -2), pt!(2, -2), pt!(0, 0), pt!(2, 0), u, k];
    let left = |x: f64| x < 0.;
    let right = |x: f64| x > 0.;
    let on = |x: f64| x.feq(0.);
    let mut sides = vec![0.0; testpoints.len()];
    for (i, pt) in testpoints.into_iter().enumerate() {
        sides[i] = pt.orientation2d(k, u)
    }
    assert!(pt!(2,2).orientation2d(k, u) < 0.);
    let side_out: Vec<&dyn Fn(f64) -> bool> = vec![&left, &left, &right, &right, &left, &right, &on, &on];
    for i in 0..side_out.len() {
        assert!(side_out[i](sides[i]))
    }
}

#[test]
fn test_unit_project() {
    let a = pt!(0.88682, -1.06102);
    let b = pt!(3.5, 1.0);
    assert_eq!(round(a.project(b), 5), 0.56121);
    assert_eq!(pt!(0., 0.).unit_vector(), pt!(0, 0));
}

#[test]
fn test_direction_rev_direction() {
    let a = pt!(0, 0);
    let b = pt!(-1, 0);
    let v = b.sub(&a);
    assert_eq!(pt!(1, 1).direction(), FRAC_PI_4);//0.7853981633974483f64
    assert_eq!(pt!(-1, 0).direction(), PI);
    assert_eq!(v.direction(), PI);
    assert_eq!(pt!(1, 3f64.sqrt()).direction(), 60f64.to_radians());
    assert_eq!(pt!(0, -1).direction(), 270f64.to_radians());

    let a = pt!(0, 0);
    let b = pt!(-1, 0);
    let v = b.sub(&a);
    assert_eq!(Point::reverse_direction(v.direction()), 0.0);
    assert_eq!(Point::reverse_direction(FRAC_PI_4), FRAC_PI_4 + PI);
    assert_eq!(Point::reverse_direction(FRAC_PI_4 + PI), FRAC_PI_4);
}

#[test]
fn test_deflection_extend() {
    let ln0 = [pt![0, 0], pt![20, 30]];
    let ln1 = [pt![20, 30], pt![40, 15]];
    let v0 = ln0[1].sub(&ln0[0]);
    let v1 = ln1[1].sub(&ln1[0]);

    assert_eq!(
        round(Point::deflection_angle(v0.direction(), v1.direction()), 10),
        round(93.17983011986422f64.to_radians(), 10));
    assert_eq!(
        round(Point::deflection_angle(v0.direction(), v0.direction()), 10),
        0f64.to_radians());

    let ln1 = [pt![20, 30], pt![20, 60]];
    let v1 = ln1[1].sub(&ln1[0]);
    assert_eq!(
        round(Point::deflection_angle(v0.direction(), v1.direction()), 10),
        round(-33.690067525979806f64.to_radians(), 10), );

    const PRECISION: i32 = 8;

    let a2 = pt!(0.88682, -1.06102);
    let b2 = pt!(3.5, 1);
    let c2 = pt!(-3, 1);
    let d2 = pt!(-1.5, -3);

    let va = a2;
    let vb = b2;
    let vc = c2;
    let vd = d2;
    let vdb = b2.sub(&d2);
    let vbc = c2.sub(&b2);

    assert_eq!(round(va.direction(), PRECISION),
               round(309.889497029295f64.to_radians(), PRECISION),
    );
    assert_eq!(round(vb.direction(), PRECISION),
               round(15.945395900922854f64.to_radians(), PRECISION),
    );
    assert_eq!(round(vc.direction(), PRECISION),
               round(161.565051177078f64.to_radians(), PRECISION),
    );
    assert_eq!(round(vd.direction(), PRECISION),
               round(243.43494882292202f64.to_radians(), PRECISION),
    );
    assert_eq!(round(vdb.magnitude(), 4),
               round(6.4031242374328485, 4),
    );
    assert_eq!(round(vdb.direction(), PRECISION),
               round(38.65980825409009f64.to_radians(), PRECISION),
    );
    let defl_angle = 157.2855876468f64;
    let vo = vdb.extend(
        3.64005494464026,
        (180.0 + defl_angle).to_radians(),
        true,
    );

    assert_eq!(round(vo[0], PRECISION),
               round(-vb[0], PRECISION),
    );
    assert_eq!(round(vo[1], PRECISION),
               round(-vb[1], PRECISION),
    );

    // "vo by extending vdb by angle to origin"
    // "vo by extending vdb by angle to origin"
    let defl_angle_b = 141.34019174590992f64;

    // extend to c from end
    let vextc = vdb.extend(6.5, (180.0 + defl_angle_b).to_radians(), true);
    assert_eq!(round(vbc[0], PRECISION), round(vextc[0], PRECISION));
    assert_eq!(round(vbc[1], PRECISION), round(vextc[1], PRECISION));

    // vextc with magnitudie extension from vdb Pnts
    assert_eq!(round(vextc[0], PRECISION), -vextc.magnitude());
    // vextc horizontal vector test:  extension from vdb Pnts
    assert_eq!(round(vextc[1], PRECISION), 0.);

    let c = pt!(5, 0).deflect(2., 90f64.to_radians(), true);
    //deflection is the right hand angle
    assert_eq!(round(c.x, PRECISION), round(0.0, PRECISION));
    assert_eq!(round(c.y, PRECISION), round(-2., PRECISION));

    let c = pt!(5, 0).deflect(2., 90f64.to_radians(), false);
    assert_eq!(round(c.x, PRECISION), round(0.0, PRECISION));
    assert_eq!(round(c.y, PRECISION), round(2., PRECISION));
}

#[test]
fn test_distance_to_vector() {
    let a = pt!(16.82295, 10.44635);
    let b = pt!(28.99656, 15.76452);
    let on_ab = pt!(25.32, 14.16);

    let tpoints = vec![
        pt!(30., 0.),
        pt!(15.78786, 25.26468),
        pt!(-2.61504, -3.09018),
        pt!(28.85125, 27.81773),
        a, b, on_ab, ];

    let t_dists = [14.85, 13.99, 23.69, 12.05, 0.00, 0.00, 0.00];
    let mut dists = vec![0.0; tpoints.len()];

    for (i, tp) in (&tpoints).into_iter().enumerate() {
        dists[i] = tp.distance_to_segment(a, b)
    }

    assert_eq!(pt!(30., 0.).distance_to_segment(a, a), a.distance(&pt!(30., 0.)));
    assert_eq!(pt!(30., 0.).distance_to_segment(b, b), b.distance(&pt!(30., 0.)));
    assert_eq!(pt!(30., 0.).square_distance_to_segment(b, b), b.square_distance(&pt!(30., 0.)));

    for i in 0..tpoints.len() {
        assert_eq!(round(dists[i], 2), round(t_dists[i], 2));
    }
}

#[test]
fn test_serialize_deserialize() {
    let point = pt!(1, 2);
    let serialized = serde_json::to_string(&point).unwrap();
    assert_eq!(serialized, String::from(r#"{"x":1.0,"y":2.0}"#));
    let deserialized: Point = serde_json::from_str(r#"{"x":1.0,"y":2.0}"#).unwrap();
    assert_eq!(point, deserialized);
    let deser_array: Point = serde_json::from_str("[1.0,2.0]").unwrap();
    assert_eq!(point, deser_array);
}