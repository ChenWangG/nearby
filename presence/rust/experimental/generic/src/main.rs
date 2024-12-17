struct Point<T> {
    x: T,
}

impl Point<u8> {
    fn print(&self) {
        println!("This is u8 Point {}", self.x);
    }
}

impl Point<f32> {
    fn print(&self) {
        println!("This is f32 Point {}", self.x);
    }
}

impl<T: std::fmt::Debug> Point<T> {
    fn generic_print(&self) {
        println!("This is generic point {:?}.", self.x);
    }

    fn mixed<F: std::fmt::Display>(&self, f: F) {
        println!("This is mixed point {}.", f);
    }
}

fn main() {
    let point = Point { x: 7 };
    point.print();
    let f_point = Point { x: 1.0};
    f_point.print();
    let g_point = Point { x: "hello"};
    g_point.generic_print();
    g_point.mixed(1);
}
