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

struct Person {
    name: String
}

struct Person1<'v> {
    name: &'v str
}

impl<'v> Person1<'v> {

    fn talk(&self) {
        println!("My name is {}", &self.name)
    }
}

impl Person {
    fn get_ref_name(&self) -> &String {
        &self.name
    }
}

struct Company<'z> {
    name: String,
    ceo: &'z Person
}

fn lifetime() {
    // let boss = Person {name: String::from("Elon Musk")};
    // let tesla = Company {name: String::from("Tesla"), ceo: &boss};

    let mut z: &String;
    let p = Person {name: String::from("John")};
    z = p.get_ref_name();
    println!("z = {}", z);

    let p1 = Person1 {name: "John Doe"};
    p1.talk();
}

pub fn results() {
    ownership();
    borrowing();
    lifetime();
}