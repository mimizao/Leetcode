use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut chars_col = vec![];
    let mut chars_row = vec![];
    let mut chars0 = vec![];
    let mut chars1 = vec![];
    let mut chars2 = vec![];
    for i in 0..9 {
        for j in 0..9 {
            chars_col.push(board[i][j]);
            chars_row.push(board[j][i]);
            if j / 3 == 0 {
                chars0.push(board[i][j]);
            } else if j / 3 == 1 {
                chars1.push(board[i][j]);
            } else {
                chars2.push(board[i][j]);
            }
        }
        if !is_valid_chars(&chars_col) || !is_valid_chars(&chars_row) {
            return false;
        }
        chars_col.clear();
        chars_row.clear();
        if i % 3 == 2 {
            if !is_valid_chars(&chars0) || !is_valid_chars(&chars1) || !is_valid_chars(&chars2) {
                return false;
            }
            chars0.clear();
            chars1.clear();
            chars2.clear();
        }
    }
    true
}

pub fn is_valid_chars(chars: &Vec<char>) -> bool {
    let mut map = HashMap::new();
    for i in 0..chars.len() {
        if chars[i] != '.' {
            if let Some(_) = map.get(&chars[i]) {
                return false;
            }
            map.insert(chars[i], i);
        }
    }
    true
}
