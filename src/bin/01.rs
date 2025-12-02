advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut pointer = 50;
    let mut password = 0;
    for x in input.split('\n') {
        if x.is_empty() {
            continue;
        }
        let (dir, val) = x.split_at(1);
        if dir == "R" {
            pointer += val.parse::<i32>().unwrap();
        } else {
            pointer -= val.parse::<i32>().unwrap();
        }
        pointer = pointer.rem_euclid(100);
        if pointer == 0 {
            password += 1;
        }
    }
    Some(password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut pointer = 50;
    let mut password: u64 = 0;
    for x in input.split('\n') {
        if x.is_empty() {
            continue;
        }
        let (dir, val) = x.split_at(1);
        let mut val = val.parse::<i32>().unwrap();
        if dir == "R" {
            while val > 0 {
                pointer += 1;
                if pointer == 100 {
                    pointer = 0;
                    password += 1;
                }
                val -= 1;
            }
        } else {
            while val > 0 {
                pointer -= 1;
                if pointer == 0 {
                    password += 1;
                }
                if pointer == -1 {
                    pointer = 99;
                }
                val -= 1;
            }
        }
        // if dir == "R" {
        //     pointer += val.parse::<i32>().unwrap();
        // } else {
        //     pointer -= val.parse::<i32>().unwrap();
        // }
        //
        // too high with, too low without
        // let test = match pointer {
        //     ..0 => pointer.abs() + 100,
        //     0.. => pointer,
        // };
        //
        // password += (test / 100).abs() as u64;
        // pointer = pointer.rem_euclid(100);
    }
    Some(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
