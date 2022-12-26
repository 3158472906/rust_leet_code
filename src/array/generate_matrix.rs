//59. 螺旋矩阵 II
// 中等
// 885
// 相关企业
// 给你一个正整数 n ，生成一个包含 1 到 n2 所有元素，且元素按顺时针顺序螺旋排列的 n x n 正方形矩阵 matrix 。
//
//
//
// 示例 1：
//
//
// 输入：n = 3
// 输出：[[1,2,3],[8,9,4],[7,6,5]]
// 示例 2：
//
// 输入：n = 1
// 输出：[[1]]
//
//
// 提示：
//
// 1 <= n <= 20

struct Solution;


impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let (
            mut left, mut right,
            mut up, mut down,
            mut res,
            mut number, n
        ) = (
            0 as usize, (n - 1) as usize,
            0 as usize, (n - 1) as usize,
            vec![vec![0; n as usize]; n as usize],
            1,
            n * n,
        );

        while number <= n {
            for i in left..=right {
                res[up][i] = number;
                number += 1;
            }
            up += 1;

            for i in up..=down {
                res[i][right] = number;
                number += 1;
            }
            right -= 1;

            let mut i = right as isize;
            while i >= left as isize {
                res[down][i as usize] = number;
                number += 1;
                i -= 1;
            }
            down -= 1;

            let mut i = down as isize;
            while i >= up as isize {
                res[i as usize][left] = number;
                number += 1;
                i -= 1;
            }
            left += 1;
        }

        res
    }
}

#[test]
fn test_soution() {
    println!("{:?}", Solution::generate_matrix(1));
}


#[test]
fn test_for() {
    for i in 10..1 {
        println!("{}", i);
    }
}