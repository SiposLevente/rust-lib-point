pub mod point {
    pub struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        pub fn new(x: T, y: T) -> Point<T> {
            Point { x, y }
        }

        pub fn x(&self) -> T
        where
            T: Copy,
        {
            self.x
        }

        pub fn y(&self) -> T
        where
            T: Copy,
        {
            self.y
        }

        pub fn set_x(&mut self, x: T) {
            self.x = x;
        }

        pub fn set_y(&mut self, y: T) {
            self.x = y;
        }

        pub fn offset(&mut self, x: T, y: T)
        where
            T: std::ops::AddAssign,
        {
            self.x += x;
            self.y += y;
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

        pub fn to_string(&self) -> String
        where
            T: std::fmt::Display,
        {
            format!("(x:{}, y:{})", self.x, self.y)
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
