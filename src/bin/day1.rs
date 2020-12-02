use advent_of_code::{concat_newline, Opts};
use aoc_helper::{AocDay, Puzzle};

fn main() {
    let opts = Opts::parse();

    let mut day_1 = AocDay::new(2020, 1).with_session_id(&opts.token);

    let part_1 = Puzzle::new(1, |input: String| {
        let input: Vec<i64> = input.lines().map(|num| num.parse().unwrap()).collect();
        input
            .iter()
            .enumerate()
            .find_map(|(i, first)| {
                input
                    .iter()
                    .skip(i)
                    .find(|second| first + *second == 2020)
                    .map(|second| first * second)
            })
            .unwrap()
    })
    .with_examples(&[concat_newline!("1721", "979", "366", "299", "675", "1456")]);

    let part_2 = Puzzle::new(2, |input: String| {
        let input: Vec<i64> = input.lines().map(|num| num.parse().unwrap()).collect();
        input
            .iter()
            .enumerate()
            .find_map(|(i, first)| {
                input.iter().enumerate().skip(i).find_map(|(j, second)| {
                    input
                        .iter()
                        .skip(j)
                        .find(|third| first + second + *third == 2020)
                        .map(|third| first * second * third)
                })
            })
            .unwrap()
    })
    .with_examples(&[concat_newline!("1721", "979", "366", "299", "675", "1456")]);

    day_1.test(&part_1);
    day_1.test(&part_2);

    day_1.run(&part_1).unwrap();
    day_1.run(&part_2).unwrap();
}
