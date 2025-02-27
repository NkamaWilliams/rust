



use core::num;
use std::ops::Add;
use std::result;
use std::collections::{HashMap, HashSet};

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results: Vec<Vec<i32>> = Vec::new();

    fn backtrack(start: usize, path:&mut Vec<i32>, nums: &Vec<i32>, results: &mut Vec<Vec<i32>>) {
        results.push(path[..].to_vec());
        for i in start..nums.len(){
            path.push(nums[i]);
            backtrack(i+1, path, nums, results);
            path.pop();
        }
    }

    let mut path = Vec::new();
    backtrack(0, &mut path, &nums, &mut results);
    results
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ans = Vec::new();
    let mut nums = Vec::new();
    fn backtrack(start: usize, total: i32, target: i32, nums: &mut Vec<i32>, candidates: &Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if total == target {
            ans.push(nums[..].to_vec());
            return;
        }
        else if total > target{
            return;
        }
        else {
            for i in start..candidates.len(){
                nums.push(candidates[i]);
                backtrack(i, total + candidates[i], target, nums, candidates, ans);
                nums.pop();
            }
        }
    }

    backtrack(0, 0, target, &mut nums, &candidates, &mut ans);
    ans
}

pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    candidates.sort();
    let mut results = Vec::new();
    fn backtrack(start: usize, total: i32, target: i32, results: &mut Vec<Vec<i32>>, candidates: &Vec<i32>, nums: &mut Vec<i32>){
        if total > target {
            return;
        } else if total == target {
            results.push(nums[..].to_vec());
            return;
        } else {
            for i in start..candidates.len(){
                if candidates[i] > target{
                    break;
                }
                if i > start && candidates[i] == candidates[i-1]{
                    continue;
                }
                nums.push(candidates[i]);
                backtrack(i+1, total + candidates[i], target, results, candidates, nums);
                nums.pop();
            }
        }
    }
    backtrack(0, 0, target, &mut results, &candidates, &mut Vec::new());
    return results;
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results = Vec::new();
    let mut visited = HashSet::new();
    fn backtrack(nums: &Vec<i32>, results: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, visited: &mut HashSet<i32>) {
        if path.len() == nums.len(){
            results.push(path[..].to_vec());
            return;
        }

        else {
            for num in nums{
                if visited.contains(num){
                    continue;
                }
                path.push(*num);
                visited.insert(*num);
                backtrack(nums, results, path, visited);
                path.pop();
                visited.remove(num);
            }
        }
    }

    backtrack(&nums, &mut results, &mut Vec::new(), &mut visited);
    results
}
struct Solution();
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut results = Vec::new();
        fn backtrack(start: usize, path: &mut Vec<i32>, nums: &Vec<i32>, results: &mut Vec<Vec<i32>>) {
            results.push(path[..].to_vec());

            for i in start..nums.len(){
                if i > start && nums[i] == nums[i-1]{
                    continue;
                }
                path.push(nums[i]);
                backtrack(i+1, path, nums, results);
                path.pop();
            }
        }
        backtrack(0, &mut vec![], &nums, &mut results);
        results
    }

    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        fn backtrack(row: i32, col: i32, index: usize, board: &mut Vec<Vec<char>>, word: &Vec<char>) -> bool {
            let cur_char = board[row as usize][col as usize];
            if index == word.len(){
                return true;
            }
            else if !(0 <= row && row < board.len() as i32) || !(0 <= col && col < board[0].len() as i32) || word[index] != cur_char {
                return false;
            }

            let temp = board[row as usize][col as usize];
            board[row as usize][col as usize] = '#';

            for (x, y) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if backtrack(row.saturating_add(x), col.saturating_add(y), index + 1, board, word){
                    return true;
                }
            }
            board[row as usize][col as usize] = temp;
            false
        }
        let word: Vec<char> = word.chars().collect();
        for i in 0..board.len(){
            for j in 0..board[0].len(){
                if board[i][j] == word[0] && backtrack(i as i32, j as i32, 0, &mut board, &word){
                    return true;
                }
            }
        }
        false
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut results = vec![];

        fn isPalindrome(s: &str) -> bool{
            s.chars().eq(s.chars().rev())
        }

        fn backtrack(start: usize, s: &str, path: &mut Vec<String>, results:&mut Vec<Vec<String>>){
            if start == s.len(){
                results.push(path[..].to_vec());
                return;
            }

            for i in start + 1..s.len() + 1{
                let word = s.get(start..i).unwrap();
                if isPalindrome(word){
                    path.push(word.into());
                    backtrack(i, s.get(start..i).unwrap().into(), path, results);
                    path.pop();
                }
            }
        }
        backtrack(0, &s.as_str(), &mut vec![], &mut results);
        results
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut mapping: HashMap<String, Vec<char>> = HashMap::new();
        mapping.insert("2".into(), vec!['a', 'b', 'c']);
        mapping.insert("3".into(), vec!['d', 'e', 'f']);
        mapping.insert("4".into(), vec!['g', 'h', 'i']);
        mapping.insert("5".into(), vec!['j', 'k', 'l']);
        mapping.insert("6".into(), vec!['m', 'n', 'o']);
        mapping.insert("7".into(), vec!['p', 'q', 'r', 's']);
        mapping.insert("8".into(), vec!['t', 'u', 'v']);
        mapping.insert("9".into(), vec!['w', 'x', 'y', 'z']);

        let mut results = Vec::new();
        let digits: Vec<char> = digits.chars().collect();

        fn backtrack(start: usize, path: &mut Vec<char>, digits: &Vec<char>, results: &mut Vec<String>, mapping: &HashMap<String, Vec<char>>){
            if start == digits.len(){
                results.push(path.iter().collect());
                return;
            }
            let key: String = digits[start].into();
            if let Some(characters) = mapping.get(&key){
                for &chr in characters{
                    path.push(chr);
                    backtrack(start + 1, path, digits, results, mapping);
                    path.pop();
                }
            }
        }
        backtrack(0, &mut Vec::new(), &digits, &mut results, &mapping);
        results
    }
}