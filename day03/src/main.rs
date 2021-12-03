fn part1(inputs: &Vec<&str>) {
    let mut bit_counter: Vec<usize> = Vec::new();
    for _ in 0..inputs[0].len() {
        bit_counter.push(0);
    }

    for input in inputs {
        for i in 0..input.len() {
            bit_counter[i] += match input.bytes().nth(i).unwrap() {
                b'0' => Some(0),
                b'1' => Some(1),
                _ => None,
            }
            .unwrap();
        }
    }

    let mut gamma_rate_str = String::new();
    let mut epsilon_rate_str = String::new();
    for i in 0..inputs[0].len() {
        let half_total = inputs.len() / 2;
        let set_bits = bit_counter[i];

        if set_bits >= half_total {
            gamma_rate_str.push('1');
            epsilon_rate_str.push('0');
        } else {
            gamma_rate_str.push('0');
            epsilon_rate_str.push('1');
        }
    }

    let gamma_rate = i32::from_str_radix(gamma_rate_str.as_str(), 2).unwrap();
    let epsilon_rate = i32::from_str_radix(epsilon_rate_str.as_str(), 2).unwrap();

    println!(
        "Submarine power consumption is {}",
        gamma_rate * epsilon_rate
    )
}

fn find_rating<P>(inputs: &Vec<&str>, predicate: P) -> usize
where
    P: Fn(usize, usize) -> char,
{
    let mut pattern = String::new();

    loop {
        let rem: Vec<&&str> = inputs
            .iter()
            .filter(|x| x.starts_with(pattern.as_str()))
            .collect();

        if rem.len() == 1 {
            return usize::from_str_radix(rem[0], 2).unwrap();
        }

        let set_bits = rem
            .iter()
            .filter(|x| x.bytes().nth(pattern.len()).unwrap() == b'1')
            .count();

        let unset_bits = rem.len() - set_bits;

        pattern.push(predicate(set_bits, unset_bits));
    }
}

fn part2(inputs: &Vec<&str>) {
    let oxygen_generator_rating =
        find_rating(inputs, |set, unset| if set >= unset { '1' } else { '0' });
    let co2_scrubber_rating =
        find_rating(inputs, |set, unset| if unset <= set { '0' } else { '1' });

    println!("Oxygen generator rating is {}", oxygen_generator_rating);
    println!("CO2 scrubber rating is {}", co2_scrubber_rating);
    println!(
        "Submarine life support rating is {}",
        oxygen_generator_rating * co2_scrubber_rating
    );
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").trim().split('\n').collect();

    part1(&input);
    part2(&input);
}
