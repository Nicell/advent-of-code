fn main() {
    let input = include_str!("./input.txt");
    part1(input);
    part2(input);
}

fn neighbor_is_symbol(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let mut neighbors = 0;

    fn is_symbol(c: char) -> bool {
        !c.is_digit(10) && c != '.'
    }

    for row in -1..2 {
        for col in -1..2 {
            if row == 0 && col == 0 { continue; }
            if is_symbol(schematic[(i as i32 + row) as usize][(j as i32 + col) as usize]) { neighbors += 1; }
        }
    }

    neighbors > 0
}

fn part1(input: &str) {
    let mut lines = input.lines();
    let mut schematic: Vec<Vec<char>> = Vec::new();

    while let Some(line) = lines.next() {
        let mut chars = line.chars();
        let mut row: Vec<char> = Vec::new();

        row.push('.');
        while let Some(c) = chars.next() {
            row.push(c);
        }
        row.push('.');

        schematic.push(row);
    }

    schematic.insert(0, vec!['.'; schematic[0].len()]);
    schematic.push(vec!['.'; schematic[0].len()]);

    let mut total = 0;

    for (i, row) in schematic.iter().enumerate() {
        let mut number: Vec<char> = Vec::new();
        let mut valid = false;

        for (j, e) in row.iter().enumerate() {
            if e.is_digit(10) {
                number.push(*e);
                if valid || neighbor_is_symbol(&schematic, i, j) { valid = true };
                continue;
            }

            if valid {
                let number: String = number.iter().collect();
                let number: i32 = number.parse().unwrap();
                total += number;
            }

            number = Vec::new();
            valid = false;
        }
    }

    println!("{}", total);
}

fn find_number(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut number: Vec<char> = Vec::new();

    if !schematic[i][j].is_digit(10) {
        println!("{}", schematic[i][j])
    }

    number.push(schematic[i][j]);

    let mut before = j - 1;
    while schematic[i][before].is_digit(10) {
        number.insert(0, schematic[i][before]);
        before -= 1;
    }

    let mut after = j + 1;
    while schematic[i][after].is_digit(10) {
        number.push(schematic[i][after]);
        after += 1;
    }

    number.iter().collect::<String>().parse().unwrap()
}

fn get_gear_ratio(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();

    for row in -1..2 {
        if schematic[(i as i32 + row) as usize][j].is_digit(10) {
            numbers.push(find_number(schematic, (i as i32 + row) as usize, j));
        } else {
            for col in -1..2 {
                if schematic[(i as i32 + row) as usize][(j as i32 + col) as usize].is_digit(10) {
                    numbers.push(find_number(schematic, (i as i32 + row) as usize, (j as i32 + col) as usize));
                }
            }
        }
    }

    if numbers.len() == 2 {
        numbers[0] * numbers[1]
    } else {
        0
    }
}

fn part2(input: &str) {
    let mut lines = input.lines();
    let mut schematic: Vec<Vec<char>> = Vec::new();

    while let Some(line) = lines.next() {
        let mut chars = line.chars();
        let mut row: Vec<char> = Vec::new();

        row.push('.');
        while let Some(c) = chars.next() {
            row.push(c);
        }
        row.push('.');

        schematic.push(row);
    }

    schematic.insert(0, vec!['.'; schematic[0].len()]);
    schematic.push(vec!['.'; schematic[0].len()]);

    let mut total = 0;

    for (i, row) in schematic.iter().enumerate() {
        for (j, e) in row.iter().enumerate() {
            if *e == '*' {
                total += get_gear_ratio(&schematic, i, j)
            }
        }
    }

    println!("{}", total);
}
