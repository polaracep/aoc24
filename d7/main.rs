use curl::easy::Easy;
use curl::easy::List;
use std::fs;
use std::process;

fn get_input(day: i32) -> String {
    let input: String;
    let mut curl = Easy::new();

    let file_name = format!("./input{}.txt", day);
    let token_name = format!("../key");
    let token: String;

    token = match fs::read_to_string(&token_name) {
        Ok(s) => s.trim().to_string(),
        Err(_) => {
            println!("No token!");
            process::exit(1);
        }
    };

    input = match fs::read_to_string(&file_name) {
        Ok(s) => s,
        Err(_) => {
            let mut request_str = String::new();
            {
                println!("Downloading input...");

                let mut headers = List::new();

                headers
                    .append(format!("Cookie: session={}", token).as_str())
                    .unwrap();
                curl.http_headers(headers).unwrap();

                curl.url(format!("https://adventofcode.com/2024/day/{}/input", day).as_str())
                    .unwrap();
                let mut transfer = curl.transfer();

                transfer
                    .write_function(|data| {
                        request_str.push_str(std::str::from_utf8(data).unwrap());
                        Ok(data.len())
                    })
                    .unwrap();
                transfer.perform().unwrap();
            }

            let request_str = &request_str;

            fs::write(file_name, request_str).unwrap();

            request_str.to_string()
        }
    };
    return input;
}

fn main() {
    let input = get_input(7);

    println!("p1: {}", part1(&input));
    println!("p2: {}", part2(&input));

    process::exit(0);
}

fn part2(input: &String) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let mut line: Vec<i64> = line
            .split([' ', ':'])
            .filter_map(|x| x.parse::<i64>().ok())
            .rev()
            .collect();
        let res: i64 = line.pop().unwrap();
        let argh = next_number_2(&line, 0);
        if argh.contains(&res) {
            sum += res;
        }
    }
    sum
}

fn next_number_2(numbers: &Vec<i64>, index: usize) -> Vec<i64> {
    if index == numbers.len() - 1 {
        return vec![numbers[index]];
    }
    let this_n = numbers[index];
    // println!("this: {this_n}");

    let mut computations: Vec<i64> = vec![];

    next_number_2(numbers, index + 1)
        .into_iter()
        .for_each(|n| computations.push(n + this_n));

    next_number_2(numbers, index + 1)
        .into_iter()
        .for_each(|n| computations.push(n * this_n));

    // concatenation
    next_number_2(numbers, index + 1).into_iter().for_each(|n| {
        computations.push(
            (n.to_string() + &this_n.to_string())
                .parse::<i64>()
                .unwrap(),
        )
    });

    computations
}

fn part1(input: &String) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let mut line: Vec<i64> = line
            .split([' ', ':'])
            .filter_map(|x| x.parse::<i64>().ok())
            .rev()
            .collect();
        let res: i64 = line.pop().unwrap();

        let argh = next_number_1(&line, 0);
        if argh.contains(&res) {
            sum += res;
        }
    }
    sum
}

fn next_number_1(numbers: &Vec<i64>, index: usize) -> Vec<i64> {
    if index == numbers.len() - 1 {
        return vec![numbers[index]];
    }
    let this_n = numbers[index];
    // println!("this: {this_n}");

    let mut computations: Vec<i64> = vec![];

    next_number_1(numbers, index + 1)
        .into_iter()
        .for_each(|n| computations.push(n + this_n));

    next_number_1(numbers, index + 1)
        .into_iter()
        .for_each(|n| computations.push(n * this_n));

    computations
}
