use std::fs;

fn part1(lines: &Vec<i32>) {
    let mut prev = 0;
    let mut increments = 0;
    for depth in lines {
        if prev != 0 && *depth > prev {
            increments += 1;
        }

        prev = *depth;
    }

    println!("Total increments of depth: {}", increments);
}

struct Part2Input<'a> {
    offset: usize,
    lines: &'a Vec<i32>,
}
impl<'a> Iterator for Part2Input<'a> {
    type Item = (i32, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.lines.len() - self.offset < 3 {
            return None;
        }

        let result = Some((
            self.lines[self.offset],
            self.lines[self.offset + 1],
            self.lines[self.offset + 2],
        ));
        self.offset += 1;
        return result;
    }
}
fn make_part2_iter(lines: &Vec<i32>) -> Part2Input {
    return Part2Input {
        offset: 0,
        lines: lines,
    };
}

fn part2(lines: &Vec<i32>) {
    let mut prev_sum = 0;
    let mut increments = 0;
    for run in make_part2_iter(lines) {
        let sum = run.0 + run.1 + run.2;
        if prev_sum != 0 && sum > prev_sum {
            increments += 1;
        }

        prev_sum = sum;
    }

    println!("Total increments of running series: {}", increments);
}

fn main() {
    let lines: Vec<i32> = fs::read_to_string("input1.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    part1(&lines);
    part2(&lines);
}
