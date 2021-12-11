use std::cmp;

fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut costs = vec![vec![0_i32; n]; m];
    costs[0][0] = grid[0][0];

    for i in 1..m {
        costs[i][0] = grid[i][0] + costs[i - 1][0];
    }
    for j in 1..n {
        costs[0][j] = grid[0][j] + costs[0][j - 1];
    }

    for i in 1..m {
        for j in 1..n {
            costs[i][j] = grid[i][j] + cmp::min(costs[i - 1][j], costs[i][j - 1]);
        }
    }

    costs[m - 1][n - 1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(min_path_sum(input), 7);
    }
}
