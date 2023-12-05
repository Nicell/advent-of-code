fn main() {
    let input = include_str!("./input.txt");
    part1(input);
    part2(input);
}

struct Rule {
    dst: i64,
    src: i64,
    len: i64,
}

struct Pair {
    start: i64,
    len: i64,
}

fn part1(input: &str) {
    let mut lines = input.lines();

    let line = lines.next().unwrap();
    let seeds = line.split(": ").collect::<Vec<_>>()[1]
        .split_whitespace()
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut maps = Vec::new();

    while let Some(line) = lines.next() {
        if line.len() == 0 {
            continue;
        }

        if !line.chars().next().unwrap().is_digit(10) {
            maps.push(Vec::new());
        } else {
            let numbers = line
                .split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            maps.last_mut().unwrap().push(Rule {
                dst: numbers[0],
                src: numbers[1],
                len: numbers[2],
            });
        }
    }

    let mut minimum = i64::MAX;

    for seed in seeds.iter() {
        let mut current = *seed;

        for map in maps.iter() {
            for rule in map.iter() {
                if current >= rule.src && current < rule.src + rule.len {
                    current = rule.dst + (current - rule.src);
                    break;
                }
            }
        }

        if current < minimum {
            minimum = current;
        }
    }

    println!("{}", minimum);
}

fn part2(input: &str) {
    let mut lines = input.lines();

    let line = lines.next().unwrap();
    let seeds = line.split(": ").collect::<Vec<_>>()[1]
        .split_whitespace()
        .collect::<Vec<_>>()
        .iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut maps = Vec::new();

    while let Some(line) = lines.next() {
        if line.len() == 0 {
            continue;
        }

        if !line.chars().next().unwrap().is_digit(10) {
            maps.push(Vec::new());
        } else {
            let numbers = line
                .split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            maps.last_mut().unwrap().push(Rule {
                dst: numbers[0],
                src: numbers[1],
                len: numbers[2],
            });
        }
    }

    for map in maps.iter_mut() {
        map.sort_by(|a, b| a.src.cmp(&b.src));
    }

    let mut seed_pairs = seeds
        .chunks(2)
        .map(|chunk| Pair {
            start: chunk[0],
            len: chunk[1],
        })
        .collect::<Vec<_>>();

    for map in maps.iter() {
        let mut next_pairs = Vec::new();

        for mut seed_pair in seed_pairs {
            for rule in map.iter() {
                if seed_pair.start < rule.src {
                    next_pairs.push(Pair {
                        start: seed_pair.start,
                        len: seed_pair.len.min(rule.src - seed_pair.start),
                    });

                    seed_pair.start = rule.src;
                    seed_pair.len -= next_pairs.last().unwrap().len;

                    if seed_pair.len <= 0 {
                        break;
                    }
                }

                if seed_pair.start < rule.src + rule.len {
                    next_pairs.push(Pair {
                        start: rule.dst + (seed_pair.start - rule.src),
                        len: seed_pair.len.min(rule.src + rule.len - seed_pair.start),
                    });

                    seed_pair.start = rule.src + rule.len;
                    seed_pair.len -= next_pairs.last().unwrap().len;

                    if seed_pair.len <= 0 {
                        break;
                    }
                }
            }

            if seed_pair.len > 0 {
                next_pairs.push(Pair {
                    start: seed_pair.start,
                    len: seed_pair.len,
                });
            }
        }

        seed_pairs = next_pairs;
    }

    let minimum = seed_pairs.iter().map(|pair| pair.start).min().unwrap();

    println!("{}", minimum);
}
