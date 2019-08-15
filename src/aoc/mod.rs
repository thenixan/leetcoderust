mod y2015;

pub fn resolve(input: &str) {
    match input {
        "2015-1" => y2015::task_1::run(),
        "2015-1e" => y2015::task_1::run_e(),
        "2015-2" => y2015::task_2::run(),
        "2015-2e" => y2015::task_2::run_e(),
        _ => println!("Unresolved task")
    }
}