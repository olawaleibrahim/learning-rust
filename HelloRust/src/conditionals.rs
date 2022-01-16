pub fn if_statements() {
    let temp = 0;
    if temp > 30 {
        println!("really hot outside");
    }
    else if temp < 5 {
        println!("really cold!");
    }
    else {
        println!("temp is okay.");
    }
    let day = if temp > 25 {"sunny"} else if temp < 0 {"snowy"} else {"cloudy"};
    println!("day is {}", day);
    println!("it is {}", if temp > 25 {"hot"} else if temp < 0 {"cold"} else {"normal"}
    );
    println!("it is {}", if temp > 25 {
        if temp > 40 {"very hot"}  else {"hot"}
    }
    else if temp < 5 {
        if temp < -10 {"extremely cold"}  else {"very cold"}
    }
    else {"normal"}
    );
}

pub fn while_loop_statements() {
    let mut x = 1;
    while x< 1000 {
        x *= 2;
        if x == 64 {continue;}
        println!("x is {}", x);
    }
    let mut y: u32 = 1;
    loop // while true
    {
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 {break;}
    }
}