use std::collections::HashMap;

fn hash_coord(x: i32, y: i32) -> i32 {
    x * 100_000 + y
}

fn plot_line(map: &mut HashMap<i32, i32>, start: &[i32; 2], end: &[i32; 2], diagonal: bool) {
    let step_x = i32::cmp(&end[0], &start[0]) as i32;
    let step_y = i32::cmp(&end[1], &start[1]) as i32;

    if !diagonal && (step_x != 0 && step_y != 0) {
        return;
    }

    let mut x = start[0];
    let mut y = start[1];
    while !(x == end[0] && y == end[1]) {
        *map.entry(hash_coord(x, y)).or_insert(0) += 1;

        x += step_x;
        y += step_y;
    }

    *map.entry(hash_coord(x, y)).or_insert(0) += 1;
}

fn part1(inputs: &Vec<Vec<[i32; 2]>>) {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for input in inputs {
        plot_line(&mut map, &input[0], &input[1], false);
    }

    println!(
        "Part 1: Number of points where at least two lines overlap: {}",
        map.values().filter(|x| **x >= 2).count()
    );
}

fn part2(inputs: &Vec<Vec<[i32; 2]>>) {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for input in inputs {
        plot_line(&mut map, &input[0], &input[1], true);
    }

    println!(
        "Part 2: Number of points where at least two lines overlap: {}",
        map.values().filter(|x| **x >= 2).count()
    );
}

fn main() {
    let input: Vec<Vec<[i32; 2]>> = include_str!("input.txt")
        .trim()
        .split('\n')
        .map(|line| {
            line.split(" -> ")
                .map(|coord| coord.split(',').map(|n| n.parse::<i32>().unwrap()))
                .map(|mut coord| [coord.next().unwrap(), coord.next().unwrap()])
                .collect()
        })
        .collect();

    part1(&input);
    part2(&input);
}
