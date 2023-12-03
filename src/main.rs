fn main() {
    let input = include_str!("../inputs/day-01.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn parse_line(line: &str) -> Option<usize> {
    let digits = line
        .chars()
        .filter(|n| n.is_ascii_digit())
        .collect::<Vec<_>>();
    let first_digit = digits.first()?;
    let last_digit = digits.last()?;
    format!("{first_digit}{last_digit}").parse::<usize>().ok()
}

fn part1(input: &str) -> usize {
    input.lines().filter_map(parse_line).sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let iter = ElvenDigitsIterator {
                string: line.to_string(),
                index: 0,
            };
            iter.collect::<String>()
        })
        .filter_map(|line| parse_line(&line))
        .sum()
}

struct ElvenDigitsIterator {
    string: String,
    index: usize,
}

impl Iterator for ElvenDigitsIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.string.len() {
            let s = &self.string[self.index..self.string.len()];
            self.index += 1;

            match s.chars().next() {
                Some(char) if char.is_digit(10) => return Some(char),
                _ => {}
            }

            if s.starts_with("one") {
                return Some('1');
            } else if s.starts_with("two") {
                return Some('2');
            } else if s.starts_with("three") {
                return Some('3');
            } else if s.starts_with("four") {
                return Some('4');
            } else if s.starts_with("five") {
                return Some('5');
            } else if s.starts_with("six") {
                return Some('6');
            } else if s.starts_with("seven") {
                return Some('7');
            } else if s.starts_with("eight") {
                return Some('8');
            } else if s.starts_with("nine") {
                return Some('9');
            } else {
                continue;
            }
        }
        None
    }
}

#[test]
fn test_part1() {
    let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    assert_eq!(part1(input), 142);
}

#[test]
fn test_part2() {
    let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

    assert_eq!(part2(input), 281);
}

#[test]
fn test_iterator() {
    let mut iter = ElvenDigitsIterator {
        string: "eightwothree4".to_string(),
        index: 0,
    };

    assert_eq!(iter.next(), Some('8'));
    assert_eq!(iter.next(), Some('2'));
    assert_eq!(iter.next(), Some('3'));
    assert_eq!(iter.next(), Some('4'));
}
