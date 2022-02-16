use point::point::Point;

#[test]
fn point_creaton() {
    let p = Point::new(1, 1);
    assert_eq!(1, p.x);
    assert_eq!(1, p.y);

    let p = Point::new(1.0, 1.0);
    assert_eq!(1.0, p.x);
    assert_eq!(1.0, p.y);

    let p = Point::new("1", "1");
    assert_eq!("1", p.x);
    assert_eq!("1", p.y);
}

#[test]
fn point_offset() {
    let mut p = Point::new(1, 1);
    assert_eq!(1, p.x);
    assert_eq!(1, p.y);

    let mut counter = 1;
    assert_eq!(1, counter);
    assert_eq!(1, counter);

    for i in -1000..=1000 {
        p.offset(i, i);
        counter += i;
        assert_eq!(p.x, counter);
        assert_eq!(p.y, counter);
    }
}

#[test]
fn point_to_string() {
    let mut p = Point::new(1, 1);
    assert_eq!(1, p.x);
    assert_eq!(1, p.y);

    for i in -1000..=1000 {
        p.x = i;
        p.y = i;
        let test_string = p.to_string();
        assert_eq!(format!("(x:{},y:{})", i, i), test_string);
    }
}


