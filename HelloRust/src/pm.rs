fn how_many(no:i32) -> &'static str {
    match no {
        0 => "no",
        1 => "one",
        2 | 3  => "2 or three",
        z @ 4..=7 => "a few",
        12 => "dozen",
        _ if (no % 2 == 0) => "some",
        _ => "a lot of",
    }
}
pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }
    let point = (0, 4);
    match point {
        (0, 0) => println!("point is at origin"),
        (0, y) => println!("point is on x origin, at y = {}", y),
        (x, 0) => println!("point is on y origin, at y = {}", x),
        (x, y) => println!("point is at position ({},{})", x, y),
    }
}