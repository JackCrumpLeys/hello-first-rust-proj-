// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

use unicode_segmentation::UnicodeSegmentation;

pub(crate) fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if arg.to_lowercase().starts_with("sum") {
            sum()
        } else if arg.to_lowercase().starts_with("double"){
            double()
        } else {
            count(arg)
        }

    }
}

fn sum() {
    let mut sum = 0;
    for i in 7..=23 {
        sum += i;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    while x < 500 {
        x *= 2;
        count += 1;
    }

    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {
    let mut count = 0;
    for char in arg.graphemes(true).into_iter() {
        count+=1
    }

    println!("letters in {}: {}", arg, count);
}