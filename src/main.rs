mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn print_soln<T: std::fmt::Display, S: std::fmt::Display>(day: &str, solns: (T, S)) {
    println!("{day}.1: {}", solns.0);
    println!("{day}.2: {}", solns.1);
}

fn main() {
    print_soln("1", (day1::soln_1_1(), day1::soln_1_2()));
    print_soln("2", (day2::soln_2_1(), day2::soln_2_2()));
    print_soln("3", (day3::soln_3_1(), day3::soln_3_2()));
    print_soln("4", (day4::soln_4_1(), day4::soln_4_2()));
    print_soln("5", day5::soln());
    print_soln("6", day6::soln());
    print_soln("7", day7::soln());
    print_soln("8", day8::soln());
    print_soln("9", day9::soln());
    print_soln("10", day10::soln());
    print_soln("11", day11::soln());
    print_soln("12", day12::soln());
    print_soln("13", day13::soln());
    print_soln("14", day14::soln());
}
