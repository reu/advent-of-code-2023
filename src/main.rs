fn main() {
    let input = include_str!("../inputs/day-01.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let digits = line.chars().filter(|n| n.is_digit(10)).collect::<Vec<_>>();
            let first_digit = digits.first()?;
            let last_digit = digits.last()?;
            format!("{first_digit}{last_digit}").parse::<usize>().ok()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line
                .replace("oneight", "18")
                .replace("twone", "21")
                .replace("threeight", "38")
                .replace("fiveight", "58")
                .replace("eightwo", "82")
                .replace("eighthree", "83")
                .replace("nineight", "98")
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9")
        })
        .filter_map(|line| {
            let digits = line.chars().filter(|n| n.is_digit(10)).collect::<Vec<_>>();
            let first_digit = digits.first()?;
            let last_digit = digits.last()?;
            format!("{first_digit}{last_digit}").parse::<usize>().ok()
        })
        .sum()
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
