//! This module contains a [Span] settings, it helps to
//! make a cell take more space then it generally takes.
//!
//! # Example
//!
//! ```
//! use tabled::{object::Cell, Modify, TableIteratorExt, Span};
//!
//! let data = [[1, 2, 3], [4, 5, 6]];
//!
//! let table = data.table()
//!     .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
//!     .with(Modify::new(Cell(0, 1)).with(Span::column(2)))
//!     .to_string();
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+---+---+---+\n",
//!         "| 0 |   1   |\n",
//!         "+---+---+---+\n",
//!         "| 1 | 2 | 3 |\n",
//!         "+---+---+---+\n",
//!         "|   4   | 6 |\n",
//!         "+---+---+---+\n",
//!     )
//! )
//! ```

use crate::CellOption;
use papergrid::{Entity, Grid, Settings};

pub use papergrid::{AlignmentHorizontal, AlignmentVertical};

/// Span represent a horizontal/column span setting for any cell on a [Table].
///
/// ```rust,no_run
///   # use tabled::{Style, Span, Modify, object::Columns, Table};
///   # let data: Vec<&'static str> = Vec::new();
///     let table = Table::new(&data)
///         .with(Modify::new(Columns::single(0)).with(Span::column(2)));
/// ```
///
/// [Table]: crate::Table
#[derive(Debug)]
pub struct Span {
    size: usize,
}

impl Span {
    /// New constructs a horizontal/column [Span].
    pub fn column(size: usize) -> Self {
        Self { size }
    }
}

impl CellOption for Span {
    fn change_cell(&mut self, grid: &mut Grid, row: usize, column: usize) {
        grid.set(Entity::Cell(row, column), Settings::new().span(self.size));
    }
}
