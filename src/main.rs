fn main() {
    let input = include_str!("../puzzle-input.txt");
    let mut elf_totals = input.lines().fold(vec![], |elves:Vec<u32>, new_value| {
        if elves.len() == 0 { vec![new_value.parse().expect("First value wasn't a u32")]}
        else if new_value == "" { [elves.as_slice(), [0].as_slice()].concat() }
        else { 
            let mut new_vec = elves.clone(); 
            let last = new_vec.last_mut().unwrap(); 
            *last = *last + new_value.parse::<u32>().expect("Value wasn't a u32");
            new_vec
        }
    });
    // Descending order
    elf_totals.sort_by(|a,b| b.cmp(a));

    let part_1 = elf_totals.iter().max().expect("No result found");
    let part_2: u32 = elf_totals.iter().take(3).sum();

    println!("{}", part_1);
    println!("{}", part_2);

}
