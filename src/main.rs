mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    println!("1.1: {}", day1::soln_1_1());
    println!("1.2: {}", day1::soln_1_2());
    println!("2.1: {}", day2::soln_2_1());
    println!("2.2: {}", day2::soln_2_2());
    println!("3.1: {}", day3::soln_3_1());
    println!("3.2: {}", day3::soln_3_2());
    println!("4.1: {}", day4::soln_4_1());
    println!("4.2: {}", day4::soln_4_2());

    let (day5_1, day5_2) = day5::soln();
    println!("5.1: {day5_1}");
    println!("5.2: {day5_2}");
}
