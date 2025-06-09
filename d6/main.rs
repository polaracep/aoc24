use curl::easy::Easy;
use curl::easy::List;
use std::fs;
use std::process;
use std::usize;

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

#[derive(Clone)]
struct Guard {
    y: usize,
    x: usize,
    dirx: i32,
    diry: i32,
}

fn main() {
    let input = get_input(6);

    println!("{input}");

    let mut grid: Vec<Vec<char>> = vec![];
    input.lines().for_each(|l| grid.push(l.chars().collect()));

    let mut guard = Guard {
        x: 0,
        y: 0,
        dirx: 0,
        diry: -1,
    };

    for (i, row) in grid.iter().enumerate() {
        match row.into_iter().position(|x| *x == '^') {
            Some(a) => {
                guard.x = a;
                guard.y = i;
            }
            None => continue,
        };
    }

    println!("p1: {}", part1(&guard, &grid));
    println!("p1: {}", part2(&guard, &grid));

    process::exit(0);
}

fn eval_board(mut guard: Guard, mut grid: Vec<Vec<char>>) -> Option<Vec<Vec<char>>> {
    // (guard cords),(guard facing)
    let mut visited: Vec<((usize, usize), (i32, i32))> = vec![];

    loop {
        grid[guard.y][guard.x] = '+';

        let nexty = (guard.y as i32 + guard.diry) as usize;
        let nextx = (guard.x as i32 + guard.dirx) as usize;

        match grid.get(nexty) {
            Some(row) => match row.get(nextx) {
                Some('#' | 'O') => {
                    if visited.contains(&((guard.y, guard.x), (guard.diry, guard.dirx))) {
                        // opakujeme se!
                        return None;
                    }

                    visited.push(((guard.y, guard.x), (guard.diry, guard.dirx)));
                    let cached: (i32, i32) = (guard.diry, guard.dirx);

                    guard.dirx = -1 * cached.0;
                    guard.diry = cached.1;
                }
                None => return Some(grid),
                _ => {
                    guard.y = nexty;
                    guard.x = nextx;
                }
            },
            None => return Some(grid),
        }

        grid[guard.y][guard.x] = '^';
    }
}

fn part1(guard: &Guard, grid: &Vec<Vec<char>>) -> i32 {
    let solved = eval_board(guard.clone(), grid.clone()).unwrap();
    let mut score = 0;

    solved.iter().for_each(|r| {
        r.iter().for_each(|c| {
            if *c == '+' {
                score += 1;
            }
        });
    });
    score
}

fn part2(guard: &Guard, grid: &Vec<Vec<char>>) -> i32 {
    let solved = eval_board(guard.clone(), grid.clone()).unwrap();

    let mut ob_pos: Vec<(usize, usize)> = vec![];

    for (y, row) in solved.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '+' {
                ob_pos.push((y, x));
            }
        }
    }

    ob_pos.sort_unstable();
    ob_pos.dedup();

    let mut score = 0;
    for pos in ob_pos {
        let mut grid_test = solved.clone();
        grid_test[pos.0][pos.1] = 'O';

        if eval_board(guard.clone(), grid_test.clone()).is_none() {
            score += 1;
            // print_grid(&grid_test);
        }
    }
    score
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        println!("{:?}", line.iter().collect::<String>());
    }
}
