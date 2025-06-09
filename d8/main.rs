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
    let input = get_input(8);

    println!("{input}");

    println!("P1: {}", part1(&input));
    println!("P2: {}", part2(&input));
    process::exit(0);
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        println!("{:?}", line.iter().collect::<String>());
    }
}

fn part1(input: &String) -> i32 {
    let mut sum = 0;

    let mut frequencies: Vec<char> = vec![];
    input.chars().into_iter().for_each(|x| {
        if !frequencies.contains(&x) && x != '.' && x != '\n' {
            frequencies.push(x);
        }
    });

    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut positions: Vec<Vec<(usize, usize)>> = vec![];
    println!("{:?}", frequencies);

    for (a, freq) in frequencies.iter().enumerate() {
        positions.push(Vec::new());
        for (i, line) in grid.iter().enumerate() {
            for (j, _row) in line.iter().enumerate() {
                if grid[i][j] == *freq {
                    positions[a].push((i, j));
                }
            }
        }
    }

    for antennas in positions {
        for pos in antennas.clone() {
            for other_pos in antennas.clone() {
                if pos == other_pos {
                    continue;
                }
                let transform = (
                    other_pos.0.overflowing_sub(pos.0).0,
                    other_pos.1.overflowing_sub(pos.1).0,
                );
                let antinode: (usize, usize) = (
                    pos.0.overflowing_sub(transform.0).0,
                    pos.1.overflowing_sub(transform.1).0,
                );

                if antinode.0 > grid.len() - 1 || antinode.1 > grid[0].len() - 1 {
                    continue;
                }
                if grid[antinode.0][antinode.1] != '#' {
                    grid[antinode.0][antinode.1] = '#';
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn part2(input: &String) -> i32 {
    let mut sum = 0;

    let mut frequencies: Vec<char> = vec![];
    input.chars().into_iter().for_each(|x| {
        if !frequencies.contains(&x) && x != '.' && x != '\n' {
            frequencies.push(x);
        }
    });

    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut positions: Vec<Vec<(usize, usize)>> = vec![];
    println!("{:?}", frequencies);

    for (a, freq) in frequencies.iter().enumerate() {
        positions.push(Vec::new());
        for (i, line) in grid.iter().enumerate() {
            for (j, _row) in line.iter().enumerate() {
                if grid[i][j] == *freq {
                    positions[a].push((i, j));
                }
            }
        }
    }

    for antennas in positions {
        for pos in antennas.clone() {
            for other_pos in antennas.clone() {
                if pos == other_pos {
                    continue;
                }
                let transform = (
                    other_pos.0.overflowing_sub(pos.0).0,
                    other_pos.1.overflowing_sub(pos.1).0,
                );
                let mut i: usize = 0;
                loop {
                    let antinode: (usize, usize) = (
                        pos.0.overflowing_sub(i.overflowing_mul(transform.0).0).0,
                        pos.1.overflowing_sub(i.overflowing_mul(transform.1).0).0,
                    );

                    if antinode.0 > grid.len() - 1 || antinode.1 > grid[0].len() - 1 {
                        break;
                    }
                    if grid[antinode.0][antinode.1] != '#' {
                        grid[antinode.0][antinode.1] = '#';
                        sum += 1;
                    }
                    i += 1;
                }
            }
        }
    }

    sum
}
