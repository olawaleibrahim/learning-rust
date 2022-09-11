pub fn strings() {
    let s: &'static str = "hello there";
    for char in s.chars() {
        println!("{}", char);
    }
    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= 'z' as u8 {
        letters.push(a as char);
        letters.push_str(",");
        a += 1
    }
    println!("{}", letters);

    // &str <> string
    let u:&str = &letters;
    println!("u = {}", u);

    // concatenation
    let z = letters + &"letters";
    println!("z = {}", z);

    // string from str (slice)
    let new_string = String::from("New string here: Hello World!");
    println!("new_string = {}", new_string);

    // str slice to string
    let mut abc = "hello world".to_string();
    abc.remove(0);
    println!("string = {}", abc);
    abc.push_str("!!!");
    let goodbye = "goodbye";
    println!("string = {}", abc.replace("ello", goodbye));

    // string formatting
    let name = "Wale";
    let greeting = format!("hi, I'm {}, nice to meet you", name);
    println!("{}", greeting);

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}", hello, rust);
    println!("{}", hello_rust);

    let run = "run";
    let rwr = format!("{0}, {1} {0}!", run, name);
    println!("{}", rwr);

    let mixed = format!("{1} {} {0} {} {data}", "alpha", "beta", data="delta"); //beta alpha alpha beta delta
    println!("{}", mixed);
}