use std::env;

mod problems;
mod aoc;

fn main() {
    let args: Vec<String> = env::args().collect();
    let aoc_code = args.get(2);
    match args.get(1) {
        Some(x) => {
            println!("Task: {}", x);
            match x.as_str() {
                "aoc" => aoc::resolve(aoc_code.unwrap()),
                "4" => problems::problem_4(),
                "5" => problems::problem_5(),
                "6" => problems::problem_6(),
                "7" => problems::problem_7(),
                "8" => problems::problem_8(),
                "9" => problems::problem_9(),
                "11" => problems::problem_11(),
                "13" => problems::problem_13(),
                "70" => problems::problem_70(),
                "226" => problems::problem_226(),
                "337" => problems::problem_337(),
                "561" => problems::problem_561(),
                "584" => problems::problem_584(),
                "838" => problems::problem_838(),
                "1052" => problems::problem_1052(),
                _ => println!("Unknown task"),
            };
        }
        None => println!("Provide task"),
    }
}
