use std::{cell::RefCell, collections::HashMap, fmt, fs, rc::Rc};

#[derive(Default)]
struct Node {
    size: usize,
    children: HashMap<String, NodeHandle>,
    parent: Option<NodeHandle>,
}

type NodeHandle = Rc<RefCell<Node>>;

struct PrettyNode<'a>(&'a NodeHandle);

impl<'a> fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();

        if this.size == 0 {
            writeln!(f, "(dir)")?;
        } else {
            writeln!(f, "(file, size={})", this.size)?;
        }

        for (name, child) in &this.children {
            for (index, line) in format!("{:?}", PrettyNode(child)).lines().enumerate() {
                if index == 0 {
                    writeln!(f, "{name} {line}")?;
                } else {
                    writeln!(f, " {line}")?;
                }
            }
        }

        Ok(())
    }
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    fn total_size(&self) -> u64 {
        self.children
            .values()
            .map(|child| child.borrow().total_size())
            .sum::<u64>()
            + self.size as u64
    }
}

fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    #[allow(clippy::needless_collect)]
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

    Box::new(
        std::iter::once(n).chain(
            children
                .into_iter()
                .filter_map(|child| {
                    if child.borrow().is_dir() {
                        Some(all_dirs(child))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}

fn main() {
    let input = fs::read_to_string("src/day7/input.txt").unwrap();

    let root = Rc::new(RefCell::new(Node::default()));

    let mut current = root.clone();

    input.lines().for_each(|line| {
        let words: Vec<&str> = line.split(' ').collect();
        match words[0] {
            "$" => match words[1] {
                "cd" => match words[2] {
                    ".." => {
                        let parent = current.borrow().parent.clone().unwrap();
                        current = parent;
                    }
                    "/" => (),
                    _ => {
                        let child = current
                            .borrow_mut()
                            .children
                            .entry(words[2].into())
                            .or_default()
                            .clone();
                        current = child;
                    }
                },
                "ls" => (),

                _ => (),
            },
            entry => {
                if entry == "dir" {
                    let dir = current
                        .borrow_mut()
                        .children
                        .entry(words[1].into())
                        .or_default()
                        .clone();
                    dir.borrow_mut().parent = Some(current.clone());
                } else {
                    let size = entry.parse::<usize>().unwrap();
                    let file = current
                        .borrow_mut()
                        .children
                        .entry(words[1].into())
                        .or_default()
                        .clone();
                    file.borrow_mut().size = size;
                    file.borrow_mut().parent = Some(current.clone());
                }
            }
        }
    });

    // println!("{:#?}", PrettyNode(&root));

    let sum = all_dirs(root.clone())
        .map(|d| d.borrow().total_size())
        .filter(|&s| s <= 100_000)
        .inspect(|s| {
            dbg!(s);
        })
        .sum::<u64>();

    dbg!(sum);

    let total_space = 70_000_000_u64;
    let used_space = root.borrow().total_size();
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_space = 30_000_000_u64;
    let minimum_space_to_free = needed_space.checked_sub(free_space).unwrap();

    let removed_dir_size = all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s >= minimum_space_to_free)
        .inspect(|s| {
            dbg!(s);
        })
        .min();

    dbg!(removed_dir_size);
}
