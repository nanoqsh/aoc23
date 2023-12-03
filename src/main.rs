#[allow(dead_code)]
mod day1;
#[allow(dead_code)]
mod day2;
mod day3;

fn main() {
    // let p1 = day1::p1(include_str!("d1"));
    // let p2 = day1::p2(include_str!("d1"));
    // let p1 = day2::p1(include_str!("d2"));
    // let p2 = day2::p2(include_str!("d2"));

    let p1 = day3::p1(include_str!("d3"));
    let p2 = day3::p2(include_str!("d3"));
    println!("{p1} {p2}");
}
