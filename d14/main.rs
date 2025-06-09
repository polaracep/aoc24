use array2d::Array2D;
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
    // 
    return input;
}

struct Robot {
    pos: (usize, usize),
    vel: (i32, i32),
}

impl std::fmt::Debug for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\npos: ({}, {}), vel: ({}, {})",
            self.pos.0, self.pos.1, self.vel.0, self.vel.1
        )
    }
}

static BOUND_X: usize = 11;
static BOUND_Y: usize = 7;

fn main() {
    let input = get_input(14);

    let mut robots: Vec<Robot> = vec![];

    let sec = 2;
    for l in input.lines() {
        let parsed: Vec<&str> = l.split([',', '=', ' ']).collect();
        println!("{parsed:?}");
        // y,x
        robots.push(Robot {
            pos: (parsed[1].parse().unwrap(), parsed[2].parse().unwrap()),
            vel: (parsed[4].parse().unwrap(), parsed[5].parse().unwrap()),
        });
    }

    println!("{:?}", robots);
    print_robots(&robots);
    println!();
    for r in &mut robots {
        let mut new_pos: (i32, i32) = (
            r.pos.0 as i32 + (sec * r.vel.0),
            r.pos.1 as i32 + (sec * r.vel.1),
        );

        let rest = new_pos.0 % BOUND_X as i32;
        println!("rest: {rest}");
        if rest < 0 {
            new_pos.0 = BOUND_X as i32 + rest;
            if new_pos.0 < 0 {
                panic!();
            }
        } else {
            new_pos.0 = rest;
        }

        let rest = new_pos.1 % BOUND_Y as i32;
        if rest < 0 {
            new_pos.1 = BOUND_Y as i32 + rest;
            if new_pos.1 < 0 {
                panic!();
            }
        } else {
            new_pos.1 = rest;
        }
        r.pos = (new_pos.0 as usize, new_pos.1 as usize);
    }
    println!("{:?}", robots);
    print_robots(&robots);
    process::exit(0);
}

fn print_robots(robots: &Vec<Robot>) {
    let mut grid: Array2D<i32> = Array2D::filled_with(0, BOUND_Y, BOUND_X);
    // let mut grid: Vec<Vec<i32>> = vec![];
    for r in robots {
        // grid[(r.pos.1, r.pos.0)] += 1;
        match grid.get(r.pos.0, r.pos.1) {
            Some(x) => grid[(r.pos.0, r.pos.1)] = x + 1,
            None => (),
            //None => grid[(r.pos.0, r.pos.1)] = 1,
        }
    }

    for l in grid.rows_iter() {
        println!(
            "{}",
            l.into_iter()
                .map(|x| {
                    if *x == 0 {
                        ".".to_string()
                    } else {
                        x.to_string()
                    }
                })
                .collect::<String>()
        );
    }
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
