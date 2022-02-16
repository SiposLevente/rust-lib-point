pub mod point {
    use std::fmt::{Display, Formatter, Result};
    use std::ops::{Add, AddAssign, Mul, Sub};

    pub struct Point<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Point<T> {
        pub fn new(x: T, y: T) -> Point<T> {
            Point { x, y }
        }

        pub fn offset(&mut self, x: T, y: T)
        where
            T: AddAssign,
        {
            self.x += x;
            self.y += y;
        }

        pub fn distance(&self, other_point: Point<T>) -> T
        where
            T: Sub<Output = T> + Add<Output = T> + Mul<Output = T> + Copy,
        {
            let point = Point::new(self.x, self.y);
            let sub = other_point - point;
            (sub.x * sub.x) + (sub.y * sub.y)
        }

        pub fn to_string(&self) -> String
        where
            T: Display,
        {
            format!("(x:{},y:{})", self.x, self.y)
        }
    }

    impl<T> Add for Point<T>
    where
        T: Add<Output = T>,
    {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl<T> Sub for Point<T>
    where
        T: Sub<Output = T>,
    {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl<T> PartialEq for Point<T>
    where
        T: PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    impl<T> Display for Point<T>
    where
        T: Display,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "({},{})", self.x, self.y)
        }
    }
}
