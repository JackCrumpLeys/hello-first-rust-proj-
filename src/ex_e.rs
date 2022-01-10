use std::{thread, time};

pub(crate) fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
       println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)
}

fn inspect(str: &str) -> bool {
    if str.ends_with("s") {
        println!("word: {} is plural!", str);
        true
    } else {
        println!("word: {} is not a plural!", str);
        false
    }
}

fn change(str: &mut String) {
    if !inspect(str) {
        str.push_str("s");
    }
}

fn eat(str: String) -> bool {
    println!("eating... ");
    let time = time::Duration::from_secs_f32(0.3f32);
    thread::sleep(time);
    if str.starts_with("b") && str.contains("a") {return true;}
    false
}