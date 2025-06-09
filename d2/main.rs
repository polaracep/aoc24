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
    let _input = get_input(2);

    let mut input = Vec::new();

    for line in fs::read_to_string("./input2.txt").unwrap().lines() {
        input.push(line.to_string())
    }

    let mut reports = Vec::<Vec<i32>>::new();

    for i in 0..input.len() {
        reports.push(Vec::new());
        input[i]
            .split(' ')
            .into_iter()
            .for_each(|x| reports[i].push(x.parse::<i32>().unwrap()))
    }

    let mut safe = 0;

    for r in reports.clone() {
        safe += get_valid(&r, 0);
    }

    println!("{}", safe);
    process::exit(0);
}

fn get_valid(r: &Vec<i32>, dampener: i32) -> i32 {
    let smer;

    if r[0] > r[1] {
        smer = 1; // klesajici
    } else {
        smer = -1; // rostouci
    }

    let mut i = 0;

    loop {
        let calc = r[i] - r[i + 1];

        // neco je spatne!
        if i32::abs(calc) > 3 || calc == 0 || (calc < 0 && smer > 0) || (calc > 0 && smer < 0) {
            if dampener == 0 {
                for j in 0..r.len() {
                    let mut cloned_report = r.clone();
                    cloned_report.remove(j);
                    if get_valid(&cloned_report, 1) == 1 {
                        println!("valid, {:?}", cloned_report);
                        return 1;
                    }
                    println!("no, {:?}", cloned_report);
                }
                return 0;
            } else {
                break;
            }
        }

        i += 1;
        if i == r.len() - 1 {
            println!("YES! {:?}; 1; {}", r, dampener);
            return 1;
        }
    }
    0
}

/*
    let mut safe = 0;

    for r in reports.clone() {
        let smer;

        if r[0] > r[1] {
            // klesajici
            smer = 1;
        } else {
            // rostouci
            smer = -1;
        }
        safe += 1;
        for i in 0..r.len() - 1 {
            let calc = r[i] - r[i + 1];

            if i32::abs(calc) > 3 || calc == 0 {
                print!("n, ");
                safe -= 1;
                break;
            } else if (calc < 0 && smer > 0) || (calc > 0 && smer < 0) {
                print!("n, ");
                safe -= 1;
                break;
            }
            print!("s, ");
        }
        println!("next");
    }
*/
