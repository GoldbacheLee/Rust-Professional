/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place. 
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};


pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    let rows = matrix.len();
    if rows == 0 {
        return;
    }
    let cols = matrix[0].len();
    if cols == 0 {
        return;
    }
    
    if rows == cols {
        // 针对方阵，采用原地旋转方法：分层交换
        for layer in 0..(rows / 2) {
            let first = layer;
            let last = rows - 1 - layer;
            for i in first..last {
                let offset = i - first;
                // 保存顶部元素
                let top = matrix[first][i];
                // 左侧元素移动到顶部
                matrix[first][i] = matrix[last - offset][first];
                // 底部元素移动到左侧
                matrix[last - offset][first] = matrix[last][last - offset];
                // 右侧元素移动到底部
                matrix[last][last - offset] = matrix[i][last];
                // 顶部元素移动到右侧
                matrix[i][last] = top;
            }
        }
    } else {
        // 非方阵，构造旋转后的新矩阵
        // 新矩阵的行数等于原矩阵的列数，列数等于原矩阵的行数
        let mut new_matrix: Vec<Vec<i32>> = vec![vec![0; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                new_matrix[j][rows - 1 - i] = matrix[i][j];
            }
        }
        *matrix = new_matrix;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![3, 1],
            vec![4, 2],
        ]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![
            vec![1],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![1],
        ]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![5, 3, 1],
            vec![6, 4, 2],
        ]);
    }
}
