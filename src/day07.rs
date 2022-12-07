use core::cell::Cell;
use std::io::Read;

struct Node {
    name: Vec<u8>,
    size: Cell<u32>,
    value: NodeType
}

struct Dir {
    entries: Vec<usize>,
    parent: usize
}

enum NodeType {
    File,
    Dir(Dir)
}

pub struct FileSystem {
    current_dir: usize,
    nodes: Vec<Node>
}

impl FileSystem {
    fn new() -> FileSystem {
        let mut fs = FileSystem {
            current_dir: 0,
            nodes: Vec::with_capacity(1000)
        };
        fs.add_node(Node {
            name: b"/".to_vec(),
            size: Default::default(),
            value: NodeType::Dir(Dir {
                entries: Vec::new(),
                parent: 0
            })
        });
        fs
    }
    fn add_node(&mut self, n: Node) -> usize {
        let is_root = n.name == b"/";
        let index = self.nodes.len();
        self.nodes.push(n);
        if !is_root {
            if let NodeType::Dir(dir) = &mut self.nodes[self.current_dir].value {
                dir.entries.push(index);
            }
        }
        return index
    }

    fn cd(&mut self, name: &[u8]) {
        self.current_dir = if name == b"/" {
            0
        } else {
            if let NodeType::Dir(dir) = &self.nodes[self.current_dir].value {
                if name == b".." {
                    dir.parent
                } else {
                    *dir.entries.iter().filter(|&&id| self.nodes[id].name == name).next().unwrap()
                }
            } else {
                unreachable!()
            }
        }
    }

    fn add_dir(&mut self, name: &[u8]) {
        self.add_node(Node {
            name: name.to_vec(),
            size: Default::default(),
            value: NodeType::Dir(Dir {
                entries: Vec::new(),
                parent: self.current_dir
            })
        });
    }

    fn add_file(&mut self, name: &[u8], size: u32) {
        self.add_node(Node {
            name: name.to_vec(),
            size: Cell::new(size),
            value: NodeType::File
        });
    }

    fn update_dir_sizes(&self, current_dir: usize) {
        match &self.nodes[current_dir].value {
            NodeType::Dir(dir) => {
                let mut size = 0;
                for entry in &dir.entries {
                    self.update_dir_sizes(*entry);
                    size += self.nodes[*entry].size.get();
                }
                self.nodes[current_dir].size.set(size)
            },
            _ => ()
        }
    }

    fn iter_dirs(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter().filter(|f| match f.value {
            NodeType::Dir(_) => true,
            _ => false
        })
    }

    fn total_size(&self) -> u32 {
        self.nodes[0].size.get()
    }
}
pub fn parse_fs(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();
    input.lines().for_each(|l| {
        match l.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["$", "cd", s]=> fs.cd(s.as_bytes()),
            ["$", _] => (),
            ["dir", d] => fs.add_dir(d.as_bytes()),
            [s, name] => fs.add_file(name.as_bytes(), s.parse().unwrap()),
            _ => unreachable!()
        }
    });
    fs
}

fn parse<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<u32> {
    let (mut size, mut subdirs) = (0, vec![]);
    loop {
        match input.next().map(|l| l.split_whitespace().collect::<Vec<_>>()).as_deref() {
            Some(["$", "cd", ".."]) | None => break,
            Some(["$", "cd", s]) if *s != "/" => {
                subdirs.extend(parse(input));
                size += subdirs.last().unwrap()
            }
            Some([s, _]) if *s != "$" && *s != "dir" => {
                size += s.parse::<u32>().unwrap();
            }
            _ => (),
        }
    }
    subdirs.push(size);
    subdirs
}


#[aoc_generator(day7)]
pub fn generator(input: &str) -> String {
    input.to_string()
}

#[aoc(day7, part1)]
pub fn part1(inputs: &str) -> u32 {
    parse(&mut inputs.lines())
        .into_iter()
        .filter(|&size| size < 100_000)
        .sum()
}

#[aoc(day7, part1, tree)]
pub fn part1_true(inputs: &str) -> u32 {
    let fs = parse_fs(inputs);
    fs.update_dir_sizes(0);
    fs.iter_dirs().map(|n| n.size.get()).filter(|&s| s < 100_000).sum()
}

#[aoc(day7, part2)]
pub fn part2(inputs: &str) -> u32 {
    let dirsizes = parse(&mut inputs.lines());
    let missing = 30_000_000 - (70_000_000 - dirsizes.last().unwrap());
    dirsizes.into_iter().filter(|&size| size >= missing).min().unwrap()
}

#[aoc(day7, part2, tree)]
pub fn part2_true(inputs: &str) -> u32 {
    let fs = parse_fs(inputs);
    fs.update_dir_sizes(0);
    let missing = 30_000_000 - (70_000_000 - fs.total_size());
    fs.iter_dirs().map(|n| n.size.get()).filter(|&s| s >= missing).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "$ cd /
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
    pub fn test1() {
        assert_eq!(part1(&generator(&SAMPLE)), 95437)
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(&SAMPLE)), 24933642)
    }
}
