fn main() {
    let data = get_data();

    println!("Question one answer {}", drive_crane_9000(&data));
    println!("Question two answer {}", drive_crane_9001(&data));
}

fn get_data() -> String {
    std::fs::read_to_string("day5/src/data.txt").unwrap_or_default()
}

fn drive_crane_9000(data: &str) -> String {
   generate_ans(drive_crane(&data, false))
}

fn drive_crane_9001(data: &str) -> String {
   generate_ans(drive_crane(&data, true))
}

fn generate_ans(stacks: Vec<Stack>) -> String {
    let mut ans = String::new();
    for a_stack in &stacks {
        ans.push_str(&a_stack.first_crate_value());
    }

    ans
}

fn get_stacks(data: &str) -> Vec<Stack> {
    let mut crate_data: Vec<&str> = Vec::new();
    let mut total_stack = 0;
    let mut stack: Vec<Stack> = Vec::new();

    for a_line in data.lines() {
        if a_line.is_empty() {
            break;
        }
        if a_line.contains('1') {
            for a_number in a_line.split(' ') {
                if !a_number.is_empty() {
                    total_stack += 1;
                }
            }
        } else {
            crate_data.push(a_line);
        }
    }

    for _ in 0..total_stack {
        stack.push(Stack::new());
    }

    for a_line in crate_data {
        let mut chars = a_line.chars();
        for a_stack in 0..total_stack {
            let mut value = String::new();

            chars.next();  // skip [
            value.push(chars.next().unwrap_or_default());  // A...Z
            chars.next(); // skip ]
            chars.next(); // skip the space in betwen

            if !value.trim().is_empty() {
                match stack.get_mut(a_stack) {
                    Some(s) => {
                        s.stack_crate(TheCrate::new(
                            value.trim(),
                        ));
                    }
                    None => (),
                }
            }
        }
    }

    stack
}

fn drive_crane(data: &str, keep_order: bool) -> Vec<Stack> {
    let mut at_commands = false;
    let mut stacks = get_stacks(data);

    for a_line in data.lines() {
        if a_line.trim().is_empty() {
            at_commands = true;
            continue;
        }

        if at_commands {
            let pieces = a_line.split(' ').collect::<Vec<&str>>();

            let amount: usize = (*pieces.get(1).unwrap_or(&"0")).parse().unwrap_or_default();
            let from: usize = (*pieces.get(3).unwrap_or(&"0"))
                .parse::<usize>()
                .unwrap_or_default()
                - 1;
            let to: usize = (*pieces.get(5).unwrap_or(&"0"))
                .parse::<usize>()
                .unwrap_or_default()
                - 1;

            match stacks.get_mut(from) {
                Some(from_stack) => {
                    let crates = from_stack.remove_n_crates(amount, keep_order);
                    if !crates.is_empty() {
                        stacks
                            .get_mut(to)
                            .expect("could not get distination stack")
                            .stack_n_crates_on_top(crates);
                    }
                }
                None => (),
            }
        }
    }

    stacks
}

#[derive(Debug)]
struct Stack {
    crates: Vec<TheCrate>,
}

#[derive(Debug, Clone)]
struct TheCrate {
    value: String,
}

impl Stack {
    fn new() -> Self {
        Self { crates: Vec::new() }
    }

    fn stack_crate(&mut self, a_crate: TheCrate) {
        self.crates.push(a_crate);
    }

    fn stack_create_on_top(&mut self, a_crate: TheCrate) {
        self.crates.insert(0, a_crate);
    }

    fn stack_n_crates_on_top(&mut self, crates: Vec<TheCrate>) {
        for a_crate in crates {
            self.stack_create_on_top(a_crate);
        }
    }

    fn remove_n_crates(&mut self, amount: usize, keep_order: bool) -> Vec<TheCrate> {
        let mut removed_crates = Vec::new();
        for _ in 0..amount {
            if let Some(a_crate) = self.crates.first() {
                removed_crates.push(a_crate.clone());
                self.crates.drain(0..1);
            }
        }

        if keep_order {
            removed_crates.reverse();
        }

        removed_crates
    }

    fn first_crate_value(&self) -> String {
        self.crates
            .get(0)
            .unwrap_or(&TheCrate::new(""))
            .value
            .to_owned()
    }
}

impl TheCrate {
    fn new(value: &str) -> Self {
        Self {
            value: value.into(),
        }
    }
}
