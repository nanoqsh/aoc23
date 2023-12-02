mod day1;

fn main() {
    let p1 = day1::p1(include_str!("d1"));
    let p2 = day1::p2(include_str!("d1"));
    println!("{p1} {p2}");
}
