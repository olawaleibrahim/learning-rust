use std::fmt::Debug;
use std::ops::{Add, AddAssign, Neg};

trait Animal {
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human{name: name}
    }

    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello", self.name())
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat{name: name}
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name())
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self {
            result += *x;
        }
        return result

    }
}

#[derive(Debug)]
struct Circle {
    radius: f64
}

#[derive(Debug)]
struct Square {
    side: f64
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// fn print_info(shape: impl Shape + Debug) {
// fn print_info<T: Shape + Debug>(shape: T) {
fn print_info<T>(shape: T) where T: Shape + Debug{
    println!("{:?}", shape);
    println!("The area is {}", shape.area())
}

struct Person {
    name: String
}

impl Person {
    // fn new(name: &str) -> Person {
    //     Person{name: name.to_string()}
    // }
    fn new<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }
}

struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name)
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}

impl<T> Add for Complex<T>
    where T: Add<Output = T> {
    type Output = Complex<T>;

    // a+b
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> AddAssign for Complex<T> 
    where T: AddAssign<T>{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im
    }

}

impl<T> Neg for Complex<T>
    where T: Neg<Output=T>{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }

}

impl<T> PartialEq for Complex<T>
    where T: PartialEq {
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

pub fn traits() {
    // let h = Human{name:"John"};
    let h = Human::create("John");
    h.talk();
    let h1: Human = Animal::create("John Samuel");
    h1.talk();
    let c = Cat{name:"Misty"};
    // c.talk();
    //
    let a = vec![1,2,3];
    println!("sum = {}", a.sum());

    let c = Circle{radius: 3.0};
    print_info(c);

    let john = Person::new("Olawale");
    let name = "Jane".to_string();
    let jane = Person::new(name/*.as_ref()*/);

    let mut clever: Creature;
    {
        let goblin = Creature::new("Jeff");
        println!("game proceeds");
        clever = goblin;
        println!("end of scope");
    }

    let mut a = Complex::new(1.0, 2.0);
    let mut b = Complex::new(0.3, 4.0);
    // println!("{:?}", a+b);

    //a+=b;
    //println!("{:?}", -a);
    println!("{:?}", b==a);
}
