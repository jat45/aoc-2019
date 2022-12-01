use adjacent_pair_iterator::AdjacentPairIterator;

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 136760..595731 {
        let s = format!("{}", &i);
        if is_six_digit(&s)
            && at_least_two_adjacent_digits_are_the_same(&s)
            && digits_never_decrease(&s)
        {
            part1 += 1;
            if two_adjacent_matching_digits_are_not_part_of_a_larger_group_of_matching_digits(&s) {
                part2 += 1;
            }
        }
    }
    println!("Part 1: {0}", part1);
    println!("Part 2: {0}", part2);
}

fn is_six_digit(s: &str) -> bool {
    s.len() == 6
}

fn at_least_two_adjacent_digits_are_the_same(s: &str) -> bool {
    s.chars().adjacent_pairs().any(|pair| pair.0 == pair.1)
}

fn digits_never_decrease(s: &str) -> bool {
    s.chars().adjacent_pairs().all(|pair| pair.0 <= pair.1)
}

fn two_adjacent_matching_digits_are_not_part_of_a_larger_group_of_matching_digits(s: &str) -> bool {
    let groups = s.chars().into_iter().fold(Vec::new(), |mut acc, c| {
        if acc.is_empty() {
            acc.push(Vec::new());
        }
        if let Some(last) = acc.last_mut() {
            if let Some(previous) = last.last() {
                if previous == &c {
                    last.push(c);
                } else {
                    // new character found
                    acc.push(vec![c]);
                }
            } else {
                // nothing in last so add current
                last.push(c);
            }
        }
        acc
    });
    groups.iter().any(|group| group.len() == 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("123456", true)]
    #[test_case("1", false)]
    #[test_case("1234567", false)]
    fn test_is_six_digit(input: &str, expected: bool) {
        let actual = is_six_digit(input);

        assert_eq!(actual, expected)
    }

    #[test_case("123456", false)]
    #[test_case("11", true)]
    #[test_case("1223", true)]
    #[test_case("12223", true)]
    fn test_at_least_two_adjacent_digits_are_the_same(input: &str, expected: bool) {
        let actual = at_least_two_adjacent_digits_are_the_same(input);

        assert_eq!(actual, expected)
    }

    #[test_case("123456", true)]
    #[test_case("111111", true)]
    #[test_case("12232", false)]
    fn test_digits_never_decrease(input: &str, expected: bool) {
        let actual = digits_never_decrease(input);

        assert_eq!(actual, expected)
    }

    #[test_case("112233", true)]
    #[test_case("123444", false)]
    #[test_case("111122", true)]
    // 112233 meets these criteria because the digits never decrease and all repeated digits are exactly two digits long.
    // 123444 no longer meets the criteria (the repeated 44 is part of a larger group of 444).
    // 111122 meets the criteria (even though 1 is repeated more than twice, it still contains a double 22).
    fn test_two_adjacent_matching_digits_are_not_part_of_a_larger_group_of_matching_digits(
        input: &str,
        expected: bool,
    ) {
        let actual =
            two_adjacent_matching_digits_are_not_part_of_a_larger_group_of_matching_digits(input);

        assert_eq!(actual, expected)
    }
}
