enum Direction {
    Forward,
    Up,
    Down,
}

struct Input {
    direction: Direction,
    distance: i32,
}

fn part1(inputs: &Vec<Input>) {
    let mut horizontal = 0;
    let mut depth = 0;

    for input in inputs {
        match input.direction {
            Direction::Forward => horizontal += input.distance,
            Direction::Up => depth -= input.distance,
            Direction::Down => depth += input.distance,
        }
    }

    println!(
        "Part 1: Horizontal: {}, Depth: {}. X * Depth = {}",
        horizontal,
        depth,
        horizontal * depth
    );
}

fn part2(inputs: &Vec<Input>) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for input in inputs {
        match input.direction {
            Direction::Forward => {
                horizontal += input.distance;
                depth += aim * input.distance;
            }
            Direction::Up => aim -= input.distance,
            Direction::Down => aim += input.distance,
        }
    }

    println!(
        "Part 2: Horizontal: {}, Depth: {}. X * Depth = {}",
        horizontal,
        depth,
        horizontal * depth
    );
}

fn main() {
    let inputs: Vec<Input> = include_str!("input.txt")
        .trim()
        .split('\n')
        .map(|line| {
            let mut split = line.split(' ');
            Input {
                direction: match split.next().unwrap() {
                    "forward" => Some(Direction::Forward),
                    "up" => Some(Direction::Up),
                    "down" => Some(Direction::Down),
                    _ => None,
                }
                .unwrap(),
                distance: split.next().unwrap().parse::<i32>().unwrap(),
            }
        })
        .collect();

    part1(&inputs);
    part2(&inputs);
}
