#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate core;

mod sh;
mod conditionals;
mod matching;
mod data_structures;
mod pm;
mod strings;
mod guessing_game;
mod functions;
mod traits;
mod dispatch;
mod memory;
mod circular_references;

use std::mem;

const MEANING_OF_LIFE:u8 = 40;
static mut Z:i16 = 234;

fn operators() {
    // arithmetic
    let mut a = 2+3*4;
    // println!("{}", a);
    a = a+1;
    // println!("{}", a);
    // println!("remainder of {} / {} = {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    // println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    // println!("{} cubed = {}, b^pi = {}", b, b_cubed, b_to_pi);

    // bitwise
    let two_to_10 = 3<<10;
    // println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    let x = 5;
    let x_is_5 = x == 5; // true

    // println!("pi is lesser than 4: {}, x = {}: {}", pi_less_4, x, x_is_5);


}

fn main() {
    unsafe {
        // println!("Meaning of Life value: {}", Z);
        Z = 32;
        // println!("Meaning of Life after value: {}", Z);
    }
    operators();
    let a: u8 = 123; // u = unsigned, 8 bits, 0 - 255
    // println!("a = {}", a);  // immutable

    let mut b: i8 = 0;
    // println!("value of b before is: {}", b);
    b = 42;
    // println!("value of b after is: {}", b);

    let mut c = 123456789; // i32
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z, size_of_z, size_of_z*8
    );

    let d:char = 'x';
    // print!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    // let mut g: bool = false;
    // g = true;

    // modules
    // sh::stack_and_heap();
    // conditionals::if_statement();
    // conditionals::while_statement();
    // conditionals::for_loop();
    // matching::country_code_matching();
    // data_structures::structures();
    // data_structures::enums();
    // data_structures::unions();
    // data_structures::option_t();
    // data_structures::arrays();
    // data_structures::slices();
    // data_structures::tuples();
    // data_structures::generics();
    // data_structures::vectors();
    // data_structures::hash_maps();
    // data_structures::hash_sets();
    // data_structures::iterators();
    // pm::pattern_matching();
    // strings::strings();
    // guessing_game::number_guessing();
    // functions::user_functions();
    // functions::methods();
    // functions::closures();
    functions::higher_order();
    traits::traits();
    memory::results();
    circular_references::results();
    // dispatch::results();

}
