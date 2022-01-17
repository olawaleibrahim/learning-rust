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
        Color::CmykColor{cyan,magenta,yellow,black} => println!("cymk {},{},{},{}",
                                                                            cyan, magenta, yellow, black);
        _ => ()
    }
}