use std::error::Error;
use std::fs;

fn main() {
    let result = read_file("src/day1.txt");
    let converter = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];

    let file_content: String = match result {
        Ok(content) => content,
        Err(_err) => {
            eprintln!("Something bad happened");
            return;
        }
    };

    let lines: Vec<&str> = file_content.split("\n").collect();
    println!("{:?}", lines);

    let mut nums: Vec<String> = vec![];

    for line in lines {
        let mut value: String = String::new();
        for (i, s) in line.char_indices() {
            let char_to_string = s.to_string();
            if is_num(&char_to_string) {
                value = value + &char_to_string;
            }

            for (idx, val) in converter.iter().enumerate() {
                let checker = &line[i..];

                if checker.starts_with(val) {
                    value = value + &(idx + 1).to_string();
                }
            }
        }

        nums.push(value);
    }

    println!("{:?}", nums);

    let mut first_and_last: Vec<String> = vec![];
    for num in nums.iter() {
        let reverse: String = num.chars().rev().collect::<String>();

        let mut first_num_unwrap: String = String::new();
        let mut last_num_unwrap: String = String::new();
        let first_num = num.chars().nth(0);
        match first_num {
            Some(text) => {
                first_num_unwrap.push(text);
            }
            None => {
                eprintln!("Something bad happened at first word");
            }
        };
        let last_num = reverse.chars().nth(0);
        match last_num {
            Some(text) => {
                last_num_unwrap.push(text);
            }
            None => {
                eprintln!("Something bad happened");
            }
        };

        let final_num = first_num_unwrap + &last_num_unwrap;
        first_and_last.push(final_num);
    }

    println!("{:?}", first_and_last);

    let mut final_sum: Vec<u128> = vec![];
    for to_sum in first_and_last.iter() {
        let answer = to_sum.parse::<u128>();
        match answer {
            Ok(v) => {
                final_sum.push(v);
            }
            Err(_er) => println!("Something happened while parsing"),
        }
    }

    println!("{:?}", final_sum);
    let final_answer: u128 = final_sum.iter().sum();
    println!("{}", final_answer);
}

fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let lines: String = fs::read_to_string(path)?;

    Ok(lines)
}

fn is_num(base: &str) -> bool {
    let result = base.parse::<f64>().map(|_| true).map_err(|_| false);

    match result {
        Ok(v) => v,
        Err(r) => r,
    }
}
