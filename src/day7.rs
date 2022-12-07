use std::collections::HashMap;

#[aoc_generator(day7)]
pub fn input_generator_part1(input: &str) -> HashMap<String, usize> {
    let mut file_only_sizes: HashMap<String, usize> = HashMap::new();
    let mut subdirectory_names: HashMap<String, Vec<String>> = HashMap::new();
    // the current directory we're in - use "" for the root, instead of "/", to avoid concatenation weirdness
    let mut current_path = String::from("");
    let mut lines = input.lines();
    let mut all_directories = vec![String::from("")];
    while let Some(line) = lines.next() {
        if line == "$ ls" {
            continue;
        }
        // if we're cd-ing somewhere, then we need to adjust our current location
        if let Some(new_location) = line.strip_prefix("$ cd ") {
            if new_location == ".." {
                // up a directory - pop off the last path component
                let (new_path, _) = current_path.rsplit_once("/").unwrap();
                current_path = new_path.to_owned();
            } else if new_location == "/" {
                // back to the root - replace everything
                current_path = String::from("");
            } else {
                current_path = current_path + "/" + new_location;
                // remember that we've visited this directory
                all_directories.push(current_path.clone())
            }
            continue;
        }

        // otherwise this is the output from ls
        if let Some((left, right)) = line.split_once(" ") {
            if left == "dir" {
                // this directory has a subdirectory - remember the (full) path of it
                subdirectory_names.entry(current_path.clone()).or_insert(Vec::new()).push(current_path.clone() + "/" + right);
            } else {
                // this directory contains a file - add its size to the running total for this dir
                *file_only_sizes.entry(current_path.clone()).or_insert(0) += left.parse::<usize>().unwrap();
            }
        }
    }

    all_directories.into_iter().map(|d| (d.clone(), calculate_full_size(&d, &file_only_sizes, &subdirectory_names))).collect()
}

// could memoize, but why bother
pub fn calculate_full_size(directory: &str, filesizes: &HashMap<String, usize>, directories: &HashMap<String, Vec<String>>) -> usize {
    // full size of a directory is the size of all files in it (if any), plus the recursively-calculated size of all its subdirectories
    let size_of_files = filesizes.get(directory).unwrap_or(&0);
    let size_of_subdirectories: usize = directories.get(directory).unwrap_or(&Vec::new()).iter().map(|d| calculate_full_size(d, &filesizes, &directories)).sum();
    size_of_files + size_of_subdirectories
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
    *input.values().filter(|&v| v >= &extra_space_needed).min().unwrap()
}

#[test]
fn test_input_parsing() {
    let input =
r#"
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