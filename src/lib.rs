//! Functionality for working with 2D grids.
//!
//! ## Why?
//! Working through the Advent of Code problems, I repeatedly found myself working with a 2D "grid". This led to re-implementing much of the same functionality across multiple problems, often with errors.
//!
//! ## Examples
//! ### Get the adjacent column coordinates of a particular cell
//!
//! ```
//! use griddy::Grid;
//!
//! let grid = Grid::from_2d_unchecked(vec![
//!     vec![0, 1, 2],
//!     vec![4, 5, 6],
//!     vec![7, 8, 9],
//! ]);
//!
//! // Cooridinates for values 1, 8
//! assert_eq!(
//!     vec![(0, 1), (2, 1)],
//!     grid.col_neighbors(1, 1),
//! );
//!```
//!
//! ### Get all the neighbors of a particular cell.
//!
//! Getting all neighbors for cell at `[1, 0]` (value: 4) would return a vec of cooridinates:
//! ```
//! use griddy::Grid;
//!
//! let grid = Grid::from_2d_unchecked(vec![
//!     vec![0, 1, 2],
//!     vec![4, 5, 6],
//!     vec![7, 8, 9],
//! ]);
//!
//! // Cooridinates for values 5, 0, 7, 1, 8
//! assert_eq!(
//!     vec![(1, 1), (0, 0), (2, 0), (0, 1), (2, 1)],
//!     grid.neighbors(1, 0),
//! );
//! ```

mod grid;

pub use grid::Grid;
