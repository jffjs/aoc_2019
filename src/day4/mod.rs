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

    // part 2
    let mut possible_codes = 0;
    for code in start..=end {
        let code_str = &format!("{}", code);
        if has_two_adjacent(code_str) && does_not_decrease(code_str) {
            let groups = get_groups(code_str);
            let groups_of_two: Vec<&String> =
                groups.iter().filter(|group| group.len() == 2).collect();

            if groups_of_two.len() >= 1 {
                possible_codes += 1;
            }
        }
    }

    println!("Day 4-2: {}", possible_codes);
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

fn find_group(idx: usize, code: &str, mut group: String) -> String {
    if let Some(c) = code.chars().nth(idx) {
        if let Some(gc) = group.chars().nth(group.len() - 1) {
            if gc == c {
                group.push(c);
                group = find_group(idx + 1, code, group);
            }
        }
    }
    group
}

fn get_groups(code: &str) -> Vec<String> {
    let mut groups: Vec<String> = Vec::new();

    let mut i = 0;
    loop {
        if i >= code.len() {
            break;
        }

        let c = code.chars().nth(i).unwrap();
        let mut group = c.to_string();
        group = find_group(i + 1, code, group);

        let group_len = group.len();
        if group_len > 1 {
            groups.push(group);
        }
        i += group_len;
    }

    groups
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

    #[test]
    fn test_find_group() {
        assert_eq!(
            find_group(1, "11234", String::from("1")),
            String::from("11")
        );
        assert_eq!(
            find_group(1, "1111234", String::from("1")),
            String::from("1111")
        );
        assert_eq!(
            find_group(3, "1122234", String::from("2")),
            String::from("222")
        );
        assert_eq!(
            find_group(5, "112344", String::from("4")),
            String::from("44")
        );
    }

    #[test]
    fn test_get_groups() {
        assert_eq!(
            get_groups("112233"),
            vec!["11".to_owned(), "22".to_owned(), "33".to_owned()]
        );
        assert_eq!(
            get_groups("1122233"),
            vec!["11".to_owned(), "222".to_owned(), "33".to_owned()]
        );
        assert_eq!(
            get_groups("1232233"),
            vec!["22".to_owned(), "33".to_owned()]
        );
        assert_eq!(
            get_groups("1232233"),
            vec!["22".to_owned(), "33".to_owned()]
        );
        assert_eq!(get_groups("111111"), vec!["111111".to_owned()]);
    }
}
