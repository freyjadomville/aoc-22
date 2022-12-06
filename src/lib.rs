pub mod aoc {
    pub fn advent_one() {
        let input = include_str!("../puzzle-input-01.txt");
        let mut elf_totals = input.lines().fold(vec![], |mut elves, new_value| {
            if elves.len() == 0 {
                elves.push(new_value.parse().expect("First value wasn't a u32"));
            } else if new_value == "" {
                elves.push(0)
            } else {
                let last = elves.last_mut().expect("no 'last element' to access");
                *last = *last + new_value.parse::<u32>().expect("Value wasn't a u32");
            }
            elves
        });
        // Descending order
        elf_totals.sort_by(|a, b| b.cmp(a));

        let part_1 = elf_totals.iter().max().expect("No result found");
        let part_2: u32 = elf_totals.iter().take(3).sum();

        println!("{}", part_1);
        println!("{}", part_2);
    }

    pub fn advent_two() {
        let input = include_str!("../puzzle-input-02.txt");

        let part_1 = input
            .lines()
            .map(|value| match value {
                "A X" => 1 + 3,
                "B X" => 1 + 0,
                "C X" => 1 + 6,
                "A Y" => 2 + 6,
                "B Y" => 2 + 3,
                "C Y" => 2 + 0,
                "A Z" => 3 + 0,
                "B Z" => 3 + 6,
                "C Z" => 3 + 3,
                _ => panic!("Unrecorgnised Value: {}", value),
            })
            .sum::<u64>();

        let part_2 = input
            .lines()
            .map(|value| match value {
                "A X" => 3 + 0,
                "B X" => 1 + 0,
                "C X" => 2 + 0,
                "A Y" => 1 + 3,
                "B Y" => 2 + 3,
                "C Y" => 3 + 3,
                "A Z" => 2 + 6,
                "B Z" => 3 + 6,
                "C Z" => 1 + 6,
                _ => panic!("Unrecorgnised Value: {}", value),
            })
            .sum::<u64>();

        println!("{}", part_1);
        println!("{}", part_2);
    }

    pub fn advent_three() {
        let input = include_str!("../puzzle-input-03.txt");

        let index = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

        let part_1: usize = input
            .lines()
            .map(|line| {
                // Safe due to ASCII input
                let first_half = &line[..line.len() / 2];
                let second_half = &line[line.len() / 2..];

                let common_char = first_half.chars().find(|c| second_half.contains(*c));

                if let Some(c) = common_char {
                    index
                        .chars()
                        .position(|p| p == c)
                        .expect("Found a non-alphabetical match")
                } else {
                    0 as usize
                }
            })
            .sum();

        println!("{}", part_1);
    }
}
