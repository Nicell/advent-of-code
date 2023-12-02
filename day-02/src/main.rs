use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut lines = input.lines().enumerate();
    let mut total = 0;

    let mut maxes = HashMap::new();
    maxes.insert("red", 12);
    maxes.insert("green", 13);
    maxes.insert("blue", 14);

    while let Some((i, line)) = lines.next() {
        let game_number = i + 1;
        let game: Vec<_> = line.split(": ").collect();
        let subsets: Vec<_> = game[1].split("; ").collect();

        let mut game_good = true;

        for subset in subsets.iter() {
            let cubes: Vec<_> = subset.split(", ").collect();

            for cube in cubes.iter() {
                let cube: Vec<_> = cube.split(" ").collect();
                let cube_number: i32 = cube[0].parse().unwrap();
                let cube_color = cube[1];

                if cube_number > maxes[cube_color] {
                    game_good = false;
                    break;
                }
            }
            if !game_good { break };
        }

        if game_good { total += game_number };
    }

    println!("{}", total);
}

fn part2(input: &str) {
    let mut lines = input.lines();
    let mut total = 0;

    while let Some(line) = lines.next() {
        let game: Vec<_> = line.split(": ").collect();
        let subsets: Vec<_> = game[1].split("; ").collect();

        let mut maxes = HashMap::new();
        maxes.insert("red", 0);
        maxes.insert("green", 0);
        maxes.insert("blue", 0);

        for subset in subsets.iter() {
            let cubes: Vec<_> = subset.split(", ").collect();

            for cube in cubes.iter() {
                let cube: Vec<_> = cube.split(" ").collect();
                let cube_number: i32 = cube[0].parse().unwrap();
                let cube_color = cube[1];

                maxes.insert(cube_color, maxes[cube_color].max(cube_number));
            }
        }

        total += maxes["red"] * maxes["green"] * maxes["blue"];
    }

    println!("{}", total);
}
