use std::collections::HashMap;

fn main() {
    let data = get_data();
    let root = Node::new_root_node();
    let mut nodes: HashMap<String, Node> = HashMap::new();

    nodes.insert(root.path.clone(), root);
    buld_directory_tree(&data, &mut nodes);

    println!("Question one answer: {}", find_directory_sum(&nodes));
    println!("Question two answer: {}", find_directory_to_delete(&nodes));
}

fn get_data() -> String {
    std::fs::read_to_string("day7/src/data.txt").unwrap_or_default()
}

fn buld_directory_tree(data: &str, nodes: &mut HashMap<String, Node>) {
    let mut current_dir = nodes.get("/").expect("No root node").path.clone();

    for a_line in data.lines() {
        if a_line.contains("$") {
            let mut command_pieces = a_line.split(" ");
            command_pieces.next(); // the $
            match command_pieces.next().unwrap_or_default() {
                "cd" => {
                    let name = command_pieces.next().unwrap_or_default();
                    if name.contains("..") {
                        let mut pieces = current_dir.split("/").collect::<Vec<_>>();
                        pieces.pop();
                        current_dir = pieces.join("/");
                        if current_dir.is_empty() {
                            current_dir = "/".to_owned();
                        }
                    } else {
                        if current_dir != name {
                            let path = nodes
                                .get(&current_dir)
                                .expect("could not find existing dir")
                                .path
                                .clone();
                            let divider = if current_dir != "/" { "/" } else { "" };
                            current_dir = format!("{}{}{}", path, divider, name);
                        }
                    }
                }
                _ => (),
            }
        } else {
            match nodes.get_mut(&current_dir) {
                Some(current) => {
                    let child = current.add_child_from_str(&a_line);
                    nodes.insert(child.path.clone(), child);
                }
                None => {
                    println!("cannot find parent: {}", &current_dir);
                }
            }
        }
    }
}

fn find_directory_sum(nodes: &HashMap<String, Node>) -> usize {
    let mut sum = 0;
    let max_size = 100000;

    for (_, a_node) in nodes {
        match a_node.node_type {
            NodeType::Directory => {
                let size = a_node.get_size(nodes);
                if size <= max_size {
                    sum += size;
                }
            }
            _ => (),
        }
    }

    sum
}

fn find_directory_to_delete(nodes: &HashMap<String, Node>) -> usize {
    let mut sums: HashMap<usize, String> = HashMap::new();
    let mut sizes = Vec::new();
    let total_disk_space = 70000000usize;
    let minimum_free_space_required = 30000000usize;
    let mut total_used_space = 0usize;

    for (_, a_node) in nodes {
        match a_node.node_type {
            NodeType::Directory => {
                let size = a_node.get_size(nodes);
                if size > total_used_space {
                    total_used_space = size;
                }
                sums.insert(size, a_node.path.clone());
            }
            _ => (),
        }
    }


    for (size, _) in sums.into_iter() {
        sizes.push(size);
    }

    sizes.sort(); 

    let mut the_size = 0;

    for a_size in sizes {
        if a_size >= minimum_free_space_required - (total_disk_space - total_used_space){
           the_size = a_size; 
           break;
        }
    }

    the_size
}

#[derive(Debug, Clone)]
enum NodeType {
    File,
    Directory,
}
#[derive(Debug, Clone)]
struct Node {
    size: usize,
    node_type: NodeType,
    path: String,
    children: Option<Vec<String>>,
}

impl Node {
    fn new(node_str: &str, parent_path: &str) -> Self {
        let mut pieces = node_str.split(" ");
        let node_type = pieces.next().unwrap_or_default();
        let divider = if parent_path != "" && parent_path != "/" {
            "/"
        } else {
            ""
        };

        let path = format!(
            "{}{}{}",
            parent_path,
            divider,
            pieces.next().unwrap_or_default().trim()
        );

        if node_type.contains("dir") {
            Self {
                path,
                node_type: NodeType::Directory,
                children: Some(Vec::new()),
                size: 0,
            }
        } else {
            Self {
                path,
                node_type: NodeType::File,
                children: None,
                size: node_type.parse().unwrap_or_default(),
            }
        }
    }

    fn new_root_node() -> Self {
        Self::new("dir /", "")
    }

    fn add_child(&mut self, child: &Node) {
        match &mut self.children {
            Some(children) => {
                children.push(child.path.clone().into());
            }
            None => (),
        }
    }

    fn add_child_from_str(&mut self, node_str: &str) -> Node {
        let child = Self::new(node_str, &self.path);
        self.add_child(&child);
        child
    }

    fn get_size(&self, nodes: &HashMap<String, Node>) -> usize {
        let mut sum = 0;

        match self.node_type {
            NodeType::Directory => match &self.children {
                Some(children) => {
                    for a_child in children {
                        sum += nodes
                            .get(a_child)
                            .expect("Could not get child")
                            .get_size(nodes);
                    }
                }
                None => (),
            },
            NodeType::File => {
                sum = self.size;
            }
        }

        sum
    }
}
