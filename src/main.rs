fn main() {
    let input = include_str!("../puzzle-input.txt");
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
