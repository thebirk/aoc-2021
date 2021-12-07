fn fuel_required_for_alignment(crabs: &Vec<i32>, alignment: i32) -> i32 {
    let mut sum = 0;

    for crab in crabs.iter() {
        sum += i32::abs(crab - alignment);
    }

    sum
}

fn part1(input: &Vec<i32>) {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();

    let mut lowest_fuel = i32::MAX;
    for i in *min..=*max {
        let fuel = fuel_required_for_alignment(input, i);

        if fuel < lowest_fuel {
            lowest_fuel = fuel;
        }
    }

    println!(
        "Part 1: Lowest fuel required for alignment is {}",
        lowest_fuel
    );
}

fn fuel_required_for_alignment_2(crabs: &Vec<i32>, alignment: i32) -> i32 {
    let mut sum = 0;

    for crab in crabs.iter() {
        let diff = i32::abs(crab - alignment);
        sum += (diff * diff + diff) / 2;
    }

    sum
}

fn part2(input: &Vec<i32>) {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();

    let mut lowest_fuel = i32::MAX;
    for i in *min..=*max {
        let fuel = fuel_required_for_alignment_2(input, i);

        if fuel < lowest_fuel {
            lowest_fuel = fuel;
        }
    }

    println!(
        "Part 2: Lowest fuel required for alignment is {}",
        lowest_fuel
    );
}

fn main() {
    let input: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    part1(&input);
    part2(&input);
}
