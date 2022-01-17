#![allow(unused_variables)]
#![allow(dead_code)]
use std::mem;

struct Point {
    x: f64,
    y: f64
}
struct Line {
    start: Point,
    end: Point
}

pub fn structures() {
    let p1 = Point {
        x: 3.0,
        y: 4.0
    };
    let p2 = Point{x:4.0, y:5.0};
    println!("point p is at {}, {} ", p1.x, p1.y);

    let myline = Line {
        start: p1, end: p2
    };
}

pub fn arrays()
{
    // 1-D array
    let mut a:[i32;5] = [1,2,3,4,5];
    a[0] = 321;
    println!("list a : {:?}", a);
    println!("list a has {} elements, first is {}", a.len(), a[0]);

    let b = [1u8; 10];
    for (i, idx) in (0..b.len()).enumerate() {
        println!("{}, index:{}", b[i], idx);
    }
    println!("b takes up {} bytes, a = {value} bytes", mem::size_of_val(&b),
             value=mem::size_of_val(&a));
    // 2-D array

    let mtx:[[f32;3]; 2] =
    [
        [1.0, 0.0, 0.0],
        [1.0, 2.0, 0.0]
    ];
    println!("2-D array: {:?}", mtx);
    // print out the diagonal
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i==j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}