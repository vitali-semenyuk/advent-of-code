use std::fmt::Display;

#[derive(Debug)]
enum Command<'a> {
    ChangeDirectory(&'a str),
    List(Vec<FsItem>),
}

#[derive(Debug)]
enum FsItem {
    Directory(String),
    File { name: String, size: u32 },
}

impl From<&str> for FsItem {
    fn from(string: &str) -> Self {
        let (size, name) = string.split_once(" ").unwrap();
        let name = name.to_string();

        if size == "dir" {
            Self::Directory(name)
        } else {
            let size = size.parse().unwrap();
            Self::File { name, size }
        }
    }
}

#[derive(Debug)]
struct FsNode<'a> {
    name: String,
    size: u32,
    parent: Option<&'a FsNode<'a>>,
    children: Vec<FsNode<'a>>,
}

impl FsNode<'_> {
    fn new(name: String) -> FsNode<'static> {
        FsNode {
            name,
            size: 0,
            parent: None,
            children: Vec::new(),
        }
    }

    fn change_directory(&mut self, name: &str) -> &mut FsNode {
        self
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> i32 {
    let commands: Vec<_> = input[1..]
        .split("$ ")
        .map(|command| {
            let command = command.trim();
            if command.starts_with("cd") {
                let (_, directory) = command.split_once(" ").unwrap();
                Command::ChangeDirectory(directory)
            } else {
                let content = command.lines().skip(1).map(FsItem::from).collect();
                Command::List(content)
            }
        })
        .collect();

    let mut root = FsNode::new("/".to_string());
    let mut current_directory = &mut root;
    for command in commands {
        match command {
            Command::ChangeDirectory(directory) => {
                current_directory = current_directory.change_directory(directory);
                // current_directory = if directory == "/" {
                //     &root
                // } else {
                //     current_directory.change_directory(directory)
                // }
            }
            Command::List(fs_items) => {
                for fs_item in fs_items {
                    match fs_item {
                        FsItem::Directory(directory) => {
                            let fs_node = FsNode::new(directory);
                            current_directory.children.push(fs_node);
                        }
                        FsItem::File { name, size } => {
                            let mut fs_node = FsNode::new(name);
                            fs_node.size = size;
                            current_directory.children.push(fs_node);
                        }
                    }
                }
            }
        }
    }
    dbg!(current_directory);

    0
}

fn solve_second_part(input: &str) -> i32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_first_part() {
        let answer = 95437;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 42;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    // check_answers!(42, 42);
}
