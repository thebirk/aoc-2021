fn step(state: &mut Vec<i64>) {
    let mut new_fish = 0;

    for fish in state.iter_mut() {
        if *fish == 0 {
            *fish = 6;
            new_fish += 1;
        } else {
            *fish -= 1;
        }
    }

    for _ in 0..new_fish {
        state.push(8);
    }
}

fn part1(input: &Vec<i64>) {
    let mut fish = input.clone();
    for _ in 0..80 {
        step(&mut fish);
    }

    println!(
        "After 80 days the lanternfish population ish {}",
        fish.iter().count()
    );
}

fn part2(input: &Vec<i64>) {
    let mut state: [i64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for n in input.iter() {
        state[*n as usize] += 1;
    }

    for _ in 0..256 {
        let new_fish = state[0];
        state[0] = state[1];
        state[1] = state[2];
        state[2] = state[3];
        state[3] = state[4];
        state[4] = state[5];
        state[5] = state[6];
        state[6] = new_fish + state[7];
        state[7] = state[8];
        state[8] = new_fish;
    }

    println!(
        "After 256 days the lanternfish population is {}",
        state.iter().sum::<i64>()
    );
}

fn main() {
    let input: Vec<i64> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    part1(&input);
    part2(&input);
}
