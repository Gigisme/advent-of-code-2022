use std::fs::{File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let file = Path::new("first/input.txt");
    let lines = match read_lines(file) {
        Ok(lines) => lines,
        Err(e) => panic!("Problem opening file: {}", e)
    };
    let elves = store_lines(lines);

    let mut top3: [i32; 3] = [0; 3];
    for elf in elves {
        let calories = elf.iter().sum();
        if calories > top3[0] {
            top3[2] = top3[1];
            top3[1] = top3[0];
            top3[0] = calories
        }
        else if calories > top3[1] {
            top3[2] = top3[1];
            top3[1] = calories;
        }
        else if calories > top3[2] {
            top3[2] = calories;
        }
    }
    let sum:i32 = top3.iter().sum();
    println!("{:?}", sum)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn store_lines(lines: io::Lines<BufReader<File>>) -> Vec<Vec<i32>>
{
    let mut outer: Vec<Vec<i32>> = Vec::new();
    let mut inner: Vec<i32> = Vec::new();
    for line in lines {
        match line {
            Ok(string) => {
                if let Ok(value) = string.parse::<i32>() {
                    inner.push(value);
                }
                else {
                    outer.push(inner.clone());
                    inner.clear();
                }
            },
            Err(e) => panic!("Problem reading: {}", e)
        };
    }
    outer
}