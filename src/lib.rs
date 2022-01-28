pub mod point {
    pub struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        pub fn new(x: T, y: T) -> Point<T> {
            Point { x, y }
        }

        pub fn distance(&self, other_point: Point<T>) -> T
        where
            T: std::ops::Sub<Output = T>
                + std::ops::Add<Output = T>
                + std::ops::Mul<Output = T>
                + Copy,
        {
            let point = Point::new(self.x, self.y);
            let sub = other_point - point;
            (sub.x * sub.x) + (sub.y * sub.y)
        }

        pub fn print(&self) -> String
        where
            T: std::fmt::Display,
        {
            format!("({},{})", self.x, self.y)
        }
    }

    impl<T> std::ops::Add for Point<T>
    where
        T: std::ops::Add<Output = T>,
    {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl<T> std::ops::Sub for Point<T>
    where
        T: std::ops::Sub<Output = T>,
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
}
