use std::mem;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::data_structures::Color::CmykColor;

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point
}


enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CmykColor{cyan: u8, magenta: u8, yellow: u8, black: u8},
}


union IntOrFloat {
    i: i32,
    f: f32
}

pub fn structures() {
    let p = Point {
        x: 3.0, y: 4.0
    };
    println!("point p is at {}, {}", p.x, p.y);
    let p2 = Point {
        x: 5.0, y: 6.0
    };
    let myline = Line {
        start: p, end: p2
    };
    println!("Line is from point {},{} to point {},{}", myline.start.x, myline.start.y, myline.end.x, myline.end.y);
}

pub fn enums() {
    // let c:Color = Color::RgbColor(0,1,0);
    let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)
            | Color::CmykColor{cyan: _, magenta: _, yellow: _, black: 255} => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})", r,g,b),
        _ => ()
    }
}

pub fn unions() {
    let mut iof = IntOrFloat{i: 123};
    iof.i = 234;

    let value = unsafe {iof.i};
    println!("iof.i = {}", value);
    process_value(IntOrFloat{i: 20});

}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat {i: 42} => {
                println!("meaning of life");
            }
            IntOrFloat {i} => {
                println!("value = {}", i);
            }
            IntOrFloat {f} => {
                println!("value = {}", f);
            }
        }
    }
}

pub fn option_t() {
    let x = 3.0;
    let y = 2.0;

    let result =
        if y != 0.0 { Some(x/y) } else { None };
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("cannot divide by zero")
    }
}

pub fn arrays() {
    let mut a:[i32;5] = [1,2,3,4,5];
    println!("a has {} elements, first is {}",
            a.len(), a[0]);
    a[0] = 321;
    println!("a[0] = {}", a[0]);
    println!("{:?}", a);
    if a != [1,2,3,4,5] {
        println!("does not match");
    };
    if a == [1,2,3,4,5] {
        println!("match");
    }
    else {
        println!("does not match too");
    };
    let b = [1u8; 10];
    for i in 0..b.len() {
        println!("{}", b[i]);
    }
    println!("b took up {} bytes", mem::size_of_val(&b));
    let mtx:[[f32;3];2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("matrix: {:?}", mtx);
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{},{}] = {}", i, j, mtx[i][j]);
            };
        }
    }
    for x in &mtx { println!("{:?}", x); }
}

fn use_slice(slice: &mut[i32]) {
    println!("first elem: {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}
pub fn slices() {
    let mut data = [1,2,3,4,5];
    use_slice(&mut data[1..4]);
    println!("{:?}", data);
}

fn sum_and_product(x:i32, y:i32) -> (i32, i32) {
    (x+y, x*y)
}

pub fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    //destructuring
    let (a,b) = sp;
    println!("a = {}, b = {}", a, b);
    let sp2 = sum_and_product(4,7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c,d), (e,f)) = combined;
    // tuple elements do not have to have the same type elements unlike arrays
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    // single element tuple
    let meaning = (42,);
    println!("{:?}", meaning);
}

// Option<T>
struct Point1<T> {
    x: T,
    y: T
}

struct Point2<T,V> {
    x: T,
    y: V
}

struct Line1<T,V,W> {
    start: Point1<T>,
    end: Point2<W,V>
}

pub fn generics() {
    let a: Point1<u8> = Point1 { x: 0, y: 0};
    let b: Point2<f64, f32> = Point2 { x: 1.0, y: 3.4};

    let myline = Line1 { start: a, end: b};
}

pub fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    a.push(333);

    println!("{:?}", a);

    let idx:usize = 2;
    println!("a[0] = {}", a[0]);

    // Option
    match a.get(idx) {
        Some(x) => println!("a[{}] = {}", idx, x),
        None => println!("error, no such element")
    }
    for x in &a { println!("{}", x); }
    a.push(44);
    println!("{:?}", a);
    let last_elem = a.pop();
    println!("last element in vector = {:?}, new vector = {:?}", last_elem, a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

pub fn hash_maps() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);

    for (key, value) in &shapes {
        println!("{}: {}", key, value);
    }
    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);
    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes.entry(
            "circle".into()
        ).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);
}

pub fn hash_sets() {
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);
    greeks.insert("delta");

    let added_vega = greeks.insert("beta");
    if added_vega {
        println!("we added vega! hooray!")
    }

    if !greeks.contains("kappa") {
        println!("we don't have kappa")
    }
    greeks.insert("kappa");
    if greeks.contains("kappa") {
        println!("we have kappa")
    }

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // subset
    println!("is {:?} a subset of {:?} ? {}", _2_8, _1_10, _2_8.is_subset(&_1_10));

    // disjoint = no common elements
    println!("is {:?} disjointed from {:?}? {}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));

    // union, intersection
    println!("items in either {:?} and {:?} are {:?}", _2_8, _6_10, _2_8.union(&_6_10));
}

pub fn iterators() {
    let mut vec = vec![3, 2, 1];
    for x in vec.iter_mut() {
        *x += 2
    }
    println!("vec = {:?}", vec);
    let mut vec2 = vec![1, 2, 3];
    vec2.extend(vec);
    println!("vec2 = {:?}", vec2);
}