fn main() {
    let data = get_data();

    println!("Question one ansawer: {}", find_start_of_packet(&data));
    println!("Question two ansawer: {}", find_start_of_message(&data));
}

fn get_data() -> String {
    std::fs::read_to_string("day6/src/data.txt").unwrap_or_default()
}

fn find_start_of_packet(data: &str) -> usize {
    walk_stream(data, 4)
}

fn find_start_of_message(data: &str) -> usize {
    walk_stream(data, 14)
}

fn walk_stream(data: &str, steps: usize) -> usize {
    let mut spot = 0usize;
    for a_line in data.lines() {
        for offset in 0..=a_line.len() - steps {
           let k = &a_line[offset..(offset + steps)] ;
           if is_start_of_packet(&k.chars().collect::<Vec<char>>()) {
            spot = offset + steps;
            break;
           }
        }
    }

    spot
}

fn is_start_of_packet(sequence: &Vec<char>) -> bool {
    let mut the_squence = String::new();
    for a_char in sequence {
        if the_squence.contains::<char>(*a_char) {
            return false;
        }
        the_squence.push(*a_char);
    }
    true
}
