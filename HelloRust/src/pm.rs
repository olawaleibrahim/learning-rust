fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        z @ 9..=11 => "lots of oranges",
        _ if (x % 2 == 0) => "some",
        _ => "a few",
    }
}

pub fn pattern_matching() {
    for i in 0..13 {
        println!("{}: I have {} oranges", i, how_many(i));
    }
    let point = (2, 9);
    match point {
        (0,0) => println!("origin"),
        (0,y) => println!("x axis, y = {}", y),
        (x,0) => println!("y axis, x = {}", x),
        (_,y) => println!("?,{}",y),
    }
}