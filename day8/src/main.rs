fn main() {
    let data = get_data();
    let mut grid: Vec<Vec<Tree>> = Vec::new();
    let total_rows;
    let mut total_columns = 0;

    for (a_arow, a_line) in data.lines().enumerate() {
        let mut row: Vec<Tree> = Vec::new();
        for (a_column, a_char) in a_line.chars().enumerate() {
            let tree = Tree::new(
                a_char.to_digit(10).unwrap_or_default() as i32,
                a_arow,
                a_column,
            );
            row.push(tree);
        }
        if total_columns == 0 {
            total_columns = row.len();
        }
        grid.push(row);
    }
    total_rows = grid.len();

    check_visibility(&mut grid, total_rows, total_columns);
}

fn get_data() -> String {
    std::fs::read_to_string("day8/src/data.txt").unwrap_or_default()
}

fn check_visibility_up(subject: &Tree, grid: &Vec<Vec<Tree>>) -> (bool, i32) {
    let mut visible;
    let mut tree_ideal = 0;
    let mut row_up = subject.row - 1;
    loop {
        let a_tree = grid
            .get(row_up)
            .expect("bad")
            .get(subject.column)
            .expect("bad");
        if a_tree.height >= subject.height {
            tree_ideal += 1;
            visible = false;
            break;
        } else {
            tree_ideal += 1;
            visible = true;
        }

        if row_up > 0 {
            row_up -= 1;
        } else {
            break;
        }
    }

    (visible, tree_ideal)
}

fn check_visibility_down(subject: &Tree, grid: &Vec<Vec<Tree>>, total_rows: usize) -> (bool, i32) {
    let mut tree_ideal = 0;
    let mut visible = false;
    for row_down in subject.row + 1..total_rows {
        let a_tree = grid
            .get(row_down)
            .expect("bad")
            .get(subject.column)
            .expect("bad");
        if a_tree.height >= subject.height {
            visible = false;
            tree_ideal += 1;
            break;
        } else {
            tree_ideal += 1;
            visible = true;
        }
    }

    (visible, tree_ideal)
}

fn check_visibility_left(subject: &Tree, grid: &Vec<Vec<Tree>>) -> (bool, i32) {
    let mut column_left = subject.column - 1;
    let mut tree_ideal = 0;
    let mut visible;
    loop {
        let a_tree = grid
            .get(subject.row)
            .expect("bad")
            .get(column_left)
            .expect("bad");
        if a_tree.height >= subject.height {
            visible = false;
            tree_ideal += 1;
            break;
        } else {
            tree_ideal += 1;
            visible = true;
        }

        if column_left > 0 {
            column_left -= 1;
        } else {
            break;
        }
    }

    (visible, tree_ideal)
}

fn check_visibility_right(
    subject: &Tree,
    grid: &Vec<Vec<Tree>>,
    total_columns: usize,
) -> (bool, i32) {
    let mut tree_ideal = 0;
    let mut visible = false;
    for column_right in subject.column + 1..total_columns {
        let a_tree = grid
            .get(subject.row)
            .expect("bad")
            .get(column_right)
            .expect("bad");
        if a_tree.height >= subject.height {
            visible = false;
            tree_ideal += 1;
            break;
        } else {
            tree_ideal += 1;
            visible = true;
        }
    }

    (visible, tree_ideal)
}

fn check_visibility(grid: &mut Vec<Vec<Tree>>, total_rows: usize, total_columns: usize) {
    let mut sum = 0;
    let mut ideal = 0;
    for a_row in 0..total_rows {
        for a_column in 0..total_columns {
            let tree = grid
                .get(a_row)
                .expect("could not get row")
                .get(a_column)
                .expect("could not get column");
            let mut visible;

            if tree.row == 0
                || tree.row + 1 == total_rows
                || tree.column == 0
                || tree.column + 1 == total_columns
            {
                visible = true;
            } else {
                // up
                let mut result = check_visibility_up(&tree, grid);
                visible = result.0;
                let mut tree_ideal = result.1;

                // down
                result = check_visibility_down(&tree, grid, total_rows);
                tree_ideal *= result.1;
                if !visible {
                    visible = result.0;
                }

                // left
                result = check_visibility_left(&tree, grid);
                tree_ideal *= result.1;
                if !visible {
                    visible = result.0;
                }

                // right
                result = check_visibility_right(&tree, grid, total_columns);
                tree_ideal *= result.1;
                if !visible {
                    visible = result.0;
                }

                if ideal < tree_ideal {
                    ideal = tree_ideal;
                }
            }

            if visible {
                sum += 1;
            }
        }
    }

    println!("Question 1 answer: {}", sum);
    println!("Question 2 answer: {}", ideal);
}

#[derive(Debug)]
struct Tree {
    height: i32,
    row: usize,
    column: usize,
}

impl Tree {
    fn new(height: i32, row: usize, column: usize) -> Self {
        Self {
            height,
            row,
            column,
        }
    }
}
