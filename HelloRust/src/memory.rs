use std::time;
use std::rc::Rc;
use std::thread;
use std::sync::Arc;

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
    name: Arc<String>
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

    fn new(name: Arc<String>) -> Person {
        Person {name: name}
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
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
    let p = Person {name: Arc::new(String::from("John"))};
    z = p.get_ref_name();
    println!("z = {}", z);

    let p1 = Person1 {name: "John Doe"};
    p1.talk();
}

fn rc_demo(){
    let name = Arc::new("Jon".to_string());
    println!("Name = {}, name has {} strong pointers", name, Arc::strong_count(&name));
    {
        let person = Person::new(name.clone());
        println!("Name = {}, name has {} strong pointers", name, Arc::strong_count(&name));

        person.greet();
    }
    println!("Name = {}, name has {} strong pointers", name, Arc::strong_count(&name));
    println!("Name = {}", name);
}

fn arc_demo() {
    let name = Arc::new("Johnny".to_string());
    let person = Person::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}", name);

    t.join().unwrap();
}

fn concurrency() {
    let handle = thread::spawn(|| {
        for _ in 1..10 {
            println!("+");
            thread::sleep(time::Duration::from_millis(500));
        }
    });
    for _ in 1..10 {
        println!("---");
        thread::sleep(time::Duration::from_millis(300));
    }

    handle.join();
}

pub fn results() {
    ownership();
    borrowing();
    lifetime();
    rc_demo();
    arc_demo();
    concurrency();
}