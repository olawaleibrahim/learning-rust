fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) {
    *x *= 2;
}

fn find_product(x: i32, y: i32) -> i32 {
    x * y
}

pub fn user_functions() {
    print_value(34);
    let mut z = 16;
    increase(&mut z);
    println!("z = {}", z);

    let a = 45;
    let b = 28;
    let prod = find_product(a, b);
    println!("product of {} and {} = {}", a, b, prod);
}

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

pub fn methods() {
    let p1 = Point{ x: 0.0, y: 0.0 };
    let p2 = Point { x:4.0, y: 5.0 };
    let line = Line { start: p1, end: p2 };
    println!("length = {}", line.len());
}