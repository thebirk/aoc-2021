use std::fs;

fn part1(lines: &Vec<i32>) {
    let mut prev = lines[0];
    let mut increments = 0;
    for depth in &lines[1..] {
        if *depth > prev {
            increments += 1;
        }

        prev = *depth;
    }

    println!("Total increments of depth: {}", increments);
}

struct SlidingWindow<'a, T> {
    offset: usize,
    window_size: usize,
    data: &'a Vec<T>,
}

impl<'a, T> SlidingWindow<'a, T> {
    pub fn new(data: &'a Vec<T>, window_size: usize) -> SlidingWindow<T> {
        SlidingWindow {
            offset: 0,
            window_size: window_size,
            data: data,
        }
    }
}

impl<'a, T> Iterator for SlidingWindow<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() - self.offset < self.window_size {
            return None;
        }

        let offset = self.offset;
        self.offset += 1;

        return Some(&self.data[offset..=offset + 2]);
    }
}

fn part2(lines: &Vec<i32>) {
    let mut prev_sum = 0;
    let mut increments = 0;
    for window in SlidingWindow::new(lines, 3) {
        let sum = window[0] + window[1] + window[2];
        if prev_sum != 0 && sum > prev_sum {
            increments += 1;
        }

        prev_sum = sum;
    }

    let a: Vec<_> = SlidingWindow::new(lines, 3).collect();

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
