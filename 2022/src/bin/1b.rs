use AoC::input::read_stdin;
fn main() {
    let elves = read_stdin();
    let parsed: Vec<Vec<u32>> = elves
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|line| line.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut calories: Vec<u32> = parsed.iter().map(|e| e.iter().sum()).collect();
    calories.sort();
    calories.reverse();
    let top_three: u32 = calories.iter().take(3).sum();
    println!("{}", top_three);
}