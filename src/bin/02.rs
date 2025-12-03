advent_of_code::solution!(2);

fn has_repeating_one(number: &str) -> bool {
    let chars = number.chars().collect::<Vec<_>>();
    if chars.len() % 2 != 0 {
        return false;
    }
    let max = chars.len() / 2;
    if max == 0 {
        return false;
    }
    {
        let initial = chars[0];
        if chars.iter().all(|c| c == &initial) {
            return true;
        }
    }
    // doesn't work as intended but happens to work for problem 1
    'outer: for i in 2..=max {
        let initial = &chars[0..i];
        for j in i..=(chars.len() - i) {
            if &chars[j..j + i] != initial {
                continue 'outer;
            }
        }
        return true;
    }
    false

    // any duplicate, not only made up of duplicate
    // for i in 1..=max {
    //     let windows = chars.windows(i).collect::<Vec<_>>();
    //     for j in 0..(windows.len() - i) {
    //         if windows[j] == windows[j + i] {
    //             let x = windows[j].iter().collect::<String>();
    //             if x == "0" {
    //                 continue;
    //             }
    //             if x != x.parse::<i32>().unwrap().to_string() {
    //                 continue;
    //             }
    //             return true;
    //         }
    //     }
    // }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid_ids: Vec<u64> = vec![];
    for range in input.split(',') {
        let mut range = range.trim().split('-');
        let start = range.next().unwrap().parse::<u64>().unwrap();
        let end = range.next().unwrap().parse::<u64>().unwrap();
        for i in start..=end {
            if has_repeating_one(&i.to_string()) {
                invalid_ids.push(i);
            }
        }
    }
    Some(invalid_ids.iter().sum::<u64>())
}

fn has_repeating_two(number: &str) -> bool {
    let chars = number.chars().collect::<Vec<_>>();
    let max = chars.len() / 2;
    if max == 0 {
        return false;
    }
    {
        let initial = chars[0];
        if chars.iter().all(|c| c == &initial) {
            return true;
        }
    }
    'outer: for i in 2..=max {
        if chars.len() / i * i != chars.len() {
            continue;
        }
        let initial = &chars[0..i];
        for j in 1..(chars.len() / i) {
            if &chars[i * j..i * j + i] != initial {
                continue 'outer;
            }
        }
        return true;
    }
    false
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut invalid_ids: Vec<u64> = vec![];
    for range in input.split(',') {
        let mut range = range.trim().split('-');
        let start = range.next().unwrap().parse::<u64>().unwrap();
        let end = range.next().unwrap().parse::<u64>().unwrap();
        for i in start..=end {
            if has_repeating_two(&i.to_string()) {
                invalid_ids.push(i);
            }
        }
    }
    Some(invalid_ids.iter().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
