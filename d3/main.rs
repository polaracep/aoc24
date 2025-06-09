use curl::easy::Easy;
use curl::easy::List;
use regex::Regex;
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
    let input = get_input(3);

    println!("U1: {}", part1(&input));
    println!("U2: {}", part2(&input));

    process::exit(0);
}

fn part1(input: &String) -> i32 {
    let mut res: i32 = 0;

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    for cap in re.captures_iter(input.as_str()) {
        let op = cap.get(0).unwrap().as_str();

        let mut n1 = String::new();
        let mut n2 = String::new();
        let mut citer = op.chars().skip(4).peekable();

        while *citer.peek().unwrap() != ',' {
            n1 += citer.next().unwrap().to_string().as_str();
        }
        citer.next();

        while citer.peek().unwrap().is_ascii_digit() {
            n2 += citer.next().unwrap().to_string().as_str();
        }
        res += n2.parse::<i32>().unwrap() * n1.parse::<i32>().unwrap();
    }
    res
}

fn part2(input: &String) -> i32 {
    let mut res: i32 = 0;

    let mut visible = true;

    let re = Regex::new(r"don't|do|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for cap in re.captures_iter(input.as_str()) {
        let op = cap.get(0).unwrap().as_str();
        if op == "don't" {
            visible = false;
            continue;
        } else if op == "do" {
            visible = true;
            continue;
        }

        if visible == true {
            let i1 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let i2 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
            res += i1 * i2;
        }
    }

    res
}
