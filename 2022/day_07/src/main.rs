use no_space_left_on_device::*;

fn main() {
    let input = include_str!("input");
    println!("{:?}", solution_01(transform_input(input)));
    println!("{:?}", solution_02(transform_input(input)));
}

mod no_space_left_on_device {
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    type Input = Vec<Command>;
    type SolutionOne = usize;
    type SolutionTwo = usize;

    #[derive(Default, Debug)]
    pub struct StackCrates(Vec<Vec<char>>);

    #[derive(Debug)]
    pub enum ListInfo {
        Directory { name: String },
        File { size: usize, name: String },
    }

    impl ListInfo {
        fn from_line(line: &str) -> Option<Self> {
            let mut line = line.split_whitespace();
            match line.next()? {
                "dir" => Some(ListInfo::Directory {
                    name: line.next()?.to_string(),
                }),
                size => Some(ListInfo::File {
                    size: size.parse().ok()?,
                    name: line.next()?.to_string(),
                }),
            }
        }
    }

    type NodeHandle = Rc<RefCell<Node>>;

    enum Node {
        File {
            size: usize,
        },
        Dir {
            size: usize,
            children: HashMap<String, NodeHandle>,
            parent_node: Option<NodeHandle>,
        },
    }

    impl std::fmt::Debug for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Node::File { size } => f.debug_struct("NodeFile").field("size", &size).finish(),
                Node::Dir { size, children, .. } => f
                    .debug_struct("NodeStruct")
                    .field("size", &size)
                    .field("children", &children)
                    .finish(),
            }
        }
    }

    #[derive(Debug)]
    pub enum ToDirectory {
        Path(String),
        Previous,
    }

    #[derive(Debug)]
    pub enum Command {
        ChangeDirectory { to: ToDirectory },
        List { result: Vec<ListInfo> },
    }

    impl Command {
        fn from_line(line: &str) -> Option<Self> {
            let mut line = line.split_whitespace();
            match line.next()?.trim() {
                "ls" => Some(Command::List { result: vec![] }),
                "cd" => match line.next()? {
                    ".." => Some(Command::ChangeDirectory {
                        to: ToDirectory::Previous,
                    }),
                    path => Some(Command::ChangeDirectory {
                        to: ToDirectory::Path(path.to_string()),
                    }),
                },
                _ => None,
            }
        }
    }

    fn populate_node_size(node_handle: &mut NodeHandle) -> usize {
        let mut node_size = 0;
        match &mut *node_handle.borrow_mut() {
            Node::File { size } => *size,
            Node::Dir { size, children, .. } => {
                for (_, node) in children.iter_mut() {
                    node_size += populate_node_size(node);
                }
                *size = node_size;
                node_size
            }
        }
    }

    fn get_directories_with_size_less_than(
        path: String,
        node_handle: NodeHandle,
        amount: usize,
        nodes_with_size_less_than: &mut Vec<(String, NodeHandle)>,
    ) {
        let less_than_amount = if let Node::Dir { size, .. } = *node_handle.borrow_mut() {
            Some(size < amount)
        } else {
            None
        };

        if let Some(true) = less_than_amount {
            nodes_with_size_less_than.push((path, node_handle.clone()));
        }
        if let Node::Dir { children, .. } = &mut *node_handle.borrow_mut() {
            for (child_path, child_node) in children {
                get_directories_with_size_less_than(
                    child_path.clone(),
                    child_node.clone(),
                    amount,
                    nodes_with_size_less_than,
                )
            }
        }
    }

    fn process_input(input: Input) -> NodeHandle {
        use std::collections::hash_map::Entry;
        let root = Rc::new(RefCell::new(Node::Dir {
            size: 0,
            children: HashMap::new(),
            parent_node: None,
        }));
        {
            let mut current_node = root.clone();
            for cmd in input {
                match cmd {
                    Command::ChangeDirectory { to } => match to {
                        ToDirectory::Path(path) => {
                            if &path == "/" {
                                current_node = root.clone();
                            } else {
                                let current_node_second_reference = current_node.clone();
                                let mut current_inner_node =
                                    current_node_second_reference.borrow_mut();
                                if let Node::Dir { children, .. } = &mut *current_inner_node {
                                    match children.entry(path) {
                                        Entry::Occupied(val) => {
                                            current_node = val.get().clone();
                                        }
                                        Entry::Vacant(val) => {
                                            val.insert(Rc::new(RefCell::new(Node::Dir {
                                                size: 0,
                                                children: HashMap::new(),
                                                parent_node: Some(current_node.clone()),
                                            })));
                                        }
                                    }
                                }
                            }
                        }
                        ToDirectory::Previous => {
                            let actual_node = current_node.clone();
                            let actual_node_borrowed = &mut *actual_node.borrow_mut();
                            if let Node::Dir {
                                parent_node: Some(parent_node),
                                ..
                            } = actual_node_borrowed
                            {
                                current_node = parent_node.clone();
                            }
                        }
                    },
                    Command::List { result } => {
                        for li in result {
                            match li {
                                ListInfo::Directory { name } => {
                                    if let Node::Dir { children, .. } =
                                        &mut *current_node.borrow_mut()
                                    {
                                        children.insert(
                                            name,
                                            Rc::new(RefCell::new(Node::Dir {
                                                size: 0,
                                                children: HashMap::new(),
                                                parent_node: Some(current_node.clone()),
                                            })),
                                        );
                                    };
                                }
                                ListInfo::File { size, name } => {
                                    if let Node::Dir { children, .. } =
                                        &mut *current_node.borrow_mut()
                                    {
                                        children.insert(
                                            name,
                                            Rc::new(RefCell::new(Node::File { size })),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        root
    }
    pub fn solution_01(input: Input) -> SolutionOne {
        let mut root = process_input(input);
        populate_node_size(&mut root);
        let mut nodes_to_delete = vec![];
        get_directories_with_size_less_than(
            String::from("/"),
            root.clone(),
            100000,
            &mut nodes_to_delete,
        );
        nodes_to_delete
            .into_iter()
            .map(|(_, n)| match *n.borrow_mut() {
                Node::File { size } => size,
                Node::Dir { size, .. } => size,
            })
            .sum()
    }

    pub fn solution_02(input: Input) -> SolutionTwo {
        let mut root = process_input(input);
        populate_node_size(&mut root);
        let used_space = match &*root.borrow() {
            Node::File { size } => *size,
            Node::Dir { size, .. } => *size,
        };
        dbg!(used_space);
        find_smallest_dir_to_delete(root, used_space).unwrap_or_default()
    }

    fn find_smallest_dir_to_delete(node_handle: NodeHandle, used_space: usize) -> Option<usize> {
        if let Node::Dir { children, size, .. } = &*node_handle.borrow() {
            if let Some(child_size) = children
                .values()
                .filter_map(|n| find_smallest_dir_to_delete(n.clone(), used_space))
                .min()
            {
                let size = if child_size < *size {
                    child_size
                } else {
                    *size
                };
                ((used_space - size) <= 40000000).then_some(size)
            } else {
                ((used_space - *size) <= 40000000).then_some(*size)
            }
        } else {
            None
        }
    }

    pub fn transform_input(input: &str) -> Input {
        let mut lines = input.lines();
        let mut commands = vec![];
        while let Some(cmd) = lines.next() {
            match cmd.trim().strip_prefix("$ ") {
                Some(cmd) => {
                    if let Some(cmd) = Command::from_line(cmd) {
                        commands.push(cmd);
                    }
                }
                None => {
                    if let Some(Command::List { result }) = commands.last_mut() {
                        if let Some(list_info) = ListInfo::from_line(cmd) {
                            result.push(list_info);
                        }
                    }
                }
            }
        }
        commands
    }
}

#[cfg(test)]
mod test {
    use super::no_space_left_on_device::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 95437);
    }

    #[test]
    fn test_01_02() {
        let input = include_str!("example_input_01");
        assert_eq!(solution_01(transform_input(input)), 200000);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 24933642);
    }
}
