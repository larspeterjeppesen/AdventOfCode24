use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;

fn part_one() {
    let mut file = File::open("src/input").expect("Could not read input file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Could not read input into string.");

    let mut first = vec![];
    let mut second = vec![];

    let _ = input.lines().for_each(|l| {
        let mut s = l.split("   ");
        first.push(s.next());
        second.push(s.next());
    });
    first.sort();
    second.sort();

    let n = zip(first, second).fold(0, |acc, (x, y)| {
        let x = (x.unwrap().parse::<i32>().unwrap() - y.unwrap().parse::<i32>().unwrap()).abs();
        println!("{}", x);
        acc + x
    });
    println!("{}", n);
}

fn part_two() {
    let mut file = File::open("src/input").expect("Could not read input file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Could not read input into string.");

    let mut first = vec![];
    let mut second = vec![];

    let _ = input.lines().for_each(|l| {
        let mut s = l.split("   ");
        first.push(s.next().unwrap().parse::<i32>().unwrap());
        second.push(s.next().unwrap().parse::<i32>().unwrap());
    });

    let n = first.iter().fold(0i32, |acc, x| {
        acc + x * second.iter().filter(|y| *y == x).count() as i32
    });

    println!("{}", n);
}

fn main() {
    part_one();
    part_two();
}
