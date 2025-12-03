advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let banks: Vec<&str> = input.split('\n').collect();
    let mut highest: Vec<i32> = vec![];
    for bank in banks {
        if bank == "" {
            continue;
        }
        let bank = bank.trim();
        let chars: Vec<char> = bank.chars().collect();
        let mut highest_1 = (0, 0);
        for (i, c) in chars[0..chars.len() - 1].iter().enumerate() {
            let num: i32 = c.to_string().parse().unwrap();
            if num > highest_1.1 {
                highest_1 = (i, num);
            }
        }
        let mut highest_2 = 0;
        for c in &chars[highest_1.0 + 1..chars.len()] {
            let num: i32 = c.to_string().parse().unwrap();
            if num > highest_2 {
                highest_2 = num;
            }
        }
        highest.push(highest_1.1 * 10 + highest_2);
    }

    Some(highest.iter().sum::<i32>() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks: Vec<&str> = input.split('\n').collect();
    let mut highest: Vec<u64> = vec![];
    for bank in banks {
        if bank == "" {
            continue;
        }
        let bank = bank.trim();
        let chars: Vec<char> = bank.chars().collect();
        let mut arr = vec![vec!["".to_string(); chars.len()]; 12];
        for i in 0..12 {
            for j in (0..chars.len()).rev() {
                let max_str_w = if i != 0 {
                    if j == chars.len() - 1 {
                        ""
                    } else {
                        arr[i - 1][j + 1].as_str()
                    }
                } else {
                    ""
                };
                let max_str_wo: u64 = if j == chars.len() - 1 {
                    0
                } else {
                    arr[i][j + 1].parse().unwrap()
                };
                let max_str_w: u64 = format!("{}{}", chars[j], max_str_w).parse().unwrap();
                arr[i][j] = max_str_wo.max(max_str_w).to_string();
            }
        }
        highest.push((arr[11][0]).parse().unwrap());
    }

    Some(highest.iter().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
