use std::{fmt::Display, iter::Sum, str::FromStr};

fn main() {
    let input = include_str!("../inputs/day-02.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

struct Game {
    id: usize,
    plays: Vec<Play>,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, plays) = s.split_once(':').ok_or("Invalid game")?;
        let (_, game_id) = game.split_once(' ').ok_or("Invalid game id")?;
        let plays = plays
            .split(';')
            .map(|plays| {
                plays
                    .split(',')
                    .map(|play| play.trim())
                    .filter_map(|play| Play::from_str(play).ok())
                    .sum()
            })
            .collect::<Vec<_>>();

        Ok(Game {
            id: game_id.parse().map_err(|_| "Game id not a number")?,
            plays,
        })
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Play {
    r: usize,
    g: usize,
    b: usize,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq)]
enum PlayParseError {
    InvalidPlay,
    InvalidCubeQuantity,
    InvalidColor(String),
}

impl Display for PlayParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayParseError::InvalidPlay => write!(f, "Invalid play"),
            PlayParseError::InvalidCubeQuantity => write!(f, "Invalid cube quantity"),
            PlayParseError::InvalidColor(color) => write!(f, "Invalid color: {color}"),
        }
    }
}

impl FromStr for Play {
    type Err = PlayParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (n, color) = s.split_once(' ').ok_or(PlayParseError::InvalidPlay)?;
        let n = n
            .parse::<usize>()
            .map_err(|_| PlayParseError::InvalidCubeQuantity)?;
        let p = Play::default();
        match color {
            "red" => Ok(Play { r: n, ..p }),
            "green" => Ok(Play { g: n, ..p }),
            "blue" => Ok(Play { b: n, ..p }),
            color => Err(PlayParseError::InvalidColor(color.to_string())),
        }
    }
}

impl Sum for Play {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Play::default(), |sum, play| Play {
            r: sum.r + play.r,
            g: sum.g + play.g,
            b: sum.b + play.b,
        })
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .filter(|game| {
            let (max_r, max_g, max_b) = game.plays.iter().fold((0, 0, 0), |(r, g, b), play| {
                (r.max(play.r), g.max(play.g), b.max(play.b))
            });
            max_r <= 12 && max_g <= 13 && max_b <= 14
        })
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .map(|game| {
            let (max_r, max_g, max_b) = game.plays.iter().fold((0, 0, 0), |(r, g, b), play| {
                (r.max(play.r), g.max(play.g), b.max(play.b))
            });
            max_r * max_g * max_b
        })
        .sum()
}

#[test]
fn test_part1() {
    let input = indoc::indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    assert_eq!(part1(input), 8);
}

#[test]
fn test_part2() {
    let input = indoc::indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    assert_eq!(part2(input), 2286);
}
