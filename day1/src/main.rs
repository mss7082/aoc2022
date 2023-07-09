use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let path = "input.txt";
    println!("{:?}", get_input(path).unwrap().iter().max().unwrap()); // Part 1
    let mut sorted_vec = get_input(path).unwrap();

    sorted_vec.sort();
    sorted_vec.reverse();

    println!("{:?}", sorted_vec.iter().take(3).sum::<i32>()); // Part 2
}

fn get_input(path: &str) -> Result<Vec<i32>, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let result = buffered
        .lines()
        .map(|ls| ls.unwrap())
        .into_iter()
        .collect::<Vec<String>>();

    let temp = result
        .split(|x| x.is_empty())
        .map(|x| x.iter().fold(0, |acc, y| acc + y.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    Ok(temp)
}
