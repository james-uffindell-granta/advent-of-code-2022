use std::collections::HashMap;

pub struct Directory {
    size_of_files: usize,
    // fully-qualified
    subdirectory_names: Vec<String>,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            size_of_files: 0,
            subdirectory_names: Vec::new(),
        }
    }

    pub fn total_size(&self, directory_tree: &HashMap<String, Directory>) -> usize {
        self.subdirectory_names
            .iter()
            .map(|d| directory_tree.get(d).unwrap().total_size(directory_tree))
            .sum::<usize>()
            + self.size_of_files
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self::new()
    }
}

#[aoc_generator(day7)]
pub fn input_generator_part1(input: &str) -> HashMap<String, usize> {
    let mut directories: HashMap<String, Directory> =
        HashMap::from([(String::from(""), Directory::new())]);
    // the current directory we're in - use "" for the root, instead of "/", to avoid concatenation weirdness
    let mut current_path = String::from("");
    for line in input.lines() {
        if line == "$ ls" {
            continue;
        }
        // if we're cd-ing somewhere, then we need to adjust our current location
        if let Some(new_location) = line.strip_prefix("$ cd ") {
            if new_location == ".." {
                // up a directory - pop off the last path component
                let (new_path, _) = current_path.rsplit_once('/').unwrap();
                current_path = new_path.to_owned();
            } else if new_location == "/" {
                // back to the root - replace everything
                current_path = String::from("");
            } else {
                current_path = current_path + "/" + new_location;
                // remember that we've visited this directory, if we haven't seen it before
                directories
                    .entry(current_path.clone())
                    .or_insert(Directory::new());
            }
            continue;
        }

        // otherwise this is the output from ls
        if let Some((left, right)) = line.split_once(' ') {
            let mut current_directory = directories.get_mut(&current_path).unwrap();
            if left == "dir" {
                // this directory has a subdirectory - remember the (full) path of it
                current_directory
                    .subdirectory_names
                    .push(current_path.clone() + "/" + right);
            } else {
                // this directory contains a file - add its size to the running total for this dir
                current_directory.size_of_files += left.parse::<usize>().unwrap();
            }
        }
    }

    directories
        .iter()
        .map(|(name, d)| (name.clone(), d.total_size(&directories)))
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &HashMap<String, usize>) -> usize {
    input.values().filter(|&v| v <= &100_000).sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &HashMap<String, usize>) -> usize {
    let root_directory_size = input.get("").unwrap();
    let already_free_size = 70_000_000 - root_directory_size;
    let extra_space_needed = 30_000_000 - already_free_size;
    *input
        .values()
        .filter(|&v| v >= &extra_space_needed)
        .min()
        .unwrap()
}

#[test]
fn test_input_parsing() {
    let input = r#"
$ cd /
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
7214296 k
"#;

    let parsed_input = input_generator_part1(input);
    let dir_total = solve_part1(&parsed_input);

    assert_eq!(dir_total, 95437);

    let dir_to_delete = solve_part2(&parsed_input);

    assert_eq!(dir_to_delete, 24_933_642);
}
