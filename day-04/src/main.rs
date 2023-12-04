fn main() {
    let input = include_str!("./input.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut lines = input.lines();
    let mut total = 0;

    while let Some(line) = lines.next() {
        let game = line.split(": ").collect::<Vec<_>>();
        let cards = game[1].split(" | ").collect::<Vec<_>>();
        let winning = cards[0].split_whitespace().collect::<Vec<_>>();
        let mine = cards[1].split_whitespace().collect::<Vec<_>>();

        let wins = mine.iter().filter(|card| winning.contains(card)).count();

        if wins > 0 {
            let base: i32 = 2;
            total += base.pow((wins - 1) as u32);
        }
        
    }

    println!("{}", total);
}

fn part2(input: &str) {
    let lines = input.lines();
    let mut copies = vec![1; lines.clone().count()];

    for (i, line) in lines.enumerate() {
        let game = line.split(": ").collect::<Vec<_>>();
        let cards = game[1].split(" | ").collect::<Vec<_>>();
        let winning = cards[0].split_whitespace().collect::<Vec<_>>();
        let mine = cards[1].split_whitespace().collect::<Vec<_>>();

        let wins = mine.iter().filter(|card| winning.contains(card)).count();

        for j in 0..wins {
            copies[i + j + 1] += copies[i];
        }
    }

    let total = copies.iter().sum::<i32>();
    println!("{}", total);
}
