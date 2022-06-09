fn main() {
    println!("Hello, world!");
}

pub struct Solution;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let cur_color = image[sr as usize][sc as usize];
        if cur_color != new_color {
            Self::paint(&mut image, sr, sc, new_color, cur_color);
        }
        image
    }
    pub fn paint(image: &mut Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32, cur_color: i32) {
        if sr < 0 || sr >= image.len() as i32 {
            return;
        }
        if sc < 0 || sc >= image[0].len() as i32 {
            return;
        }
        if image[sr as usize][sc as usize] == cur_color {
            image[sr as usize][sc as usize] = new_color;
            Self::paint(image, sr - 1, sc, new_color, cur_color);
            Self::paint(image, sr + 1, sc, new_color, cur_color);
            Self::paint(image, sr, sc + 1, new_color, cur_color);
            Self::paint(image, sr, sc - 1, new_color, cur_color);
        }
    }
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len() - 1;
        let n = grid[0].len() - 1;
        let mut res = 0;
        let mut visited = vec![vec![false; n + 1]; m + 1];
        for i in 0..=m {
            for j in 0..=n {
                if grid[i][j] == 0 {
                    continue;
                }
                res = res.max(Self::dfs(&grid, (m, n), (i, j), &mut visited));
            }
        }
        res
    }
    fn dfs(
        grid: &[Vec<i32>],
        bounds: (usize, usize),
        cur: (usize, usize),
        visited: &mut [Vec<bool>],
    ) -> i32 {
        let (m, n) = bounds;
        let (x, y) = cur;
        if visited[x][y] {
            return 0;
        }
        visited[x][y] = true;
        let mut res = 1;
        if x > 0 && grid[x - 1][y] == 1 {
            res += Self::dfs(grid, bounds, (x - 1, y), visited);
        }
        if x < m && grid[x + 1][y] == 1 {
            res += Self::dfs(grid, bounds, (x + 1, y), visited);
        }
        if y > 0 && grid[x][y - 1] == 1 {
            res += Self::dfs(grid, bounds, (x, y - 1), visited);
        }
        if y < n && grid[x][y + 1] == 1 {
            res += Self::dfs(grid, bounds, (x, y + 1), visited);
        }
        res
    }
}
