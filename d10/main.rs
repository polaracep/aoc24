use array2d::Array2D;
use curl::easy::Easy;
use curl::easy::List;
use std::char;
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

static mut POSITIONS: Vec<(usize, usize)> = vec![];

fn main() {
    let mut input = get_input(10);
    let size = input.lines().next().unwrap().len();
    let mut sum1 = 0;
    let mut sum2 = 0;

    println!("{}", input);

    input = input.chars().into_iter().filter(|x| *x != '\n').collect();
    let grid = Array2D::from_iter_row_major(input.chars(), size, size).unwrap();

    for r in 0..grid.row_len() {
        for c in 0..grid.column_len() {
            if grid[(r, c)] == '0' {
                let score1 = check_neighbors(&grid, (r, c), 1);
                // println!("On r: {}, c: {} w: {}", r, c, score1);
                let score2 = check_neighbors(&grid, (r, c), 2);
                sum1 += score1;
                sum2 += score2;
            }
            unsafe {
                POSITIONS.clear();
            }
        }
    }
    println!("1: {}, 2: {}", sum1, sum2);

    process::exit(0);
}

fn check_neighbors(grid: &Array2D<char>, pos: (usize, usize), part: usize) -> u32 {
    let num = grid[pos];

    if num == '9' {
        if part == 1 {
            unsafe {
                if !POSITIONS.contains(&pos) {
                    POSITIONS.push(pos);
                    return 1;
                }
                return 0;
            }
        } else {
            return 1;
        }
    }

    let mut score = 0;

    // add 1 to the char
    let num = char::from_digit(num.to_digit(10).unwrap() + 1, 10).unwrap();

    // dolu
    if grid.get(pos.0 + 1, pos.1).unwrap_or(&'x') == &num {
        score += check_neighbors(grid, (pos.0 + 1, pos.1), part);
    }

    // nahoru
    if grid.get(pos.0.overflowing_sub(1).0, pos.1).unwrap_or(&'x') == &num {
        score += check_neighbors(grid, (pos.0 - 1, pos.1), part);
    }

    // doprava
    if grid.get(pos.0, pos.1 + 1).unwrap_or(&'x') == &num {
        score += check_neighbors(grid, (pos.0, pos.1 + 1), part);
    }

    // doleva
    if grid.get(pos.0, pos.1.overflowing_sub(1).0).unwrap_or(&'x') == &num {
        score += check_neighbors(grid, (pos.0, pos.1 - 1), part);
    }
    score
}
