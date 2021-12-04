type Board = Vec<i32>;

fn sum_of_unmarked(mark: &[bool; 25], board: &Board) -> i32 {
    let mut sum = 0;

    for y in 0..5 {
        for x in 0..5 {
            if !mark[x + y * 5] {
                sum += board[x + y * 5];
            }
        }
    }

    sum
}

fn find_win(draws: &Vec<i32>, board: &Board) -> Option<(i32, i32)> {
    let mut mark = [false; 25];

    for (round, draw) in draws.iter().enumerate() {
        match board.iter().position(|x| x == draw) {
            Some(pos) => mark[pos] = true,
            None => continue,
        };

        for x in 0..5 {
            let mut win_row = true;
            let mut win_col = true;
            for y in 0..5 {
                if !mark[x + y * 5] {
                    win_row = false;
                }
                if !mark[y + x * 5] {
                    win_col = false;
                }
            }

            if win_row || win_col {
                return Some((round as i32, sum_of_unmarked(&mark, board) * draw));
            }
        }
    }

    None
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt")
        .trim()
        .split('\n')
        .filter(|x| !x.is_empty())
        .collect();

    let mut input_iter = input.iter();

    let draws: Vec<i32> = input_iter
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let boards: Vec<Board> = input_iter
        .as_slice()
        .chunks(5)
        .map(|x| {
            x.iter()
                .map(|x| {
                    x.trim()
                        .split(' ')
                        .filter(|x| !x.is_empty())
                        .map(|n| n.trim().parse::<i32>().unwrap())
                })
                .flatten()
                .collect()
        })
        .collect();

    let mut wins: Vec<(usize, i32, i32)> = boards
        .iter()
        .enumerate()
        .map(|(i, b)| (i, find_win(&draws, b)))
        .filter(|w| w.1.is_some())
        .map(|w| (w.0, w.1.unwrap()))
        .map(|w| (w.0, w.1 .0, w.1 .1))
        .collect();
    wins.sort_by(|a, b| a.1.cmp(&b.1));

    let best_board = wins[0];
    println!(
        "Board #{} wins in the fewest moves {}, with a score {}",
        best_board.0, best_board.1, best_board.2,
    );
    for row in boards[best_board.0].chunks(5) {
        println!("{:?}", row);
    }

    let worst_board = wins[wins.len() - 1];
    println!(
        "Board #{} wins with the most moves {}, with a score {}",
        worst_board.0, worst_board.1, worst_board.2,
    );
    for row in boards[worst_board.0].chunks(5) {
        println!("{:?}", row);
    }
}
