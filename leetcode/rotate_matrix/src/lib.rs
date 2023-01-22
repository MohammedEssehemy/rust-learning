use std::collections::HashMap;
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let max_index = matrix.len() - 1;
    let mut replaced_values = HashMap::new();
    for i in 0..=max_index {
        for j in 0..=max_index {
            let new_x = j;
            let new_y = max_index - i;
            replaced_values
                .entry(format!("{new_x}_{new_y}"))
                .or_insert(matrix[new_x][new_y]);
            matrix[new_x][new_y] = *replaced_values
                .get(&format!("{i}_{j}"))
                .unwrap_or(&matrix[i][j]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let output = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        rotate(&mut matrix);
        assert_eq!(matrix, output);
    }

    #[test]
    fn example_2() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            //(0, 0) => (0, 3) x1, y1 => y1, 3-x1
            //(0, 1) => (1, 3)
            //(0, 2) => (2, 3)
            //(0, 3) => (3, 3)
            vec![2, 4, 8, 10],
            //(1, 0) => (0, 2)
            //(1, 1) => (1, 2)
            //(1, 2) => (2, 2)
            //(1, 3) => (3, 2)
            vec![13, 3, 6, 7],
            //(2, 0) => (0, 1)
            //(2, 1) => (1, 1)
            //(2, 2) => (2, 1)
            //(2, 3) => (3, 1)
            vec![15, 14, 12, 16],
            //(3, 0) => (0, 0)
            //(3, 1) => (1, 0)
            //(3, 2) => (2, 0)
            //(3, 3) => (3, 0)
        ];
        let output = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        rotate(&mut matrix);
        assert_eq!(matrix, output);
    }
}
