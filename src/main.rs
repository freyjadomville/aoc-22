use aoc_2022::aoc;

fn main() {
    let day = std::env::args().nth(1).expect("No day given");

    match day.as_str() {
        "1" => aoc::advent_one(),
        "2" => aoc::advent_two(),
        "3" => aoc::advent_three(),
        "4" => aoc::advent_four(),
        _ => println!("Day not implemented"),
    }
}
