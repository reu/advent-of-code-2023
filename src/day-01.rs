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
        .map(|line| ElvenDigitsIterator::new(line).collect::<String>())
        .filter_map(|line| parse_line(&line))
        .sum()
}

struct ElvenDigitsIterator {
    chars: Vec<char>,
    index: usize,
}

impl ElvenDigitsIterator {
    pub fn new(s: &str) -> Self {
        Self {
            chars: s.chars().collect(),
            index: 0,
        }
    }
}

impl Iterator for ElvenDigitsIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.chars.len() {
            let remaining = &self.chars[self.index..self.chars.len()];

            self.index += 1;

            return match remaining {
                ['o', 'n', 'e', ..] => Some('1'),
                ['t', 'w', 'o', ..] => Some('2'),
                ['t', 'h', 'r', 'e', 'e', ..] => Some('3'),
                ['f', 'o', 'u', 'r', ..] => Some('4'),
                ['f', 'i', 'v', 'e', ..] => Some('5'),
                ['s', 'i', 'x', ..] => Some('6'),
                ['s', 'e', 'v', 'e', 'n', ..] => Some('7'),
                ['e', 'i', 'g', 'h', 't', ..] => Some('8'),
                ['n', 'i', 'n', 'e', ..] => Some('9'),
                [n, ..] if n.is_ascii_digit() => Some(*n),
                _ => continue,
            };
        }
        None
    }
}

#[test]
fn test_part1() {
    let input = indoc::indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    assert_eq!(part1(input), 142);
}

#[test]
fn test_part2() {
    let input = indoc::indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    assert_eq!(part2(input), 281);
}

#[test]
fn test_iterator() {
    let mut iter = ElvenDigitsIterator::new("eightwone3sevenine4");

    assert_eq!(iter.next(), Some('8'));
    assert_eq!(iter.next(), Some('2'));
    assert_eq!(iter.next(), Some('1'));
    assert_eq!(iter.next(), Some('3'));
    assert_eq!(iter.next(), Some('7'));
    assert_eq!(iter.next(), Some('9'));
    assert_eq!(iter.next(), Some('4'));
    assert_eq!(iter.next(), None);
}
