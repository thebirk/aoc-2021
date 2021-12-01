#[derive(Debug)]
enum RotateDirection {
    Left,
    Right,
}

#[derive(Debug)]
struct Input {
    direction: RotateDirection,
    distance: i32,
}

enum Direction {
    North,
    East,
    South,
    West,
}

fn rotate_left(orientation: Direction) -> Direction {
    match orientation {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    }
}

fn rotate_right(orientation: Direction) -> Direction {
    match orientation {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn manhattan_distance(x0: i32, y0: i32, x1: i32, y1: i32) -> i32 {
    i32::abs(x0 - x1) + i32::abs(y0 - y1)
}

fn part1(inputs: &Vec<Input>) {
    let mut pos = [0, 0];
    let mut orientation = Direction::North;
    for input in inputs {
        orientation = match input.direction {
            RotateDirection::Left => rotate_left(orientation),
            RotateDirection::Right => rotate_right(orientation),
        };
        match orientation {
            Direction::North => pos[1] += input.distance,
            Direction::East => pos[0] -= input.distance,
            Direction::South => pos[1] -= input.distance,
            Direction::West => pos[0] += input.distance,
        }
    }

    println!(
        "Easter Bunny HQ is {} blocks aways",
        manhattan_distance(0, 0, pos[0], pos[1])
    );
}

fn part2(inputs: &Vec<Input>) {}

fn main() {
    let input: Vec<Input> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|x| x.trim().chars())
        .map(|mut x| {
            match x.next().unwrap() {
                'L' => Some(Input {
                    direction: RotateDirection::Left,
                    distance: x.collect::<String>().parse::<i32>().unwrap(),
                }),
                'R' => Some(Input {
                    direction: RotateDirection::Right,
                    distance: x.collect::<String>().parse::<i32>().unwrap(),
                }),
                _ => None,
            }
            .unwrap()
        })
        .collect();

    part1(&input);
    part2(&input);
}
