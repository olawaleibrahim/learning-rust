mod sh;
mod conditionals;
#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;
const MEANING_OF_RUST: u8 = 25; //no fixed address
static mut Z:i32 = 123;

fn operators() {
    unsafe {
        Z = 155;
        println!("MEANING_OF_RUST is a constant with value: {}, Z: {}", MEANING_OF_RUST, Z);
    }

    {
        let b = 22;
        println!("inside, b = {}", b);
    }
    // arithmetic
    let mut a = 2+3*4;
    println!("a = {}", a);
    a += 10;
    println!("new a = {}", a);
    let cubed_a: i32 = i32::pow(a, 3);
    println!("{} cube is {}", a, cubed_a);

    let b = 2.5;
    let cubed_b = f64::powf(b, 3.0);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, b raise to power of pi ={}", b, cubed_b, b_to_pi);
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // bitwise
    let c = 1 | 2; // | OR & AND ^  XOR ! NOR
                        // 01 OR 10 = 11 == 3_10
    println!("1|2 = {}", c);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    // > <= >= ==
    let x = 5;
    let x_is_5 = x == 5; // true
    println!("{} less than 4 is {}, and x = 5 {}", std::f64::consts::PI, pi_less_4, x_is_5)
}

fn data_types() {
    // usize isize
    let z: isize = 124;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);

    let mut c = 12; //i32
    println!("c = {} takes up {} bytes", c, mem::size_of_val(&c) );
    c = -1;
    println!("c = {}", c);

    let d: char = 'x';
    println!("d = {}, takes up {} bytes", d, mem::size_of_val(&d));

    let a: u8 = 123;// u = unsigned, 8 bits, 0-255
    println!("a = {}", a);// immutable

    let mut b: i8 = 0;
    println!("b = {} before ", b);
    b = 42;
    println!("b = {} after", b);
}

fn double_value(v: i32) -> i32 {
    v*2
}

fn main() {
    // sh::stack_and_heap();
    // operators();
    // data_types();
    let mut x: i32 = 3;
    x = double_value(x);
    println!("X : {}", x);
    x = 42;
    println!("X : {}", x);

    conditionals::if_statements();
    conditionals::while_loop_statements();
}
