use curl::easy::Easy;
use curl::easy::List;
use std::fmt::Write;
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
    let input = get_input(4);

    println!("Score 1: {}", part1(&input));
    println!("Score 2: {}", part2(&input));

    process::exit(0);
}

fn part1(input: &String) -> i32 {
    let mut score = 0;
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let grid: Vec<&str> = input.lines().collect();

    for (y, line) in grid.iter().enumerate() {
        for (x, col) in line.chars().enumerate() {
            if col == 'X' {
                for dir in directions.clone() {
                    let mut rest = String::new();
                    for i in 0..4 {
                        // println!("huhuohohohoh");
                        let index_y = y as i32 + (i as i32 * dir.1);
                        let index_x = x as i32 + (i as i32 * dir.0);
                        if index_y < 0 || index_x < 0 {
                            continue;
                        }
                        rest.write_char(
                            grid.get(index_y as usize)
                                .unwrap_or(&".")
                                .chars()
                                .nth(index_x as usize)
                                .unwrap_or_default(),
                        )
                        .unwrap();
                    }
                    if rest == "XMAS" {
                        score += 1;
                    }
                }
            }
        }
    }

    score
}

fn part2(input: &String) -> i32 {
    let mut score = 0;
    let directions: Vec<(i32, i32)> = vec![(-1, -1), (-1, 1)];

    let grid: Vec<&str> = input.lines().collect();
    let grid_len = grid.clone().into_iter().count() - 1;

    for (y, line) in grid.iter().enumerate().skip(1) {
        // nedelat posledni
        if y == grid_len {
            break;
        }
        'nextchar: for (x, col) in line.chars().enumerate().skip(1) {
            // nedelat posledni
            if x == line.chars().count() - 1 {
                break;
            }
            if col == 'A' {
                for dir in directions.clone() {
                    let diagonal_y = y as i32 + dir.1;
                    let diagonal_x = x as i32 + dir.0;

                    let c = grid
                        .get(diagonal_y as usize)
                        .unwrap_or(&".")
                        .chars()
                        .nth(diagonal_x as usize)
                        .unwrap_or_default();

                    let diagonal_y = y as i32 + (dir.1 * -1);
                    let diagonal_x = x as i32 + (dir.0 * -1);

                    let c_inverse = grid
                        .get(diagonal_y as usize)
                        .unwrap_or(&".")
                        .chars()
                        .nth(diagonal_x as usize)
                        .unwrap_or_default();

                    if !((c == 'M' && c_inverse == 'S') || (c == 'S' && c_inverse == 'M')) {
                        continue 'nextchar;
                    }
                }
                score += 1;
            }
        }
    }

    score
}
