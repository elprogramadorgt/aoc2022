use std::fs::File;
use std::io::Lines;
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let mut sum_calories = 0;

    let mut max_calories = 0;

    if let Ok(lines) = read_lines("./bitacora.txt") {
        for line in lines {
            if line.is_ok() {
                let calories = line.unwrap().parse::<i32>();

                if calories.is_ok() {
                    sum_calories += calories.clone().unwrap();
                } else {
                    if sum_calories > max_calories {
                        max_calories = sum_calories;
                    }

                    sum_calories = 0;
                }
            }
        }
    }

    println!("{max_calories}");
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file: File = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
