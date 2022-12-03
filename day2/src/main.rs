fn main() {
    let raw = get_input_data();
    let mut question2_ans= 0;
    let mut question1_ans= 0;
    for line in raw.lines().into_iter() {
        let mut game = Game::new(line);
        question1_ans+= game.my_score();
        game.apply_strategy(); // comment out for question one answer
        question2_ans+= game.my_score();
    }

    println!("question 1 ans: {}", question1_ans);
    println!("question 2 ans: {}", question2_ans);
}

fn get_input_data() -> String {
    std::fs::read_to_string("day2/src/data.txt").unwrap_or_default()
}

#[derive(Debug,PartialEq, Clone)]
enum Moves {
    ROCK,
    PAPER,
    SCISSORS,
}

enum Strategy {
    LOSE, 
    DRAW,
    WIN
}

fn elf_move(letter: &str) -> Option<Moves> {
    match letter {
        "A" => Some(Moves::ROCK),
        "B" => Some(Moves::PAPER),
        "C" => Some(Moves::SCISSORS),
        _ => None,
    }
}

fn strategic_move(letter: &str) -> Option<Strategy> {
    match letter {
        "X" => Some(Strategy::LOSE),
        "Y" => Some(Strategy::DRAW),
        "Z" => Some(Strategy::WIN),
        _ => None,
    }
}

fn my_move(letter: &str) -> Option<Moves> {
    match letter {
        "X" => Some(Moves::ROCK),
        "Y" => Some(Moves::PAPER),
        "Z" => Some(Moves::SCISSORS),
        _ => None,
    }
}

struct Game {
    elf: Moves,
    me: Moves,
    strategy: Strategy
}

impl Game {
    fn new(data: &str) -> Self {
        let mut chars = data.trim().split(" ").into_iter();
        let elfs =chars.next().unwrap_or_default(); 
        let me = chars.next().unwrap_or_default();
        Self {
            elf: elf_move(elfs).unwrap(),
            me: my_move(me).unwrap(),
            strategy: strategic_move(me).unwrap()
        }
    }

    fn my_score(&self) -> i32 {
        match &self.elf {
            Moves::SCISSORS if self.me == Moves::ROCK => 1 + 6,
            Moves::ROCK if self.me == Moves::PAPER => 2 + 6,
            Moves::PAPER if self.me == Moves::SCISSORS => 3 + 6,
            elf_move if *elf_move == self.me => {
                3 + match &self.me {
                    Moves::ROCK => 1,
                    Moves::PAPER => 2,
                    Moves::SCISSORS => 3,
                }
            }
            _ => match &self.me {
                Moves::ROCK => 1,
                Moves::PAPER => 2,
                Moves::SCISSORS => 3,
            },
        }
    }

    fn apply_strategy(&mut self) {
        match &self.strategy {
            Strategy::DRAW => {
                self.me = self.elf.clone()
            },
            Strategy::LOSE => {
                self.me = match &self.elf {
                    Moves::PAPER => Moves::ROCK,
                    Moves::ROCK => Moves::SCISSORS,
                    Moves::SCISSORS => Moves::PAPER
                }
            },
            Strategy::WIN => {
                self.me = match  &self.elf {
                   Moves::ROCK => Moves::PAPER, 
                   Moves::SCISSORS => Moves::ROCK,
                   Moves::PAPER => Moves::SCISSORS
                }
            }
        }
    }
}
