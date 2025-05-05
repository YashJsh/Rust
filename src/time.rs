use chrono::Local;

pub fn time(){
    let now = Local::now();
    println!("{}", now);
}