mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

fn print_soln<T: std::fmt::Display, S: std::fmt::Display>(day: &str, solns: (T, S)) {
    println!("{day}.1: {}", solns.0);
    println!("{day}.2: {}", solns.1);
}

fn main() {
    print_soln("01", (day01::soln_1_1(), day01::soln_1_2()));
    print_soln("02", (day02::soln_2_1(), day02::soln_2_2()));
    print_soln("03", (day03::soln_3_1(), day03::soln_3_2()));
    print_soln("04", (day04::soln_4_1(), day04::soln_4_2()));
    print_soln("05", day05::soln());
    print_soln("06", day06::soln());
    print_soln("07", day07::soln());
    print_soln("08", day08::soln());
    print_soln("09", day09::soln());
    print_soln("10", day10::soln());
    print_soln("11", day11::soln());
    print_soln("12", day12::soln());
    print_soln("13", day13::soln());
    print_soln("14", day14::soln());
    print_soln("15", day15::soln());
    print_soln("16", day16::soln());
    print_soln("17", day17::soln());
    print_soln("18", day18::soln());
    print_soln("19", day19::soln());
}
