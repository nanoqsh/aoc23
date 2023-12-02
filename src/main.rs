#[allow(dead_code)]
mod day1;
mod day2;

fn main() {
    // let p1 = day1::p1(include_str!("d1"));
    // let p2 = day1::p2(include_str!("d1"));
    let p1 = day2::p1(include_str!("d2"));
    let p2 = day2::p2(include_str!("d2"));
    println!("{p1} {p2}");
}
