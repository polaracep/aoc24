use curl::easy::Easy;
use curl::easy::List;
use std::any::type_name;
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

struct Game {
    a: (usize, usize),
    b: (usize, usize),
    dist: (usize, usize),
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a: {:?}, b: {:?}, dist: {:?}", self.a, self.b, self.dist)
    }
}

fn main() {
    let input = get_input(13);

    let str_games: Vec<String> = input.split("\n\n").map(|x| x.to_string()).collect();
    let mut sum = 0;

    // each game plays here
    for g in &str_games {
        let s: Vec<&str> = g.split(&['+', '=', ',', '\n']).collect();
        let game = Game {
            a: (s[1].parse().unwrap(), s[3].parse().unwrap()),
            b: (s[5].parse().unwrap(), s[7].parse().unwrap()),
            dist: (
                s[9].parse::<usize>().unwrap() + 10000000000000,
                s[11].parse::<usize>().unwrap() + 10000000000000,
            ),
        };

        let max_div = (game.dist.0 / game.b.0).max(game.dist.1 / game.b.1);
        println!("G: {game}, M: {max_div}");

        // compute b

        let b: f64 = (game.dist.1 as f64 * game.a.0 as f64 - game.dist.0 as f64 * game.a.1 as f64)
            / (game.a.0 as f64 * game.b.1 as f64 - game.b.0 as f64 * game.a.1 as f64);
        if b.floor() != b {
            continue;
        }
        let b = b as usize;
        let rest = game.dist.0.saturating_sub(b * game.b.0);
        let a = rest / game.a.0;

        if a.saturating_mul(game.a.0) == rest {
            if a.saturating_mul(game.a.1) + b.saturating_mul(game.b.1) == game.dist.1 {
                sum += (a * 3) + (b);
                println!("Done! A: {a}, B: {b}, g: {game}");
            }
        }
    }

    println!("Total: {sum}");
    process::exit(0);
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

/*
fn part1(input: &String) -> i32 {
    let mut sum = 0;

    sum
}

fn part2(input: &String) -> i32 {
    let mut sum = 0;

    sum
}
*/
