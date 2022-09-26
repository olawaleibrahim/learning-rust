fn ownership() {
    let v = vec![1, 2, 3];

    let print_vector = |x:Vec<i32>| -> Vec<i32> {
        print!("{:?}", x);
        x
    };

    let vv = print_vector(v);
    println!("{}", vv[0]);
}

fn borrowing() {
    let print_vector = |x:&Vec<i32>| {
        println!("{:?}", x[0]);
    };

    let v = vec![3, 2, 1];
    print_vector(&v);

    let mut a = 40;
    let b = &mut a;
    *b += 2;

    println!("a = {}", a);

    let mut z = vec![4, 3, 2, 45];
    for i in &z {
        println!("i = {}", i);
        // z.push(6);
    }
}

pub fn results() {
    ownership();
    borrowing();
}