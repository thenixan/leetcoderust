pub mod y2015;

pub fn resolve(input: &str) {
    match input {
        "2015-1" => y2015::task_1::run(),
        "2015-1e" => y2015::task_1::run_e(),
        "2015-2" => y2015::task_2::run(),
        "2015-2e" => y2015::task_2::run_e(),
        "2015-3" => y2015::task_3::run(),
        "2015-3e" => y2015::task_3::run_e(),
        "2015-4" => y2015::task_4::run(),
        "2015-4e" => y2015::task_4::run_e(),
        "2015-5" => y2015::task_5::run(),
        "2015-5e" => y2015::task_5::run_e(),
        "2015-6" => y2015::task_6::run(),
        "2015-6e" => y2015::task_6::run_e(),
        "2015-7" => y2015::task_7::run(),
        "2015-7e" => y2015::task_7::run_e(),
        "2015-8" => y2015::task_8::run(),
        "2015-8e" => y2015::task_8::run_e(),
        "2015-9" => y2015::task_9::run(),
        "2015-9e" => y2015::task_9::run_e(),
        "2015-10" => y2015::task_10::run(),
        "2015-10e" => y2015::task_10::run_e(),
        "2015-11" => y2015::task_11::run(),
        "2015-11e" => y2015::task_11::run_e(),
        "2015-12" => y2015::task_12::run(),
        "2015-12e" => y2015::task_12::run_e(),
        "2015-13" => y2015::task_13::run(),
        "2015-13e" => y2015::task_13::run_e(),
        _ => println!("Unresolved task")
    }
}