
fn main() {
    let raw = get_input_data();
    let elfs = process(&raw);
    let top_three_sum= elfs.as_slice()[0..3].into_iter().fold(0, |num, currnet_sum|  num + currnet_sum);

    println!("The elf carrying the most calories is {}, total is: {}", 0, elfs[0]);
    println!("Top three elves total {}", top_three_sum);
}

fn process(raw: &str) -> Vec<u64> {
    let mut elf_sum: u64 = 0;
    let mut elves_collection = Vec::new();

    for str_number in raw.split("\n").collect::<Vec<_>>() {
       if str_number.trim() == "" {           
         elves_collection.push(elf_sum);
         elf_sum = 0;
       }  else {
         elf_sum += str_number.trim().parse::<u64>().unwrap_or_default();
       }
    }

    elves_collection.sort(); 
    elves_collection.reverse();

    elves_collection
}

fn get_input_data() -> String {
    std::fs::read_to_string("day1/src/data.txt").expect("could not read file")
}
