use std::collections::HashSet;

fn all_different(chunk: &[char]) -> bool {
    chunk.iter().collect::<HashSet<_>>().len() == chunk.len()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    // find the 0-based index of the first 4-char chunk that has four different chars in it
    // then add 4 (for the chunk itself)
    input.chars().collect::<Vec<_>>().windows(4).enumerate().find(|(_, val)| all_different(val)).unwrap().0 + 4
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    // same as before, but 14
    input.chars().collect::<Vec<_>>().windows(14).enumerate().find(|(_, val)| all_different(val)).unwrap().0 + 14
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}