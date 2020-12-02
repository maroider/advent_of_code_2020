use advent_of_code::{concat_newline, Opts};
use aoc_helper::{AocDay, Puzzle};

fn main() {
    let opts = Opts::parse();

    let mut day_2 = AocDay::new(2020, 2).with_session_id(&opts.token);

    let part_1 = Puzzle::new(1, |input: String| {
        let input: Vec<Entry<'_>> = input.lines().map(|entry| Entry::parse(entry)).collect();
        input
            .iter()
            .filter(|entry| {
                let count = entry.password.chars().filter(|c| *c == entry.glyph).count() as u32;
                count >= entry.first && count <= entry.second
            })
            .count()
    })
    .with_examples(&[concat_newline!(
        "1-3 a: abcde",
        "1-3 b: cdefg",
        "2-9 c: ccccccccc",
    )]);

    let part_2 = Puzzle::new(2, |input: String| {
        let input: Vec<Entry<'_>> = input.lines().map(|entry| Entry::parse(entry)).collect();
        input
            .iter()
            .filter(|entry| {
                let first = entry
                    .password
                    .chars()
                    .nth(entry.first as usize - 1)
                    .map(|c| c == entry.glyph)
                    .unwrap();
                let second = entry
                    .password
                    .chars()
                    .nth(entry.second as usize - 1)
                    .map(|c| c == entry.glyph)
                    .unwrap();
                first ^ second
            })
            .count()
    })
    .with_examples(&[concat_newline!(
        "1-3 a: abcde",
        "1-3 b: cdefg",
        "2-9 c: ccccccccc"
    )]);

    day_2.test(&part_1);
    day_2.test(&part_2);

    day_2.run(&part_1).unwrap();
    day_2.run(&part_2).unwrap();
}

#[derive(Debug)]
struct Entry<'a> {
    first: u32,
    second: u32,
    glyph: char,
    password: &'a str,
}

impl<'a> Entry<'a> {
    fn parse(input: &'a str) -> Self {
        let dash_pos = input.find('-').unwrap();
        let space_pos = input.find(' ').unwrap();
        let colon_pos = input.find(':').unwrap();
        let first = input[0..dash_pos].parse().unwrap();
        let second = input[(dash_pos + 1)..space_pos].parse().unwrap();
        let glyph = input.chars().nth(space_pos + 1).unwrap();
        let password = &input[(colon_pos + 2)..];

        Self {
            first,
            second,
            glyph,
            password,
        }
    }
}
