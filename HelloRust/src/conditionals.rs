pub fn if_statement() {
    let temp = 9;
    if temp > 30 {
        println!("really hot outside");
    }
    else if temp < 5 {
        println!("really cold");
    }
    else {
        println!("temperature is OK")
    }
    let day = if temp > 20 {"sunny"} else if temp < 5 {"cloudy"} else {"okay"};
    println!("Today is {}", day);
}

pub fn while_statement() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue;
        }
        println!("{}", x);
    }
    let mut y = 1;
    loop {// while true {
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 {
            break;
        }

    }

}

pub fn for_loop() {
    for x in 1..11 {
        if x == 3 {
            continue;
        }
        if x == 8 {
            break;
        }
        println!("x = {}", x);
    }
    for (pos,y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}