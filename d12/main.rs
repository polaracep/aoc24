#![allow(dead_code)]
use array2d::Array2D;
use curl::easy::Easy;
use curl::easy::List;
use std::collections::HashSet;
use std::fs;
use std::process;

#[derive(Clone, Copy)]
enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy)]
struct Edge {
    pos_e: (usize, usize),
    dir_e: DIRECTION,
}

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

static mut EDGES: i32 = 0;
static mut AREA: i32 = 0;
static mut EDGE_LIST: Vec<Edge> = vec![];

fn main() {
    let input = get_input(12);

    let size = input.clone().lines().next().unwrap().len();
    let grid: String = input.chars().into_iter().filter(|x| *x != '\n').collect();
    let mut grid = Array2D::from_iter_row_major(grid.chars(), size, size).unwrap();
    let mut sum_1: i32 = 0;
    let mut sum_2: i32 = 0;

    print_grid(&grid);
    println!("==");
    let mut used_chars: Vec<char> = vec![];
    for i in 0..grid.row_len() {
        for j in 0..grid.column_len() {
            if grid[(i, j)].is_ascii_lowercase() {
                continue;
            }
            // nove pismenko
            let searched = grid[(i, j)];
            used_chars.push(searched);
            // print_grid(&grid);

            search_next(&mut grid, (i, j), searched, DIRECTION::UP);
            unsafe {
                println!("(1) S: {}, A: {}, E: {}", searched, AREA, EDGES);
                sum_1 += AREA * EDGES;
                EDGES = 0;
                let mut v_up: Vec<(usize, usize)> = vec![];
                let mut v_down: Vec<(usize, usize)> = vec![];
                let mut v_left: Vec<(usize, usize)> = vec![];
                let mut v_right: Vec<(usize, usize)> = vec![];

                for e in EDGE_LIST.clone() {
                    match e.dir_e {
                        DIRECTION::UP => v_up.push(e.pos_e),
                        DIRECTION::DOWN => v_down.push(e.pos_e),
                        DIRECTION::LEFT => v_left.push(e.pos_e),
                        DIRECTION::RIGHT => v_right.push(e.pos_e),
                    }
                }
                EDGE_LIST.clear();

                for mut v in vec![v_up, v_down] {
                    v.sort_unstable();
                    v.iter().for_each(|x| print!("{:?}", x));
                    let mut count: HashSet<(usize, usize)> = HashSet::new();
                    let mut last = v[0];
                    for ele in v {
                        if ele.0 == last.0 && i32::abs(last.1 as i32 - ele.1 as i32) == 1 {
                            last = ele;
                            continue;
                        }
                        count.insert(ele);
                        last = ele;
                    }
                    EDGES += count.len() as i32;

                    println!("U/D total: {}", count.len());
                }

                for mut v in vec![v_left, v_right] {
                    v.sort_unstable_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0)));
                    let mut count: HashSet<(usize, usize)> = HashSet::new();
                    let mut last = v[0];
                    for ele in v {
                        if ele.1 == last.1 && i32::abs(last.0 as i32 - ele.0 as i32) == 1 {
                            last = ele;
                            continue;
                        }
                        count.insert(ele);
                        last = ele;
                    }
                    EDGES += count.len() as i32;

                    println!("L/R total: {}", count.len());
                }

                sum_2 += EDGES * AREA;
                println!("(2) S: {}, A: {}, E: {}", searched, AREA, EDGES);
                AREA = 0;
            }
            println!();
        }
    }

    println!("1: {}, 2: {}", sum_1, sum_2);
    process::exit(0);
}
fn search_next(grid: &mut Array2D<char>, pos: (usize, usize), el: char, dir_from: DIRECTION) {
    let c = grid.get(pos.0, pos.1).unwrap_or(&' ');
    if c != &el {
        if c.to_ascii_uppercase() != el {
            unsafe {
                match dir_from {
                    DIRECTION::UP => EDGE_LIST.push(Edge {
                        pos_e: (pos.0 + 1, pos.1),
                        dir_e: dir_from,
                    }),
                    DIRECTION::DOWN => EDGE_LIST.push(Edge {
                        pos_e: (pos.0 - 1, pos.1),
                        dir_e: dir_from,
                    }),
                    DIRECTION::LEFT => EDGE_LIST.push(Edge {
                        pos_e: (pos.0, pos.1 + 1),
                        dir_e: dir_from,
                    }),
                    DIRECTION::RIGHT => EDGE_LIST.push(Edge {
                        pos_e: (pos.0, pos.1 - 1),
                        dir_e: dir_from,
                    }),
                }

                EDGES += 1;
            }
        }
        return;
    }
    unsafe {
        AREA += 1;
    }
    grid[pos] = el.to_ascii_lowercase();

    match pos.0.checked_sub(1) {
        Some(n) => search_next(grid, (n, pos.1), el, DIRECTION::UP),
        None => unsafe {
            EDGE_LIST.push(Edge {
                pos_e: pos,
                dir_e: DIRECTION::UP,
            });
            EDGES += 1;
        },
    }
    search_next(grid, (pos.0 + 1, pos.1), el, DIRECTION::DOWN);

    match pos.1.checked_sub(1) {
        Some(n) => search_next(grid, (pos.0, n), el, DIRECTION::LEFT),
        None => unsafe {
            EDGE_LIST.push(Edge {
                pos_e: pos,
                dir_e: DIRECTION::LEFT,
            });
            EDGES += 1;
        },
    }
    search_next(grid, (pos.0, pos.1 + 1), el, DIRECTION::RIGHT);
}

fn print_grid(grid: &Array2D<char>) {
    grid.rows_iter()
        .for_each(|r| println!("{}", r.collect::<String>()));
}
