pub fn day_4() {
    let input = "367479-893698";
    let split: Vec<&str> = input.split('-').collect();
    let start = split[0].parse::<u32>().unwrap();
    let end = split[1].parse::<u32>().unwrap();

    // part 1
    let mut possible_codes = 0;
    for code in start..=end {
        let code_str = &format!("{}", code);
        if has_two_adjacent(code_str) && does_not_decrease(code_str) {
            possible_codes += 1;
        }
    }

    println!("Day 4-1: {}", possible_codes);
}

fn has_two_adjacent(code: &str) -> bool {
    for (i, c) in code.chars().enumerate() {
        if let Some(next) = code.chars().nth(i + 1) {
            if next == c {
                return true;
            }
        }
    }

    false
}

fn does_not_decrease(code: &str) -> bool {
    for (i, c) in code.chars().enumerate() {
        if let Some(next) = code.chars().nth(i + 1) {
            if next < c {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_two_adjacent() {
        assert_eq!(has_two_adjacent("112345"), true);
        assert_eq!(has_two_adjacent("123456"), false);
        assert_eq!(has_two_adjacent("122345"), true);
        assert_eq!(has_two_adjacent("122345"), true);
        assert_eq!(has_two_adjacent("123455"), true);
        assert_eq!(has_two_adjacent("111111"), true);
        assert_eq!(has_two_adjacent("121212"), false);
    }

    #[test]
    fn test_does_not_decrease() {
        assert_eq!(does_not_decrease("123456"), true);
        assert_eq!(does_not_decrease("121456"), false);
        assert_eq!(does_not_decrease("111222"), true);
        assert_eq!(does_not_decrease("999998"), false);
        assert_eq!(does_not_decrease("999990"), false);
    }
}
