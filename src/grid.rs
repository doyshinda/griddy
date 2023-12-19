use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct UnequalColumnsError;

#[derive(Debug)]
/// A 2D Grid of values. Uses a cooridinate system where x increases going down
/// and y increases to the right.
/// # Example
/// ```
/// use griddy::Grid;
///
/// let grid = Grid::from_2d_unchecked(vec![
///     vec![1, 2],
///     vec![9, 8],
/// ]);
///
/// assert_eq!(grid[0][0], 1);
/// assert_eq!(grid[1][0], 9);
/// assert_eq!(grid[0][1], 2);
/// assert_eq!(grid[1][1], 8);
///```
pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    /// Initialize a 2D grid with `rows` number of rows, and `cols` number of columns, filling
    /// each cell with `value`.
    pub fn init(rows: usize, cols: usize, value: T) -> Grid<T>
    where
        T: Clone,
    {
        let mut grid = Vec::with_capacity(rows);
        for _ in 0..rows {
            grid.push(vec![value.clone(); cols]);
        }

        Grid { grid }
    }

    /// Initialize a Grid from a 2D vector. Returned `UnequalColumnsError` if all the rows are not
    /// the same length.
    pub fn from_2d(grid: Vec<Vec<T>>) -> Result<Grid<T>, UnequalColumnsError> {
        let mut c = grid.iter().map(|r| r.len()).collect::<Vec<usize>>();
        c.sort();
        c.dedup();
        if c.len() > 1 {
            return Err(UnequalColumnsError);
        }
        Ok(Grid { grid })
    }

    /// Initialize a Grid from a 2D vector.
    pub fn from_2d_unchecked(grid: Vec<Vec<T>>) -> Grid<T> {
        Grid { grid }
    }

    /// Insert a row at idx.
    pub fn insert_row(&mut self, idx: usize, row: Vec<T>) {
        self.grid.insert(idx, row);
    }

    /// Transpose
    pub fn transpose(&self) -> Grid<T>
    where
        T: Copy,
    {
        let mut g = Grid::init(
            self.cols_len(),
            self.rows_len(),
            self.grid[0][0]
        );
        for x in 0..self.cols_len() {
            for y in 0..self.rows_len() {
                g[x][y] = self.grid[y][x];
            }
        }
        g
    }

    pub fn print(&self)
    where
        T: std::fmt::Debug
    {
        for r in self.rows() {
            println!("{:?}", r);
        }
    }

    pub fn rotate(&mut self)
    where
        T: Copy
    {
        let mut temp = vec![];

        for column in 0..self.rows_len() {
            let mut t = vec![];
            for row in (0..self.rows_len()).rev() {
                t.push(self.grid[row][column]);
            }
            temp.push(t);
        }

        for i in 0..self.rows_len() {
            for j in 0..self.rows_len() {
                self.grid[i][j] = temp[i][j];
            }
        }
    }

    pub fn flip_y(&mut self) {
        for r in self.rows_mut() {
            r.reverse();
        }
    }

    /// The number of rows.
    pub fn rows_len(&self) -> usize {
        self.grid.len()
    }

    /// The number of columns.
    pub fn cols_len(&self) -> usize {
        match self.grid.len() {
            0 => 0,
            _ => self.grid[0].len(),
        }
    }

    /// Returns all the coordinates to the left (x decreases) of the coordinate `(row, col)`.
    pub fn row_left_coords(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut n = Vec::new();
        if row >= self.rows_len() || col >= self.cols_len() {
            return n;
        }

        for i in 0..col {
            n.push((row, i));
        }

        n
    }

    /// Returns all the coordinates to the right (x increases) of the coordinate `(row, col)`.
    pub fn row_right_coords(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut n = Vec::new();
        if row >= self.rows_len() || col >= self.cols_len() {
            return n;
        }

        for i in col+1..self.cols_len() {
            n.push((row, i));
        }

        n
    }

    /// Returns all the coordinates above (y decreases) of the coordinate `(row, col)`.
    pub fn col_up_coords(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut n = Vec::new();
        if row >= self.rows_len() || col >= self.cols_len() {
            return n;
        }

        for i in 0..row {
            n.push((i, col));
        }

        n
    }

    /// Returns all the coordinates below (y increases) of the coordinate `(row, col)`.
    pub fn col_down_coords(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut n = Vec::new();
        if row >= self.rows_len() || col >= self.cols_len() {
            return n;
        }

        for i in row+1..self.rows_len() {
            n.push((i, col));
        }

        n
    }

    /// Returns the coordinates to the left (x decreases) and right (x increases) of
    /// of the coordinate `(row, col)`.
    pub fn row_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        if row >= self.rows_len() || col >= self.cols_len() {
            return vec![];
        }

        let mut n = Vec::new();
        if let Some(left) = col.checked_sub(1) {
            n.push((row, left));
        }

        let right = col + 1;
        if right < self.cols_len() {
            n.push((row, right));
        }

        n
    }

    /// Returns the coordinates to the bottom (y increases) and top (y decreases) of
    /// of the coordinate `(row, col)`.
    pub fn col_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        if row >= self.rows_len() || col >= self.cols_len() {
            return vec![];
        }

        let mut n = Vec::new();
        if let Some(up) = row.checked_sub(1) {
            n.push((up, col));
        }

        if row + 1 < self.rows_len() {
            n.push((row + 1, col));
        }

        n
    }

    /// Returns the coordinates of up to 4 diagonal points of the coordinate `(row, col)`.
    pub fn diag_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        if row >= self.rows_len() || col >= self.cols_len() {
            return vec![];
        }

        let mut n = Vec::new();
        if let Some(up) = row.checked_sub(1) {
            if let Some(left) = col.checked_sub(1) {
                n.push((up, left));
            }

            if col + 1 < self.cols_len() {
                n.push((up, col + 1));
            }
        }

        let down = row + 1;
        if down < self.rows_len() {
            if let Some(left) = col.checked_sub(1) {
                n.push((down, left));
            }

            if col + 1 < self.cols_len() {
                n.push((down, col + 1));
            }
        }

        n
    }

    /// Returns all valid coordinates surrounding the coordinate `(row, col)`.
    pub fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut n = self.row_neighbors(row, col);
        n.append(&mut self.col_neighbors(row, col));
        n.append(&mut self.diag_neighbors(row, col));
        n
    }

    /// Truncate the number of rows to `size`
    pub fn truncate_rows(&mut self, size: usize) {
        self.grid.truncate(size);
    }

    /// Return an iter over the rows
    pub fn rows(&self) -> std::slice::Iter<'_, Vec<T>> {
        self.grid.iter()
    }

    /// Return a mut iter over the rows
    pub fn rows_mut(&mut self) -> std::slice::IterMut<'_, Vec<T>> {
        self.grid.iter_mut()
    }

    /// Fold the 2d grid "up" at `row`. Takes a closure that passes in a reference to the `new`
    /// and `old` grid values, where the returned value will be written to the `new` position.
    /// For example, if you have a 3x3 grid like this:
    /// ```text
    /// 0, 0, 0
    /// 0, 0, 0
    /// 1, 2, 3
    /// ```
    /// Folding at row 1 would transform the grid to this (assuming your closure overwrites the new
    /// location with the old value):
    /// ```text
    /// 1, 2, 3
    /// ```
    pub fn fold_at_row<F>(&mut self, row: usize, mut f: F) -> usize
    where
        F: FnMut(&T, &T) -> T,
    {
        let mut new_y = (0..row).rev();
        for y in (row + 1)..self.rows_len() {
            if let Some(new_y_coord) = new_y.next() {
                for x in 0..self.cols_len() {
                    self.grid[new_y_coord][x] = f(&self.grid[new_y_coord][x], &self.grid[y][x]);
                }
            } else {
                break;
            }
        }
        self.truncate_rows(row);
        self.rows_len()
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, idx: usize) -> &Self::Output {
        if idx >= self.grid.len() {
            panic!(
                "index {:?} out of bounds. Grid has {:?} rows.",
                self.grid.len(), idx
            );
        }

        &self.grid[idx]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        if idx >= self.grid.len() {
            panic!(
                "index {:?} out of bounds. Grid has {:?} rows.",
                self.grid.len(), idx
            );
        }

        &mut self.grid[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let grid = Grid::init(10, 8, 0);
        assert_eq!(grid.rows_len(), 10);
        assert_eq!(grid.cols_len(), 8);
    }

    #[test]
    fn index() {
        let grid = Grid::init(10, 8, 0);
        assert_eq!(grid[0], vec![0; 8]);
        assert_eq!(grid[1][0], 0);
    }

    #[test]
    fn index_mut() {
        let mut grid = Grid::init(10, 8, 0);
        grid[0][0] = 1;
        assert_eq!(grid[0][0], 1);
    }

    #[test]
    fn row_neighbors() {
        let grid = Grid::init(5, 5, 0);
        let rn = grid.row_neighbors(0, 5);
        assert_eq!(rn, []);

        let rn = grid.row_neighbors(5, 0);
        assert_eq!(rn, []);

        let rn = grid.row_neighbors(0, 0);
        assert_eq!(rn, vec![(0, 1)]);

        let rn = grid.row_neighbors(0, 1);
        assert_eq!(rn, vec![(0, 0), (0, 2)]);

        let rn = grid.row_neighbors(0, 4);
        assert_eq!(rn, vec![(0, 3)]);

        let rn = grid.row_neighbors(4, 0);
        assert_eq!(rn, vec![(4, 1)]);
    }

    #[test]
    fn col_neighbors() {
        let grid = Grid::init(5, 5, 0);

        let cn = grid.col_neighbors(0, 5);
        assert_eq!(cn, []);

        let cn = grid.col_neighbors(5, 0);
        assert_eq!(cn, []);

        let cn = grid.col_neighbors(0, 0);
        assert_eq!(cn, vec![(1, 0)]);

        let cn = grid.col_neighbors(1, 1);
        assert_eq!(cn, vec![(0, 1), (2, 1)]);
    }

    #[test]
    fn diag_neighbors() {
        let grid = Grid::init(5, 5, 0);

        let dn = grid.diag_neighbors(0, 5);
        assert_eq!(dn, []);

        let dn = grid.diag_neighbors(5, 0);
        assert_eq!(dn, []);

        let dn = grid.diag_neighbors(4, 0);
        assert_eq!(dn, [(3, 1)]);

        let dn = grid.diag_neighbors(4, 1);
        assert_eq!(dn, [(3, 0), (3, 2)]);

        let dn = grid.diag_neighbors(4, 1);
        assert_eq!(dn, [(3, 0), (3, 2)]);

        let dn = grid.diag_neighbors(2, 2);
        assert_eq!(dn, [(1, 1), (1, 3), (3, 1), (3, 3)]);
    }

    #[test]
    fn neighbors() {
        let grid = Grid::init(5, 5, 0);

        let n = grid.neighbors(0, 5);
        assert_eq!(n, []);

        let n = grid.neighbors(5, 0);
        assert_eq!(n, []);

        let n = grid.neighbors(2, 2);
        assert_eq!(n, [(2, 1), (2, 3), (1, 2), (3, 2), (1, 1), (1, 3), (3, 1), (3, 3)]);
    }

    #[test]
    fn fold_at_row_zero() {
        let mut grid = Grid::init(7, 10, 0);
        let rl = grid.fold_at_row(0, |new, old| new + old);
        assert_eq!(rl, 0);
    }

    #[test]
    fn fold_at_row_before_middle() {
        let num_rows = 7;
        let num_cols = 10;
        let mut grid = Grid::init(num_rows, num_cols, 0);
        grid[6] = vec![1; num_cols];
        

        let fold_row = (num_rows / 2) - 1;
        let rl = grid.fold_at_row(fold_row, |new, old| new + old);
        assert_eq!(rl, 2);
        assert_eq!(grid[0], vec![0; num_cols]);
        assert_eq!(grid[1], vec![0; num_cols]);
    }

    #[test]
    fn fold_at_row_middle() {
        let num_rows = 7;
        let num_cols = 10;
        let mut grid = Grid::init(num_rows, num_cols, 0);
        grid[6] = vec![1; num_cols];
        

        let fold_row = num_rows / 2;
        grid.fold_at_row(fold_row, |new, old| new + old);
        assert_eq!(grid.rows_len(), 3);
        assert_eq!(grid[0], vec![1; num_cols]);
    }

    #[test]
    fn fold_at_row_past_middle() {
        let num_rows = 7;
        let num_cols = 10;
        let mut grid = Grid::init(num_rows, num_cols, 0);
        grid[6] = vec![1; num_cols];
        

        let fold_row = (num_rows / 2) + 1;
        grid.fold_at_row(fold_row, |new, old| new + old);
        assert_eq!(grid.rows_len(), 4);
        assert_eq!(grid[0], vec![0; num_cols]);
        assert_eq!(grid[1], vec![0; num_cols]);
        assert_eq!(grid[2], vec![1; num_cols]);
    }

    #[test]
    fn fold_at_row_last() {
        let mut grid = Grid::init(7, 10, 0);
        let rl = grid.fold_at_row(6, |new, old| new + old);
        assert_eq!(rl, 6);
    }

    #[test]
    fn row_left_coords() {
        let grid = Grid::init(5, 5, 0);
        let rlc = grid.row_left_coords(0, 5);
        assert_eq!(rlc, []);

        let rlc = grid.row_left_coords(0, 1);
        assert_eq!(rlc, [(0, 0)]);

        let rlc = grid.row_left_coords(0, 2);
        assert_eq!(rlc, [(0, 0), (0, 1)]);

        let rlc = grid.row_left_coords(0, 3);
        assert_eq!(rlc, [(0, 0), (0, 1), (0, 2)]);
    }

    #[test]
    fn row_right_coords() {
        let grid = Grid::init(5, 5, 0);
        let rrc = grid.row_right_coords(0, 5);
        assert_eq!(rrc, []);

        let rrc = grid.row_right_coords(0, 1);
        assert_eq!(rrc, [(0, 2), (0, 3), (0, 4)]);

        let rrc = grid.row_right_coords(0, 2);
        assert_eq!(rrc, [(0, 3), (0, 4)]);
    }

    #[test]
    fn col_up_coords() {
        let grid = Grid::init(5, 5, 0);
        let cup = grid.col_up_coords(0, 3);
        assert_eq!(cup, []);

        let cup = grid.col_up_coords(1, 3);
        assert_eq!(cup, [(0, 3)]);

        let cup = grid.col_up_coords(2, 3);
        assert_eq!(cup, [(0, 3), (1, 3)]);

        let cup = grid.col_up_coords(4, 3);
        assert_eq!(cup, [(0, 3), (1, 3), (2, 3), (3, 3)]);
    }

    #[test]
    fn col_down_coords() {
        let grid = Grid::init(5, 5, 0);
        let cdp = grid.col_down_coords(5, 3);
        assert_eq!(cdp, []);

        let cdp = grid.col_down_coords(3, 3);
        assert_eq!(cdp, [(4, 3)]);

        let cdp = grid.col_down_coords(2, 3);
        assert_eq!(cdp, [(3, 3), (4, 3)]);

        let cdp = grid.col_down_coords(1, 3);
        assert_eq!(cdp, [(2, 3), (3, 3), (4, 3)]);

        let cdp = grid.col_down_coords(0, 3);
        assert_eq!(cdp, [(1, 3), (2, 3), (3, 3), (4, 3)]);
    }

    #[test]
    fn neighbors_values() {
        let grid = Grid::from_2d_unchecked(vec![
            vec![0, 1, 2],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ]);

        assert_eq!(
            vec![(1, 1), (0, 0), (2, 0), (0, 1), (2, 1)],
            grid.neighbors(1, 0),
        );

        let n = grid.neighbors(1, 1);
        let v: Vec<_> = n.iter().map(|(r, c)| grid[*r][*c]).collect();
        assert_eq!(
            vec![4, 6, 1, 8, 0, 2, 7, 9],
            v,
        );

        let n = grid.neighbors(0, 0);
        let v: Vec<_> = n.iter().map(|(r, c)| grid[*r][*c]).collect();
        assert_eq!(
            vec![1, 4, 5],
            v,
        );

        let n = grid.neighbors(1, 2);
        let v: Vec<_> = n.iter().map(|(r, c)| grid[*r][*c]).collect();
        assert_eq!(
            vec![5, 2, 9, 1, 8],
            v,
        );
    }
}
