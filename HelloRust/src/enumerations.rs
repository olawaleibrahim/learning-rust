#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;
use crate::enumerations::Color::CmykColor;

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8), // tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8}  //struct
}
pub fn enums() {
    let c:Color = Color::CmykColor {cyan:0, magenta: 128, yellow:0, black:255};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RGBColor(0, 0, 0)
        | Color::CmykColor{cyan:_,magenta:_,yellow:_,black:255} => println!("black"),
        Color::RGBColor(ab, bc, cd) => println!("rgb({},{},{})", ab, bc, cd),
        //Color::CmykColor{cyan,magenta,yellow,black} => println!("cmyk ({},{},{},{})",
                                                                //cyan, magenta, yellow, black);
        _ => ()
    }
}

union IntOrFloat{
    i: i32,
    f: f32
}

fn process_value(iof:IntOrFloat)
{
    unsafe {
        match iof {
            IntOrFloat{i:42} => {
                println!("meaning of life value")
            }
            IntOrFloat{f} => {
                println!("value = {}", f);
            }
        }
    }
}

pub fn unions(){
    let mut iof = IntOrFloat{i:123};
    iof.i = 234;

    let value = unsafe {iof.i};
    println!("iof.i:{}", value);

    process_value(IntOrFloat{f:4.0});
}