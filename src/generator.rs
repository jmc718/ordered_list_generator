/*
For reference material only

*/


use std::cmp::Ordering;
use std::io::{Read, Write};

fn main() {
    let strings = std::fs::read_to_string("src/List.txt").unwrap();
    let mut favs = strings.lines().collect::<Vec<_>>();
    println!("{:#?}", favs);

    // sort things
    favs.sort_unstable_by(|left, right| loop {
        let mut input = String::new();
        print!("Do you like {} more than {}? (y/n)", left, right);
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.chars().next().unwrap() {
            'y' => break Ordering::Less,
            'n' => break Ordering::Greater,
            _ => println!("Please enter y or n"),
        }
    });
    println!("{:#?}", favs);
}
