fn get_program(a: i64, b: i64) -> Vec<i64> {
    vec![
        1, a, b, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 5, 23, 1, 6, 23, 27,
        1, 27, 5, 31, 2, 31, 10, 35, 2, 35, 6, 39, 1, 39, 5, 43, 2, 43, 9, 47, 1, 47, 6, 51, 1, 13,
        51, 55, 2, 9, 55, 59, 1, 59, 13, 63, 1, 6, 63, 67, 2, 67, 10, 71, 1, 9, 71, 75, 2, 75, 6,
        79, 1, 79, 5, 83, 1, 83, 5, 87, 2, 9, 87, 91, 2, 9, 91, 95, 1, 95, 10, 99, 1, 9, 99, 103,
        2, 103, 6, 107, 2, 9, 107, 111, 1, 111, 5, 115, 2, 6, 115, 119, 1, 5, 119, 123, 1, 123, 2,
        127, 1, 127, 9, 0, 99, 2, 0, 14, 0,
    ]
}

fn main() {
    let v = get_program(12, 2);
    let a = run_intcode(v);
    let part2 = part2(19690720);
    println!("part 1: {}", a[0]);
    println!("Part 2: {}", part2);
}

fn part2(target: i64) -> i64 {
    for a in 0..99 {
        for b in 0..99 {
            let program = get_program(a, b);
            let result = run_intcode(program);
            if result[0] == target {
                return 100 * result[1] + result[2];
            }
        }
    }
    return -1;
}

fn get(v: &[i64], i: usize) -> i64 {
    let position = *v.get(i).unwrap();
    let value = *v.get(position as usize).unwrap();
    value
}

fn run_intcode(input: Vec<i64>) -> Vec<i64> {
    let mut program = input.clone();
    for pointer in (0..program.len()).step_by(4) {
        let opcode = program[pointer];
        match opcode {
            1 => {
                let value = get(&program, pointer + 1) + get(&program, pointer + 2);
                let position = program.get(pointer + 3).unwrap().clone() as usize;
                program[position] = value
            }
            2 => {
                let value = get(&program, pointer + 1) * get(&program, pointer + 2);
                let position = program.get(pointer + 3).unwrap().clone() as usize;
                program[position] = value
            }
            99 => continue,
            n => {
                println!("unsupported opcode {} found at position {}", n, pointer);
            }
        }
    }
    program
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![1,0,0,0,99], vec![2,0,0,0,99])]
    #[test_case(vec![2,3,0,3,99],vec![2,3,0,6,99])]
    #[test_case(vec![2,4,4,5,99,0],vec![2,4,4,5,99,9801])]
    #[test_case(vec![1,1,1,4,99,5,6,0,99],vec![30,1,1,4,2,5,6,0,99])]
    #[test_case(vec![1,9,10,3,2,3,11,0,99,30,40,50], vec![3500,9,10,70,2,3,11,0,99,30,40,50])]
    #[test_case(vec![1,5,6,4,0,1,98,0], vec![1,5,6,4,99,1,98,0])]
    fn test_run_intcode(program: Vec<i64>, expected: Vec<i64>) {
        let actual = run_intcode(program);

        assert_eq!(actual, expected)
    }
}
