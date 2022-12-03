use std::collections::HashMap;

fn main() {
    let mut lowercase = HashMap::new();
    let mut uppercase = HashMap::new();
    let mut points = 1;
    let data = get_data();

    for char in 'a'..='z' {
        lowercase.insert(char, points);
        points += 1;
    }

    for char in 'A'..='Z' {
        uppercase.insert(char, points);
        points += 1;
    }

    let mut sum = 0;

    for input in data.lines() {
        let ans = pluck_duplicates(input);
        sum += calculate_points(&ans, &lowercase, &uppercase);
    }

    println!("Question 1 answer: {}", sum);

    let badges = solve_question_two(&data);
    sum = calculate_points(&badges, &lowercase, &uppercase);

    println!("Question 2 answer: {}", sum);
}

fn get_data() -> String {
    return std::fs::read_to_string("day3/src/data.txt").unwrap_or_default();
}

fn pluck_duplicates(input: &str) -> Vec<char> {
    let half = input.chars().count() / 2;
    let mut duplicates = HashMap::new();

    for (index, ch) in input.chars().enumerate() {
        if index == half as usize {
            break;
        }
        duplicates.insert(ch.clone(), 1);
    }

    for (index, ch) in input.chars().enumerate() {
        if index < half {
            continue;
        }

        if let Some(old) = duplicates.get_mut(&ch) {
            *old += 1;
        }
    }

    let mut ans = Vec::new();

    for entry in duplicates.iter() {
        if *entry.1 > 1 {
            ans.push(entry.0.clone());
        }
    }

    ans
}

fn calculate_points(
    occurance: &[char],
    lowercase: &HashMap<char, i32>,
    uppercase: &HashMap<char, i32>,
) -> i32 {
    let mut sum = 0;
    for c in occurance {
        if let Some(p) = lowercase.get(&c) {
            sum += p;
        }
        if let Some(p) = uppercase.get(&c) {
            sum += p;
        }
    }

    sum
}

fn solve_question_two(data: &str) -> Vec<char> {
    let mut badges = Vec::new();
    let mut group_badges = HashMap::new();
    let mut has_entered = Vec::new();

    let mut lines = data.lines();
    let mut completed = false;
    loop {
        for index in 0..3 {
            has_entered.clear();
            if let Some(input) = lines.next() {
                for ch in input.chars() {
                    if has_entered.contains(&ch) {
                        continue;
                    }

                    if let Some(p) = group_badges.get_mut(&ch) {
                        *p += 1;
                    } else if index == 0 {
                        group_badges.insert(ch.clone(), 1);
                    }
                    has_entered.push(ch.clone());
                }
            } else {
                completed = true;
            }

            if index == 2 {
                for entry in group_badges.iter() {
                    if *entry.1 == 3 {
                        badges.push(entry.0.clone());
                    }
                }
                group_badges.clear();
            }
        }

        if completed {
            break;
        }
    }

    badges
}
