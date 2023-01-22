use std::collections::HashSet;
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    const BOARD_SIZE: usize = 9;
    const BOX_SIZE: usize = 3;
    // validate rows
    for row_no in 0..BOARD_SIZE {
        if has_repeat(&board[row_no]) {
            return false;
        }
    }

    for column_no in 0..BOARD_SIZE {
        let column = board.iter().map(|row| row[column_no]).collect();
        if has_repeat(&column) {
            return false;
        }
    }

    for box_no in 0..BOARD_SIZE {
        let row_start = (box_no / BOX_SIZE) * BOX_SIZE;
        let column_start = (box_no % BOX_SIZE) * BOX_SIZE;
        let box_chars = board
            .iter()
            .skip(row_start)
            .take(BOX_SIZE)
            .flat_map(|row| row.iter().skip(column_start).take(BOX_SIZE).copied())
            .collect();
        if has_repeat(&box_chars) {
            return false;
        }
    }

    true
}

fn has_repeat(digits: &Vec<char>) -> bool {
    let filled_digits = digits.iter().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
    filled_digits.len() != filled_digits.iter().collect::<HashSet<_>>().len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_valid() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(is_valid_sudoku(board));
    }

    #[test]
    fn row_invalid() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '4', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!is_valid_sudoku(board));
    }

    #[test]
    fn column_invalid() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['5', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!is_valid_sudoku(board));
    }

    #[test]
    fn box_invalid() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '5', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!is_valid_sudoku(board));
    }

    #[test]
    fn box_valid() {
        let board = vec![
            vec!['.', '9', '.', '.', '4', '.', '.', '.', '.'],
            vec!['1', '.', '.', '.', '.', '.', '6', '.', '.'],
            vec!['.', '.', '3', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
            vec!['3', '.', '.', '.', '5', '.', '.', '.', '.'],
            vec!['.', '.', '7', '.', '.', '4', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '7', '.', '.', '.', '.'],
        ];
        assert!(is_valid_sudoku(board));
    }
}
