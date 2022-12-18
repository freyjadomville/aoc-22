pub mod aoc {

    use std::collections::HashSet;

    use itertools::Itertools;
    use regex::Regex;

    pub fn advent_one() {
        let input = include_str!("../puzzle-input-01.txt");
        let mut elf_totals = input.lines().fold(vec![], |mut elves, new_value| {
            if elves.is_empty() {
                elves.push(new_value.parse().expect("First value wasn't a u32"));
            } else if new_value.is_empty() {
                elves.push(0)
            } else {
                let last = elves.last_mut().expect("no 'last element' to access");
                *last += new_value.parse::<u32>().expect("Value wasn't a u32");
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
                "B X" => 1,
                "C X" => 1 + 6,
                "A Y" => 2 + 6,
                "B Y" => 2 + 3,
                "C Y" => 2,
                "A Z" => 3,
                "B Z" => 3 + 6,
                "C Z" => 3 + 3,
                _ => panic!("Unrecorgnised Value: {}", value),
            })
            .sum::<u64>();

        let part_2 = input
            .lines()
            .map(|value| match value {
                "A X" => 3,
                "B X" => 1,
                "C X" => 2,
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
                    0_usize
                }
            })
            .sum();

        let part_2: usize = input
            .lines()
            .batching(|it| {
                // if only let chains were a thing in stable...
                if let Some(l) = it.next() {
                    if let Some(m) = it.next() {
                        it.next().map(|n| (l, m, n))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .map(|(a, b, c)| {
                let common_char = a.chars().find(|l| b.contains(*l) && c.contains(*l));
                if let Some(c) = common_char {
                    index
                        .chars()
                        .position(|p| p == c)
                        .expect("Found a non alphabetical match")
                } else {
                    0_usize
                }
            })
            .sum();

        println!("{}", part_1);
        println!("{}", part_2);
    }

    pub fn advent_four() {
        let input = include_str!("../puzzle-input-04.txt");

        let set_pairs = input
            .lines()
            .map(|line| {
                let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").expect("regex is not valid");
                let captures = regex.captures(line).expect("No matches found for regex");
                let x1 = captures[1].parse::<u32>().expect("capture parse failed");
                let x2 = captures[2].parse::<u32>().expect("capture parse failed");

                let y1 = captures[3].parse::<u32>().expect("capture parse failed");
                let y2 = captures[4].parse::<u32>().expect("capture parse failed");

                [
                    (x1..=x2).collect::<HashSet<_>>(),
                    (y1..=y2).collect::<HashSet<_>>(),
                ]
            })
            .collect::<Vec<_>>();

        let part_1 = set_pairs
            .iter()
            .filter(|[x, y]| x.is_subset(y) || x.is_superset(y))
            .count();
        let part_2 = set_pairs.iter().filter(|[x, y]| !x.is_disjoint(y)).count();

        println!("{}", part_1);
        println!("{}", part_2);
    }
}
