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

fn say_hello() {
    println!("hello!")
}

pub fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x:i32| -> i32 { x + 1 };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let two = 2;
    {
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
        let borrow_two = two;
        println!("borrow_two = {}", borrow_two);
    }

    let plus_three = |x:&mut i32| *x += 3;

    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}