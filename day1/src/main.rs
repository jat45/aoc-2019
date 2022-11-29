use std::io::{BufRead, BufReader};

fn main() {
    let input = read_input("./input.txt");
    if let Ok(masses) = input {
        let part1: i64 = masses.iter().map(fuel_requirement).sum();
        let part2: i64 = masses.iter().map(fuel_requirement_complex).sum();
        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
    } else {
        println!("Couldn't parse input {:?}", input);
    }
}

fn fuel_requirement(mass: &i64) -> i64 {
    (mass / 3) - 2
}

// A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
// At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel. So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
// The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
fn fuel_requirement_complex(mass: &i64) -> i64 {
    let mut sum = 0;
    let mut mass = fuel_requirement(mass);
    while mass > 0 {
        sum += mass;
        mass = fuel_requirement(&mass)
    }
    sum
}

fn read_input(filepath: &str) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(filepath)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .into_iter()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(12, 2)]
    #[test_case(14, 2)]
    #[test_case(1969, 654)]
    #[test_case(100756, 33583)]
    fn test_fuel_requirement(mass: i64, expected: i64) {
        let actual = fuel_requirement(&mass);

        assert_eq!(actual, expected)
    }

    // A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
    // At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel. So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
    // The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
    #[test_case(14, 2)]
    #[test_case(1969, 966)]
    #[test_case(100756, 50346)]
    fn test_fuel_requirement_complex(mass: i64, expected: i64) {
        let actual = fuel_requirement_complex(&mass);

        assert_eq!(actual, expected);
    }
}
