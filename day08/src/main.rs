#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Tree {
    height: u32,
}

pub struct Grid {
    trees: Vec<Vec<Tree>>,
}

impl Grid {
    pub fn row_count(&self) -> usize {
        self.trees.len()
    }
    pub fn column_count(&self) -> usize {
        self.trees[0].len()
    }

    pub fn row(&self, index: usize) -> &Vec<Tree> {
        self.trees.get(index).unwrap()
    }

    pub fn column(&self, index: usize) -> Vec<Tree> {
        self.trees.iter().map(|r| r[index]).collect()
    }
}

fn is_tree_visible(
    (row, column): (&Vec<Tree>, &Vec<Tree>),
    (row_index, column_index): (usize, usize),
) -> bool {
    let tree_height = row[row_index].height;
    let (left, tree_onwards_right) = row.split_at(row_index);
    let (up, tree_onwards_down) = column.split_at(column_index);

    all_shorter(left, tree_height)
        || all_shorter(&tree_onwards_right[1..], tree_height)
        || all_shorter(up, tree_height)
        || all_shorter(&tree_onwards_down[1..], tree_height)
}

fn all_shorter<'a, I>(trees: I, height: u32) -> bool
where
    I: IntoIterator<Item = &'a Tree>,
{
    trees.into_iter().all(|t| t.height < height)
}

fn count_until<'a, I>(trees: I, height: u32) -> u32
where
    I: IntoIterator<Item = &'a Tree>,
{
    // take_while + 1 would be right for inner trees, but wrong for the edge - just loop instead
    let mut count = 0;
    for t in trees {
        count += 1;
        if t.height >= height {
            break;
        }
    }
    count
}

fn scenic_score(
    (row, column): (&Vec<Tree>, &Vec<Tree>),
    (row_index, column_index): (usize, usize),
) -> u32 {
    let tree_height = row[row_index].height;
    let (left, tree_onwards_right) = row.split_at(row_index);
    let (up, tree_onwards_down) = column.split_at(column_index);

    let left_distance = count_until(left.iter().rev(), tree_height);
    let right_distance = count_until(&tree_onwards_right[1..], tree_height);
    let up_distance = count_until(up.iter().rev(), tree_height);
    let down_distance = count_until(&tree_onwards_down[1..], tree_height);

    left_distance * right_distance * up_distance * down_distance
}

pub fn input_generator_part1(input: &str) -> Grid {
    Grid {
        trees: input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .map(|i| Tree { height: i })
                    .collect()
            })
            .collect(),
    }
}

pub fn solve_part1(input: &Grid) -> usize {
    (0..input.column_count())
        .map(|r| {
            let column = input.column(r);
            (0..input.row_count())
                .filter(|&c| is_tree_visible((input.row(c), &column), (r, c)))
                .count()
        })
        .sum()
}

pub fn solve_part2(input: &Grid) -> u32 {
    (0..input.column_count())
        .map(|r| {
            let column = input.column(r);
            (0..input.row_count())
                .map(|c| scenic_score((input.row(c), &column), (r, c)))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[test]
fn test_day8() {
    let input = r#"30373
25512
65332
33549
35390
"#;

    let parsed_input = input_generator_part1(input);
    let tree_total = solve_part1(&parsed_input);
    let highest_scenic_score = solve_part2(&parsed_input);

    assert_eq!(tree_total, 21);
    assert_eq!(highest_scenic_score, 8);
}

fn main() {
    let input = input_generator_part1(include_str!("../input.txt"));

    let part_1 = solve_part1(&input);
    let part_2  = solve_part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

