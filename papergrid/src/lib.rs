//! Papergrid is a library for generating text-based tables for display
//!
//! # Example
//! ```rust
//!     use papergrid::{Grid, Entity, Settings};
//!     let mut grid = Grid::new(2, 2);
//!
//!     grid.set(Entity::Cell(0, 0), Settings::new().text("0-0"));
//!     grid.set(Entity::Cell(0, 1), Settings::new().text("0-1"));
//!     grid.set(Entity::Cell(1, 0), Settings::new().text("1-0"));
//!     grid.set(Entity::Cell(1, 1), Settings::new().text("1-1"));
//!
//!     let expected = concat!(
//!         "+---+---+\n",
//!         "|0-0|0-1|\n",
//!         "+---+---+\n",
//!         "|1-0|1-1|\n",
//!         "+---+---+\n",
//!     );
//!
//!     assert_eq!(expected, grid.to_string());
//! ```

use std::{
    cmp::{self, max},
    collections::{BTreeSet, HashMap},
    fmt::{self, Display, Write},
    hash::Hash,
    ops::{Bound, RangeBounds},
};

pub const DEFAULT_BORDERS: Borders = Borders {
    top: Some(Symbol::from_char('-')),
    top_left: Some(Symbol::from_char('+')),
    top_right: Some(Symbol::from_char('+')),
    top_intersection: Some(Symbol::from_char('+')),

    bottom: Some(Symbol::from_char('-')),
    bottom_left: Some(Symbol::from_char('+')),
    bottom_right: Some(Symbol::from_char('+')),
    bottom_intersection: Some(Symbol::from_char('+')),

    horizontal: Some(Symbol::from_char('-')),
    horizontal_left: Some(Symbol::from_char('+')),
    horizontal_right: Some(Symbol::from_char('+')),

    vertical_left: Some(Symbol::from_char('|')),
    vertical_right: Some(Symbol::from_char('|')),
    vertical_intersection: Some(Symbol::from_char('|')),

    intersection: Some(Symbol::from_char('+')),
};

const DEFAULT_BORDER_HORIZONTAL_CHAR: Symbol = Symbol::from_char(' ');
const DEFAULT_BORDER_VERTICAL_CHAR: Symbol = Symbol::from_char(' ');

const DEFAULT_INDENT_FILL_CHAR: char = ' ';

/// Grid provides a set of methods for building a text-based table
#[derive(Debug, Clone)]
pub struct Grid {
    size: (usize, usize),
    cells: Vec<Vec<String>>,
    styles: HashMap<Entity, Style>,
    margin: Margin,
    theme: Theme,
    override_split_lines: HashMap<usize, String>,
}

impl Grid {
    /// The new method creates a grid instance with default styles.
    ///
    /// The size of the grid can not be changed after the instance is created.
    ///
    /// # Example
    ///
    /// ```rust
    ///     use papergrid::{Grid, Entity, Settings};
    ///     let mut grid = Grid::new(2, 2);
    ///     assert_eq!(
    ///          grid.to_string(),
    ///          "+++\n\
    ///           +++\n\
    ///           +++\n"
    ///     )
    /// ```
    pub fn new(rows: usize, columns: usize) -> Self {
        let mut styles = HashMap::with_capacity(1);
        styles.insert(Entity::Global, Style::default());

        Grid {
            size: (rows, columns),
            cells: vec![vec![String::new(); columns]; rows],
            styles,
            margin: Margin::default(),
            theme: Theme::new(),
            override_split_lines: HashMap::new(),
        }
    }

    /// Set method is responsible for modification of cell/row/column.
    ///
    /// The method panics if incorrect cell/row/column index is given.
    ///
    /// # Example
    ///
    /// ```rust
    ///     use papergrid::{Grid, Entity, Settings};
    ///     let mut grid = Grid::new(2, 2);
    ///     grid.set(Entity::Row(0), Settings::new().text("row 1"));
    ///     grid.set(Entity::Row(1), Settings::new().text("row 2"));
    ///     assert_eq!(
    ///          grid.to_string(),
    ///          "+-----+-----+\n\
    ///           |row 1|row 1|\n\
    ///           +-----+-----+\n\
    ///           |row 2|row 2|\n\
    ///           +-----+-----+\n"
    ///     )
    /// ```
    pub fn set(&mut self, entity: Entity, settings: Settings) {
        if let Some(text) = settings.text {
            self.set_text(entity, text);
        }

        let is_style_changes = settings.padding.is_some()
            || settings.alignment_h.is_some()
            || settings.alignment_v.is_some()
            || settings.span.is_some()
            || settings.formatting.is_some();

        if is_style_changes {
            self.remove_inherited_styles(entity);
            let style = self.style_mut(entity);

            if let Some(padding) = settings.padding {
                style.padding = padding;
            }

            if let Some(alignment_h) = settings.alignment_h {
                style.alignment_h = alignment_h;
            }

            if let Some(alignment_v) = settings.alignment_v {
                style.alignment_v = alignment_v;
            }

            if let Some(span) = settings.span {
                style.span = span;
            }

            if let Some(formatting) = settings.formatting {
                style.formatting = formatting;
            }
        }

        if let Some(border) = settings.border {
            self.set_border(entity, border);
        }
    }

    pub fn margin(&mut self, margin: Margin) {
        self.margin = margin
    }

    pub fn get_margin(&self) -> &Margin {
        &self.margin
    }

    pub fn clear_theme(&mut self) {
        self.theme = Theme::new();
        self.override_split_lines.clear();
    }

    pub fn set_borders(&mut self, borders: Borders) {
        self.theme.borders = borders;
    }

    pub fn get_borders(&self) -> &Borders {
        &self.theme.borders
    }

    pub fn set_border(&mut self, entity: Entity, border: Border) {
        match entity {
            Entity::Global => {
                for col in 0..self.count_columns() {
                    for row in 0..self.count_rows() {
                        self.theme.override_border((row, col), border.clone());
                    }
                }
            }
            Entity::Column(col) => {
                for row in 0..self.count_rows() {
                    self.theme.override_border((row, col), border.clone());
                }
            }
            Entity::Row(row) => {
                for col in 0..self.count_columns() {
                    self.theme.override_border((row, col), border.clone());
                }
            }
            Entity::Cell(row, col) => {
                self.theme.override_border((row, col), border);
            }
        }
    }

    pub fn set_split_line(&mut self, row: usize, line: Line) {
        self.theme.override_line(row, line)
    }

    /// get_cell_settings returns a settings of a cell
    pub fn get_settings(&self, row: usize, col: usize) -> Settings {
        let style = self.style(Entity::Cell(row, col));
        let content = &self.cells[row][col];
        let border = self.get_border(row, col);

        Settings::default()
            .text(content)
            .alignment(style.alignment_h)
            .vertical_alignment(style.alignment_v)
            .span(style.span)
            .padding(
                style.padding.left,
                style.padding.right,
                style.padding.top,
                style.padding.bottom,
            )
            .border(border)
    }

    pub fn get_border(&self, row: usize, col: usize) -> Border {
        let mut border = self
            .theme
            .get_border((row, col), self.count_rows(), self.count_columns());

        // make sure that there's no user defined lines
        // in which case we use spaces.

        let mut top_set = border.top.is_some();
        let mut bottom_set = border.bottom.is_some();
        let mut left_set = border.left.is_some();
        let mut right_set = border.right.is_some();

        if border.top.is_none() && has_horizontal(self, row) {
            border.top = Some(DEFAULT_BORDER_HORIZONTAL_CHAR);
            top_set = true;
        }

        if border.bottom.is_none() && has_horizontal(self, row + 1) {
            border.bottom = Some(DEFAULT_BORDER_HORIZONTAL_CHAR);
            bottom_set = true;
        }

        if border.left.is_none() && has_vertical(self, col) {
            border.left = Some(DEFAULT_BORDER_VERTICAL_CHAR);
            left_set = true;
        }

        if border.right.is_none() && has_vertical(self, col + 1) {
            border.right = Some(DEFAULT_BORDER_VERTICAL_CHAR);
            right_set = true;
        }

        if border.left_top_corner.is_none() && top_set && left_set {
            border.left_top_corner = Some(DEFAULT_BORDER_VERTICAL_CHAR);
        }

        if border.left_bottom_corner.is_none() && bottom_set && left_set {
            border.left_bottom_corner = Some(DEFAULT_BORDER_VERTICAL_CHAR);
        }

        if border.right_top_corner.is_none() && top_set && right_set {
            border.right_top_corner = Some(DEFAULT_BORDER_VERTICAL_CHAR);
        }

        if border.right_bottom_corner.is_none() && bottom_set && right_set {
            border.right_bottom_corner = Some(DEFAULT_BORDER_VERTICAL_CHAR);
        }

        border
    }

    pub fn style(&self, entity: Entity) -> &Style {
        let lookup_table = match entity {
            Entity::Global => vec![Entity::Global],
            Entity::Column(column) => vec![Entity::Column(column), Entity::Global],
            Entity::Row(row) => vec![Entity::Row(row), Entity::Global],
            Entity::Cell(row, column) => vec![
                Entity::Cell(row, column),
                Entity::Column(column),
                Entity::Row(row),
                Entity::Global,
            ],
        };

        for entity in lookup_table {
            if let Some(style) = self.styles.get(&entity) {
                return style;
            }
        }

        unreachable!("there's a Entity::Global setting guaranted in the map")
    }

    fn style_mut(&mut self, entity: Entity) -> &mut Style {
        if self.styles.contains_key(&entity) {
            return self.styles.get_mut(&entity).unwrap();
        }

        let style = self.style(entity).clone();
        self.styles.insert(entity, style);
        self.styles.get_mut(&entity).unwrap()
    }

    fn remove_inherited_styles(&mut self, entity: Entity) {
        match entity {
            Entity::Global => self.styles.retain(|k, _| matches!(k, Entity::Global)),
            Entity::Column(col) => self
                .styles
                .retain(move |k, _| !matches!(k, Entity::Cell(_, c) if *c == col)),
            Entity::Row(row) => self
                .styles
                .retain(move |k, _| !matches!(k, Entity::Cell(r, _) if *r == row)),
            Entity::Cell(_, _) => {}
        }
    }

    /// get_cell_content returns content without any style changes
    pub fn get_cell_content(&self, row: usize, column: usize) -> &str {
        self.cells[row][column].as_str()
    }

    /// Count_rows returns an amount of rows on the grid
    pub fn count_rows(&self) -> usize {
        self.size.0
    }

    /// Count_rows returns an amount of columns on the grid
    pub fn count_columns(&self) -> usize {
        self.size.1
    }

    pub fn set_text<S: Into<String>>(&mut self, entity: Entity, text: S) {
        let text = text.into();
        match entity {
            Entity::Cell(row, column) => {
                self.cells[row][column] = text;
            }
            Entity::Column(column) => {
                for row in 0..self.count_rows() {
                    self.cells[row][column] = text.clone();
                }
            }
            Entity::Row(row) => {
                for column in 0..self.count_columns() {
                    self.cells[row][column] = text.clone();
                }
            }
            Entity::Global => {
                for row in 0..self.count_rows() {
                    for column in 0..self.count_columns() {
                        self.cells[row][column] = text.clone();
                    }
                }
            }
        }
    }

    /// Returns a new [Grid] that reflects a segment of the referenced [Grid]
    ///
    /// The segment is defined by [RangeBounds<usize>] for Rows and Columns
    ///
    /// # Example
    ///
    /// ```text,no_run
    /// grid
    /// +---+---+---+
    /// |0-0|0-1|0-2|
    /// +---+---+---+
    /// |1-0|1-1|1-2|
    /// +---+---+---+
    /// |2-0|2-1|2-2|
    /// +---+---+---+
    /// let rows = ..;
    /// let columns = ..1;
    /// grid.extract(rows, columns)
    /// +---+
    /// |0-0|
    /// +---+
    /// |1-0|
    /// +---+
    /// |2-0|
    /// +---+
    /// ```
    pub fn extract<R, C>(&self, rows: R, columns: C) -> Self
    where
        R: RangeBounds<usize>,
        C: RangeBounds<usize>,
    {
        let (start_row, end_row) =
            bounds_to_usize(rows.start_bound(), rows.end_bound(), self.count_rows());
        let (start_column, end_column) = bounds_to_usize(
            columns.start_bound(),
            columns.end_bound(),
            self.count_columns(),
        );

        let new_count_rows = end_row - start_row;
        let new_count_columns = end_column - start_column;
        let mut new_grid = Grid::new(new_count_rows, new_count_columns);
        new_grid.theme = self.theme.clone();

        for (new_row, row) in (start_row..end_row).enumerate() {
            for (new_column, column) in (start_column..end_column).enumerate() {
                let settings = self.get_settings(row, column);
                new_grid.set(Entity::Cell(new_row, new_column), settings);
            }
        }

        new_grid
    }

    /// Returns a total width of table, including split lines.
    pub fn total_width(&self) -> usize {
        let count_rows = self.count_rows();
        let count_columns = self.count_columns();
        if count_rows == 0 || count_columns == 0 {
            return 0;
        }

        let mut cells = self.collect_cells();
        let mut styles = self.collect_styles();

        fix_spans(&mut styles, &mut cells);

        let widths = columns_width(self, &cells, &styles);

        total_width(self, &widths, &styles, &self.margin)
    }

    pub fn override_split_line(&mut self, row: usize, line: impl Into<String>) {
        self.override_split_lines.insert(row, line.into());
    }

    // hide it by feature?
    // 'private'
    pub fn build_widths(&self) -> (Vec<Vec<usize>>, Vec<Vec<Style>>) {
        let mut cells = self.collect_cells();
        let mut styles = self.collect_styles();

        fix_spans(&mut styles, &mut cells);

        let widths = columns_width(self, &cells, &styles);

        (widths, styles)
    }

    fn collect_cells(&self) -> Vec<Vec<Vec<String>>> {
        let count_rows = self.count_rows();
        let count_columns = self.count_columns();

        let mut rows = vec![Vec::with_capacity(self.count_columns()); self.count_rows()];
        (0..count_rows).for_each(|row| {
            (0..count_columns).for_each(|col| {
                let mut content = self.cells[row][col].clone();

                let style = self.style(Entity::Cell(row, col));
                replace_tab(&mut content, style.formatting.tab_width);

                // fixme: I guess it can be done in a different place?
                let lines: Vec<_> = content.lines().map(|l| l.to_owned()).collect();
                rows[row].push(lines);
            });
        });

        rows
    }

    fn collect_styles(&self) -> Vec<Vec<Style>> {
        let mut rows = vec![Vec::with_capacity(self.count_columns()); self.count_rows()];
        (0..self.count_rows()).for_each(|row| {
            (0..self.count_columns()).for_each(|col| {
                let style = self.style(Entity::Cell(row, col));
                rows[row].push(style.clone());
            });
        });

        fix_styles(&mut rows);

        rows
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Border {
    pub top: Option<Symbol>,
    pub bottom: Option<Symbol>,
    pub left: Option<Symbol>,
    pub left_top_corner: Option<Symbol>,
    pub left_bottom_corner: Option<Symbol>,
    pub right: Option<Symbol>,
    pub right_top_corner: Option<Symbol>,
    pub right_bottom_corner: Option<Symbol>,
}

impl Border {
    /// This function constructs a cell borders with all sides set.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        top: impl Into<Symbol>,
        bottom: impl Into<Symbol>,
        left: impl Into<Symbol>,
        right: impl Into<Symbol>,
        top_left: impl Into<Symbol>,
        top_right: impl Into<Symbol>,
        bottom_left: impl Into<Symbol>,
        bottom_right: impl Into<Symbol>,
    ) -> Self {
        Self {
            top: Some(top.into()),
            bottom: Some(bottom.into()),
            right: Some(right.into()),
            right_top_corner: Some(top_right.into()),
            right_bottom_corner: Some(bottom_right.into()),
            left: Some(left.into()),
            left_bottom_corner: Some(bottom_left.into()),
            left_top_corner: Some(top_left.into()),
        }
    }

    /// This function constructs a cell borders with all sides's char set to a given character.
    /// It behaives like [Border::new] with the same character set to each side.
    pub fn filled(c: impl Into<Symbol>) -> Self {
        let c = c.into();
        Self::new(
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c.clone(),
            c,
        )
    }

    /// Set a top border character.
    pub fn top(mut self, c: impl Into<Symbol>) -> Self {
        self.top = Some(c.into());
        self
    }

    /// Set a bottom border character.
    pub fn bottom(mut self, c: impl Into<Symbol>) -> Self {
        self.bottom = Some(c.into());
        self
    }

    /// Set a left border character.
    pub fn left(mut self, c: impl Into<Symbol>) -> Self {
        self.left = Some(c.into());
        self
    }

    /// Set a right border character.
    pub fn right(mut self, c: impl Into<Symbol>) -> Self {
        self.right = Some(c.into());
        self
    }

    /// Set a top left intersection character.
    pub fn top_left_corner(mut self, c: impl Into<Symbol>) -> Self {
        self.left_top_corner = Some(c.into());
        self
    }

    /// Set a top right intersection character.
    pub fn top_right_corner(mut self, c: impl Into<Symbol>) -> Self {
        self.right_top_corner = Some(c.into());
        self
    }

    /// Set a bottom left intersection character.
    pub fn bottom_left_corner(mut self, c: impl Into<Symbol>) -> Self {
        self.left_bottom_corner = Some(c.into());
        self
    }

    /// Set a bottom right intersection character.
    pub fn bottom_right_corner(mut self, c: impl Into<Symbol>) -> Self {
        self.right_bottom_corner = Some(c.into());
        self
    }
}

/// Entity a structure which represent a set of cells.
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Entity {
    /// All cells on the grid.
    Global,
    /// All cells in a column on the grid.
    Column(usize),
    /// All cells in a row on the grid.
    Row(usize),
    /// A particular cell (row, column) on the grid.
    Cell(usize, usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Style {
    pub span: usize,
    pub padding: Padding,
    pub alignment_h: AlignmentHorizontal,
    pub alignment_v: AlignmentVertical,
    pub formatting: Formatting,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            span: 1,
            padding: Padding::default(),
            alignment_h: AlignmentHorizontal::Left,
            alignment_v: AlignmentVertical::Top,
            formatting: Formatting {
                horizontal_trim: false,
                vertical_trim: false,
                allow_lines_alignement: false,
                tab_width: 4,
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Formatting {
    pub horizontal_trim: bool,
    pub vertical_trim: bool,
    pub allow_lines_alignement: bool,
    pub tab_width: usize,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Margin {
    pub top: Indent,
    pub bottom: Indent,
    pub left: Indent,
    pub right: Indent,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Padding {
    pub top: Indent,
    pub bottom: Indent,
    pub left: Indent,
    pub right: Indent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Indent {
    pub fill: char,
    pub size: usize,
}

impl Default for Indent {
    fn default() -> Self {
        Self {
            fill: DEFAULT_INDENT_FILL_CHAR,
            size: 0,
        }
    }
}

impl Indent {
    pub fn new(size: usize, fill: char) -> Self {
        Self { size, fill }
    }

    pub fn spaced(size: usize) -> Self {
        Self {
            size,
            fill: DEFAULT_INDENT_FILL_CHAR,
        }
    }
}

/// AlignmentHorizontal represents an horizontal aligment of a cell content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentHorizontal {
    Center,
    Left,
    Right,
}

impl AlignmentHorizontal {
    fn align_with_max_width(
        &self,
        f: &mut fmt::Formatter<'_>,
        text: &str,
        width: usize,
        text_width: usize,
        max_text_width: usize,
    ) -> fmt::Result {
        let diff = width - text_width;

        match self {
            AlignmentHorizontal::Left => Self::align(f, text, 0, diff),
            AlignmentHorizontal::Right => {
                let max_diff = width - max_text_width;
                let rest = diff - max_diff;
                Self::align(f, text, max_diff, rest)
            }
            AlignmentHorizontal::Center => {
                let max_diff = width - max_text_width;
                let left = max_diff / 2;
                let rest = diff - left;
                Self::align(f, text, left, rest)
            }
        }
    }

    fn align(f: &mut fmt::Formatter<'_>, text: &str, left: usize, right: usize) -> fmt::Result {
        write!(
            f,
            "{: <left$}{text}{: <right$}",
            "",
            "",
            left = left,
            right = right,
            text = text
        )
    }
}

/// AlignmentVertical represents an vertical aligment of a cell content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentVertical {
    Center,
    Top,
    Bottom,
}

impl AlignmentVertical {
    fn top_ident(&self, height: usize, real_height: usize) -> usize {
        match self {
            AlignmentVertical::Top => 0,
            AlignmentVertical::Bottom => height - real_height,
            AlignmentVertical::Center => (height - real_height) / 2,
        }
    }
}

/// Settings represent setting of a particular cell
#[derive(Debug, Clone, Default)]
pub struct Settings {
    text: Option<String>,
    padding: Option<Padding>,
    border: Option<Border>,
    span: Option<usize>,
    alignment_h: Option<AlignmentHorizontal>,
    alignment_v: Option<AlignmentVertical>,
    formatting: Option<Formatting>,
}

impl Settings {
    /// New method constructs an instance of settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Text method sets content for a cell
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    /// padding method sets padding for a cell
    pub fn padding(mut self, left: Indent, right: Indent, top: Indent, bottom: Indent) -> Self {
        self.padding = Some(Padding {
            top,
            bottom,
            left,
            right,
        });
        self
    }

    /// Alignment method sets horizontal alignment for a cell
    pub fn alignment(mut self, alignment: AlignmentHorizontal) -> Self {
        self.alignment_h = Some(alignment);
        self
    }

    /// Alignment method sets horizontal alignment for a cell
    pub fn vertical_alignment(mut self, alignment: AlignmentVertical) -> Self {
        self.alignment_v = Some(alignment);
        self
    }

    /// Set the settings's span.
    pub fn span(mut self, span: usize) -> Self {
        self.span = Some(span);
        self
    }

    /// Set the settings's border.
    ///
    /// The border setting is in a restrictive manner, by default.
    /// So if there was no split line but border relies on it
    /// a error will be issued.
    ///
    /// To fix it you can construct split lines before calling this function.
    /// Or you can pass a `false` argument into [Self::border_restriction]
    /// so if absent lines will be created.
    pub fn border(mut self, border: Border) -> Self {
        self.border = Some(border);
        self
    }

    /// Set a formatting settings.
    ///
    /// It overades them even if any were not set.
    pub fn formatting(mut self, formatting: Formatting) -> Self {
        self.formatting = Some(formatting);
        self
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count_rows = self.count_rows();
        let count_columns = self.count_columns();

        if count_rows == 0 || count_columns == 0 {
            return Ok(());
        }

        let mut cells = self.collect_cells();
        let mut styles = self.collect_styles();

        fix_spans(&mut styles, &mut cells);

        let heights = rows_height(&cells, &styles, count_rows, count_columns);
        let widths = columns_width(self, &cells, &styles);
        let normal_widths = normalized_width(&widths, &styles, count_rows, count_columns);

        print_grid(f, self, cells, styles, widths, normal_widths, heights)
    }
}

fn build_line_cell(
    f: &mut fmt::Formatter<'_>,
    line_index: usize,
    mut cell: &[String],
    style: &Style,
    width: usize,
    height: usize,
) -> fmt::Result {
    if style.formatting.vertical_trim {
        cell = skip_empty_lines(cell);
    }

    let top_indent = top_indent(cell, style, height);
    if top_indent > line_index {
        return repeat_char(f, &Symbol::from(style.padding.top.fill), width);
    }

    let cell_line_index = line_index - top_indent;
    let cell_has_this_line = cell.len() > cell_line_index;
    // happens when other cells have bigger height
    if !cell_has_this_line {
        return repeat_char(f, &Symbol::from(style.padding.bottom.fill), width);
    }

    let mut text = cell[cell_line_index].as_str();
    if style.formatting.horizontal_trim && style.formatting.allow_lines_alignement {
        text = text.trim();
    } else if style.formatting.horizontal_trim {
        text = text.trim_end();
    }

    let line_width = string_width(text);

    if style.formatting.allow_lines_alignement {
        line_with_width(f, text, width, line_width, line_width, style)
    } else {
        let max_line_width = cell
            .iter()
            .map(|line| {
                if style.formatting.horizontal_trim {
                    line.trim_end()
                } else {
                    line
                }
            })
            .map(string_width)
            .max()
            .unwrap_or(0);

        line_with_width(f, text, width, line_width, max_line_width, style)
    }
}

fn skip_empty_lines(cell: &[String]) -> &[String] {
    let count_lines = cell.len();

    let count_empty_lines_before_text = cell
        .iter()
        .take_while(|line| line.trim().is_empty())
        .count();
    if count_empty_lines_before_text == count_lines {
        return &[];
    }

    let empty_lines_at_end = cell
        .iter()
        .rev()
        .take_while(|line| line.trim().is_empty())
        .count();

    let text_start_pos = count_empty_lines_before_text;
    let text_end_pos = cell.len() - empty_lines_at_end;

    &cell[text_start_pos..text_end_pos]
}

fn top_indent(cell: &[String], style: &Style, height: usize) -> usize {
    let height = height - style.padding.top.size;
    let content_height = cell.len();
    let indent = style.alignment_v.top_ident(height, content_height);

    indent + style.padding.top.size
}

fn repeat_char(f: &mut fmt::Formatter<'_>, c: &Symbol, n: usize) -> fmt::Result {
    if n > 0 {
        for _ in 0..n {
            c.fmt(f)?;
        }
    }
    Ok(())
}

fn line_with_width(
    f: &mut fmt::Formatter<'_>,
    text: &str,
    width: usize,
    width_text: usize,
    width_text_max: usize,
    style: &Style,
) -> fmt::Result {
    let left_indent = style.padding.left;
    let right_indent = style.padding.right;
    let alignment = style.alignment_h;

    repeat_char(f, &Symbol::from(left_indent.fill), left_indent.size)?;
    let width = width - left_indent.size - right_indent.size;
    alignment.align_with_max_width(f, text, width, width_text, width_text_max)?;
    repeat_char(f, &Symbol::from(right_indent.fill), right_indent.size)?;

    Ok(())
}

pub fn strip(s: &str, width: usize) -> String {
    #[cfg(not(feature = "color"))]
    {
        s.chars().take(width).collect::<String>()
    }
    #[cfg(feature = "color")]
    {
        let width = to_byte_length(s, width);
        ansi_str::AnsiStr::ansi_cut(s, ..width)
    }
}

#[cfg(feature = "color")]
fn to_byte_length(s: &str, width: usize) -> usize {
    s.chars().take(width).map(|c| c.len_utf8()).sum::<usize>()
}

#[cfg(not(feature = "color"))]
pub fn string_width(text: &str) -> usize {
    real_string_width(text)
}

#[cfg(feature = "color")]
pub fn string_width(text: &str) -> usize {
    let b = strip_ansi_escapes::strip(text.as_bytes()).unwrap();
    let s = std::str::from_utf8(&b).unwrap();
    real_string_width(s)
}

fn real_string_width(text: &str) -> usize {
    text.lines()
        .map(unicode_width::UnicodeWidthStr::width)
        .max()
        .unwrap_or(0)
}

fn fix_styles(styles: &mut [Vec<Style>]) {
    styles.iter_mut().for_each(|row_styles| {
        fix_invisible_cell(row_styles);
    });
}

fn fix_invisible_cell(styles: &mut [Style]) {
    (0..styles.len()).for_each(|col| {
        if !is_cell_visible(styles, col) {
            styles[col].span = 0;
        }
    });
}

// Sometimes user may not increase some span while decreasing another cell
// Which may cause an incorrect rendering.
//
// So we are fixing the spans to accordingly.
fn fix_spans(styles: &mut [Vec<Style>], cells: &mut [Vec<Vec<String>>]) {
    (0..styles.len()).for_each(|row| {
        fix_zero_spans(&mut styles[row], &mut cells[row]);
    });
}

fn fix_zero_spans(styles: &mut [Style], widths: &mut [Vec<String>]) {
    if styles.is_empty() {
        return;
    }

    // fix first column
    fix_first_column_span(styles, widths);
    // fix an inner space
    fix_zero_column_span(styles);
}

fn fix_zero_column_span(styles: &mut [Style]) {
    for i in 0..styles.len() {
        if styles[i].span > 0 {
            continue;
        }

        if is_cell_overriden(&styles[..i]) {
            continue;
        }

        let prev_visible_cell = (0..i).rev().find(|&i| styles[i].span > 0);
        if let Some(pos) = prev_visible_cell {
            let need_at_least_span = i - pos;
            styles[pos].span = need_at_least_span + 1;
        }
    }
}

fn fix_first_column_span(styles: &mut [Style], widths: &mut [Vec<String>]) {
    if styles[0].span == 0 {
        let next_visible_cell = (1..styles.len()).find(|&i| styles[i].span > 0);
        if let Some(i) = next_visible_cell {
            styles[i].span += i;
            styles.swap(0, i);
            widths.swap(0, i);
        }
    }
}

fn columns_width(
    grid: &Grid,
    cells: &[Vec<Vec<String>>],
    styles: &[Vec<Style>],
) -> Vec<Vec<usize>> {
    let mut widths = vec![vec![0; grid.count_columns()]; grid.count_rows()];
    (0..grid.count_rows()).for_each(|row| {
        (0..grid.count_columns()).for_each(|column| {
            let cell = &cells[row][column];
            let style = &styles[row][column];
            if is_cell_visible(&styles[row], column) {
                widths[row][column] = cell_width(cell, style);
            }
        });
    });

    // it's crusial to preserve order in iterations
    // so we use BTreeSet
    let mut spans = BTreeSet::new();
    styles.iter().for_each(|row_styles| {
        row_styles.iter().for_each(|style| {
            spans.insert(style.span);
        })
    });
    spans.remove(&0);

    spans.into_iter().for_each(|span| {
        adjust_width(&mut widths, styles, span, grid);
    });

    widths
}

fn adjust_width(widths: &mut [Vec<usize>], styles: &[Vec<Style>], span: usize, grid: &Grid) {
    let ranges = (0..grid.count_columns())
        .map(|col| (col, col + span))
        .take_while(|&(_, end)| end <= grid.count_columns());

    for (start, end) in ranges.clone() {
        adjust_range_width(widths, styles, start, end, grid);
    }

    if span > 1 {
        // sometimes the adjustment of later stages affect the adjastement of privious stages.
        // therefore we check if this is the case and re run the adjustement one more time.
        for (start, end) in ranges {
            let is_range_complete = is_range_complete(styles, widths, start, end, grid);
            if !is_range_complete {
                adjust_range_width(widths, styles, start, end, grid);
            }
        }
    }
}

fn adjust_range_width(
    widths: &mut [Vec<usize>],
    styles: &[Vec<Style>],
    start_column: usize,
    end_column: usize,
    grid: &Grid,
) {
    if grid.count_rows() == 0 {
        return;
    }

    // find max width of a column range
    let (max_row, max_width) = (0..grid.count_rows())
        .map(|row| {
            let width = row_width(&styles[row], &widths[row], start_column, end_column, grid);
            (row, width)
        })
        .max_by_key(|&(_, width)| width)
        .unwrap_or_default();

    // might happen when we filtered every cell
    if max_width == 0 {
        return;
    }

    // increase the widths
    (0..grid.count_rows())
        .filter(|&row| row != max_row)
        .filter(|&row| !is_there_out_of_scope_cell(&styles[row], start_column, end_column)) // ignore the cell we do handle this case later on
        .for_each(|row| {
            let row_width = row_width(&styles[row], &widths[row], start_column, end_column, grid);

            let diff = max_width - row_width;

            inc_cells_width(
                &mut widths[row],
                &styles[row],
                start_column,
                end_column,
                diff,
            );
        });

    let span = end_column - start_column;
    if span > 1 {
        // fixing the rows with out_of_scope cells
        //
        // these cells may not have correct width, therefore
        // we replace these cells's width with
        // a width of cells with the same span and on the same column.
        (0..grid.count_rows())
            .filter(|&row| row != max_row)
            .filter(|&row| is_there_out_of_scope_cell(&styles[row], start_column, end_column))
            .for_each(|row| {
                (start_column..end_column)
                    .filter(|&col| is_cell_visible(&styles[row], col))
                    .for_each(|col| {
                        let cell_with_the_same_cell = (0..grid.count_rows())
                            .filter(|&r| r != max_row)
                            .filter(|&r| r != row)
                            .filter(|&r| {
                                !is_there_out_of_scope_cell(&styles[r], start_column, end_column)
                            })
                            .find(|&r| styles[r][col].span == styles[row][col].span);

                        if let Some(r) = cell_with_the_same_cell {
                            widths[row][col] = widths[r][col];
                        }
                    })
            });
    }
}

fn is_there_out_of_scope_cell(styles: &[Style], start_column: usize, end_column: usize) -> bool {
    let first_cell_is_invisible = !is_cell_visible(styles, start_column);
    let any_cell_out_of_scope = (start_column..end_column)
        .filter(|&col| is_cell_visible(styles, col))
        .any(|col| !is_cell_in_scope(styles, col, end_column));

    first_cell_is_invisible || any_cell_out_of_scope
}

fn is_cell_in_scope(styles: &[Style], col: usize, end_col: usize) -> bool {
    let next_col = col + styles[col].span;
    next_col <= end_col
}

fn is_cell_visible(row_styles: &[Style], column: usize) -> bool {
    let is_span_zero = row_styles[column].span == 0;
    if is_span_zero {
        return false;
    }

    let is_cell_overriden = is_cell_overriden(&row_styles[..column]);

    !is_cell_overriden
}

fn is_cell_overriden(styles: &[Style]) -> bool {
    styles
        .iter()
        .enumerate()
        .any(|(i, style)| style.span > styles.len() - i)
}

fn is_range_complete(
    styles: &[Vec<Style>],
    widths: &[Vec<usize>],
    start_column: usize,
    end_column: usize,
    grid: &Grid,
) -> bool {
    let is_not_complete = (0..grid.count_rows())
        .filter(|&row| !is_there_out_of_scope_cell(&styles[row], start_column, end_column))
        .map(|row| row_width(&styles[row], &widths[row], start_column, end_column, grid))
        .fold(None, |mut acc, width| {
            match acc {
                Some((w, true)) if w != width => {
                    acc = Some((0, false));
                }
                None => {
                    acc = Some((width, true));
                }
                _ => {}
            };

            acc
        });

    matches!(is_not_complete, Some((_, true)))
}

fn row_width(styles: &[Style], widths: &[usize], start: usize, end: usize, grid: &Grid) -> usize {
    let width = width_of_range(&styles[start..end], &widths[start..end]);
    let border_count = count_borders_in_range(styles, start, end, grid);

    width + border_count
}

fn width_of_range(styles: &[Style], widths: &[usize]) -> usize {
    (0..styles.len())
        .filter(|&i| is_cell_visible(styles, i))
        .filter(|&i| is_cell_in_scope(styles, i, styles.len()))
        .map(|i| widths[i])
        .sum::<usize>()
}

fn count_borders_in_range(styles: &[Style], start: usize, end: usize, grid: &Grid) -> usize {
    (start..end)
        .filter(|&i| is_cell_visible(styles, i))
        .filter(|&i| is_cell_in_scope(styles, i, styles.len()))
        .skip(1)
        .filter(|&i| has_vertical(grid, i))
        .count()
}

fn inc_cells_width(
    widths: &mut [usize],
    styles: &[Style],
    start_range: usize,
    end_range: usize,
    inc: usize,
) {
    for (i, col) in (start_range..end_range).cycle().enumerate() {
        if i == inc {
            break;
        }

        let col = get_closest_visible_pos(styles, col);
        match col {
            Some(col) => widths[col] += 1,
            None => unreachable!("Never suppose to happen"),
        }
    }
}

fn get_closest_visible_pos(styles: &[Style], mut col: usize) -> Option<usize> {
    loop {
        if is_cell_visible(styles, col) {
            return Some(col);
        }

        if col == 0 {
            return None;
        }

        col -= 1;
    }
}

fn cell_width(cell: &[String], style: &Style) -> usize {
    let content_width = cell.iter().map(|l| string_width(l)).max().unwrap_or(0);
    content_width + style.padding.left.size + style.padding.right.size
}

fn rows_height(
    cells: &[Vec<Vec<String>>],
    styles: &[Vec<Style>],
    count_rows: usize,
    count_columns: usize,
) -> Vec<usize> {
    let mut row_heights = vec![0; count_rows];
    (0..count_rows).for_each(|row_index| {
        (0..count_columns).for_each(|column_index| {
            let cell = &cells[row_index][column_index];
            let style = &styles[row_index][column_index];
            row_heights[row_index] = max(row_heights[row_index], cell_height(cell, style));
        });
    });

    row_heights
}

fn cell_height(cell: &[String], style: &Style) -> usize {
    let is_there_padding = style.padding.left.size > 0 || style.padding.right.size > 0;
    let mut content_height = cell.len();
    if content_height == 0 && is_there_padding {
        content_height = 1;
    }

    content_height + style.padding.top.size + style.padding.bottom.size
}

fn normalized_width(
    widths: &[Vec<usize>],
    styles: &[Vec<Style>],
    count_rows: usize,
    count_columns: usize,
) -> Vec<usize> {
    let mut v = vec![0; count_columns];
    let mut skip = 0;
    for col in 0..count_columns {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        let min_spanned_row = (0..count_rows)
            .filter(|&row| styles[row][col].span > 0)
            .min_by_key(|&x| styles[x][col].span);

        if let Some(row) = min_spanned_row {
            let span = styles[row][col].span;
            let mut width = widths[row][col] - (span - 1); // todo: explain this span-1 ?

            for col in (col..col + span).cycle() {
                if width == 0 {
                    break;
                }

                v[col] += 1;
                width -= 1;
            }

            skip += span - 1;
        }
    }

    v
}

fn replace_tab(cell: &mut String, n: usize) -> &str {
    let mut skip = 0;
    while let &Some(pos) = &cell[skip..].find('\t') {
        let pos = skip + pos;

        let is_escaped = pos > 0 && cell.get(pos - 1..pos) == Some("\\");
        if is_escaped {
            skip = pos + 1;
        } else if n == 0 {
            cell.remove(pos);
            skip = pos;
        } else {
            // I'am not sure which version is faster a loop of 'replace'
            // or allacation of a string for replacement;
            cell.replace_range(pos..pos + 1, &" ".repeat(n));
            skip = pos + 1;
        }

        if cell.is_empty() || skip >= cell.len() {
            break;
        }
    }

    cell
}

// only valid to call for stabilized widths.
fn total_width(
    grid: &Grid,
    widths: &[Vec<usize>],
    styles: &[Vec<Style>],
    margin: &Margin,
) -> usize {
    let content_width = widths
        .iter()
        .next()
        .map(|row| row.iter().sum::<usize>())
        .unwrap_or(0);

    let count_borders = if grid.count_columns() == 0 {
        0
    } else {
        let rest_borders = (0..grid.count_columns())
            .filter(|&col| is_cell_visible(&styles[0], col))
            .map(|col| has_vertical(grid, col))
            .filter(|b| *b)
            .count();
        let last_col_border = has_vertical(grid, grid.count_columns()) as usize;

        last_col_border + rest_borders
    };

    content_width + count_borders + margin.left.size + margin.right.size
}

#[derive(Debug, Clone)]
struct Theme {
    borders: Borders,
    override_borders: BordersMap,
    override_lines: HashMap<usize, Line>,
}

#[derive(Debug, Clone, Default)]
pub struct Borders {
    pub top: Option<Symbol>,
    pub top_left: Option<Symbol>,
    pub top_right: Option<Symbol>,
    pub top_intersection: Option<Symbol>,

    pub bottom: Option<Symbol>,
    pub bottom_left: Option<Symbol>,
    pub bottom_right: Option<Symbol>,
    pub bottom_intersection: Option<Symbol>,

    pub horizontal: Option<Symbol>,
    pub horizontal_left: Option<Symbol>,
    pub horizontal_right: Option<Symbol>,

    pub vertical_left: Option<Symbol>,
    pub vertical_intersection: Option<Symbol>,
    pub vertical_right: Option<Symbol>,

    pub intersection: Option<Symbol>,
}

#[derive(Debug, Clone)]
struct BordersMap {
    vertical: HashMap<Position, Symbol>,
    horizontal: HashMap<Position, Symbol>,
    intersection: HashMap<Position, Symbol>,
}

#[derive(Debug, Clone, Default)]
pub struct Line {
    pub horizontal: Option<Symbol>,
    pub intersection: Option<Symbol>,
    pub left: Option<Symbol>,
    pub right: Option<Symbol>,
}

type Position = (usize, usize);

impl Theme {
    fn new() -> Self {
        Self {
            borders: DEFAULT_BORDERS,
            override_borders: BordersMap {
                vertical: HashMap::new(),
                horizontal: HashMap::new(),
                intersection: HashMap::new(),
            },
            override_lines: HashMap::new(),
        }
    }

    // fn get_borders_mut(&mut self) -> &mut Borders {
    //     &mut self.borders
    // }

    fn override_border(&mut self, pos: Position, border: Border) {
        if let Some(c) = border.top {
            self.override_borders.horizontal.insert(pos, c);
        }

        if let Some(c) = border.bottom {
            self.override_borders
                .horizontal
                .insert((pos.0 + 1, pos.1), c);
        }

        if let Some(c) = border.left {
            self.override_borders.vertical.insert(pos, c);
        }

        if let Some(c) = border.right {
            self.override_borders.vertical.insert((pos.0, pos.1 + 1), c);
        }

        if let Some(c) = border.left_top_corner {
            self.override_borders.intersection.insert((pos.0, pos.1), c);
        }

        if let Some(c) = border.left_bottom_corner {
            self.override_borders
                .intersection
                .insert((pos.0 + 1, pos.1), c);
        }

        if let Some(c) = border.right_top_corner {
            self.override_borders
                .intersection
                .insert((pos.0, pos.1 + 1), c);
        }

        if let Some(c) = border.right_bottom_corner {
            self.override_borders
                .intersection
                .insert((pos.0 + 1, pos.1 + 1), c);
        }
    }

    fn override_line(&mut self, row: usize, line: Line) {
        self.override_lines.insert(row, line);
    }

    // we can take only a border of a cell
    // which is a pitty,
    // would be cool if we could take a border of any Entity
    fn get_border(&self, pos: Position, count_rows: usize, count_cols: usize) -> Border {
        let is_first_row = pos.0 == 0;
        let is_last_row = pos.0 + 1 == count_rows;
        let is_first_col = pos.1 == 0;
        let is_last_col = pos.1 + 1 == count_cols;

        let top = if is_first_row {
            &self.borders.top
        } else {
            &self.borders.horizontal
        };

        let bottom = if is_last_row {
            &self.borders.bottom
        } else {
            &self.borders.horizontal
        };

        let left = if is_first_col {
            &self.borders.vertical_left
        } else {
            &self.borders.vertical_intersection
        };

        let left_top_corner = if is_first_row && is_first_col {
            &self.borders.top_left
        } else if is_first_col {
            &self.borders.horizontal_left
        } else {
            &self.borders.intersection
        };

        let left_bottom_corner = if is_last_row && is_first_col {
            &self.borders.bottom_left
        } else if is_first_col {
            &self.borders.horizontal_left
        } else {
            &self.borders.intersection
        };

        let right = if is_last_col {
            &self.borders.vertical_right
        } else {
            &self.borders.vertical_intersection
        };

        let right_top_corner = if is_first_row && is_last_col {
            &self.borders.top_right
        } else if is_last_col {
            &self.borders.horizontal_right
        } else {
            &self.borders.intersection
        };

        let right_bottom_corner = if is_last_row && is_last_col {
            &self.borders.bottom_right
        } else if is_last_col {
            &self.borders.horizontal_right
        } else {
            &self.borders.intersection
        };

        let mut border = Border {
            top: top.clone(),
            bottom: bottom.clone(),
            left: left.clone(),
            left_top_corner: left_top_corner.clone(),
            left_bottom_corner: left_bottom_corner.clone(),
            right: right.clone(),
            right_top_corner: right_top_corner.clone(),
            right_bottom_corner: right_bottom_corner.clone(),
        };

        if let Some(line) = self.override_lines.get(&pos.0) {
            border.top = line.horizontal.clone().or(border.top);

            if is_first_col {
                border.left_top_corner = line.left.clone().or(border.left_top_corner);
            } else {
                border.left_top_corner = line.intersection.clone().or(border.left_top_corner);
            }

            if is_last_col {
                border.right_top_corner = line.right.clone().or(border.right_top_corner);
            } else {
                border.right_top_corner = line.intersection.clone().or(border.right_top_corner);
            }
        }

        if let Some(line) = self.override_lines.get(&(pos.0 + 1)) {
            border.bottom = line.horizontal.clone().or(border.bottom);

            if is_first_col {
                border.left_bottom_corner = line.left.clone().or(border.left_bottom_corner);
            } else {
                border.left_bottom_corner = line.intersection.clone().or(border.left_bottom_corner);
            }

            if is_last_col {
                border.right_bottom_corner = line.right.clone().or(border.right_bottom_corner);
            } else {
                border.right_bottom_corner =
                    line.intersection.clone().or(border.right_bottom_corner);
            }
        }

        if let Some(b) = self.get_override_border(&pos) {
            border.top = b.top.or(border.top);
            border.bottom = b.bottom.or(border.bottom);
            border.left = b.left.or(border.left);
            border.left_top_corner = b.left_top_corner.or(border.left_top_corner);
            border.left_bottom_corner = b.left_bottom_corner.or(border.left_bottom_corner);
            border.right = b.right.or(border.right);
            border.right_top_corner = b.right_top_corner.or(border.right_top_corner);
            border.right_bottom_corner = b.right_bottom_corner.or(border.right_bottom_corner);
        }

        border
    }

    fn get_vertical(&self, pos: Position, count_cols: usize) -> Option<&Symbol> {
        let use_left = pos.1 == 0;
        let use_right = pos.1 == count_cols;

        if let Some(b) = self.override_borders.vertical.get(&pos) {
            return Some(b);
        }

        if use_right {
            self.borders.vertical_right.as_ref()
        } else if use_left {
            self.borders.vertical_left.as_ref()
        } else {
            self.borders.vertical_intersection.as_ref()
        }
    }

    fn get_horizontal(&self, pos: Position, count_rows: usize) -> Option<&Symbol> {
        let use_top = pos.0 == 0;
        let use_bottom = pos.0 == count_rows;

        if let Some(b) = self.override_borders.horizontal.get(&pos) {
            return Some(b);
        }

        if let Some(line) = self.override_lines.get(&pos.0) {
            if line.horizontal.is_some() {
                return line.horizontal.as_ref();
            }
        }

        if use_top {
            self.borders.top.as_ref()
        } else if use_bottom {
            self.borders.bottom.as_ref()
        } else {
            self.borders.horizontal.as_ref()
        }
    }

    fn get_intersection(
        &self,
        pos: Position,
        count_rows: usize,
        count_cols: usize,
    ) -> Option<&Symbol> {
        let use_top = pos.0 == 0;
        let use_bottom = pos.0 == count_rows;

        let use_left = pos.1 == 0;
        let use_right = pos.1 == count_cols;

        if let Some(b) = self.override_borders.intersection.get(&pos) {
            return Some(b);
        }

        if let Some(line) = self.override_lines.get(&pos.0) {
            if use_left && line.left.is_some() {
                return line.left.as_ref();
            }

            if use_right && line.right.is_some() {
                return line.right.as_ref();
            }

            if !use_right && !use_left && line.intersection.is_some() {
                return line.intersection.as_ref();
            }
        }

        if use_top && use_left {
            self.borders.top_left.as_ref()
        } else if use_top && use_right {
            self.borders.top_right.as_ref()
        } else if use_bottom && use_left {
            self.borders.bottom_left.as_ref()
        } else if use_bottom && use_right {
            self.borders.bottom_right.as_ref()
        } else if use_top {
            self.borders.top_intersection.as_ref()
        } else if use_bottom {
            self.borders.bottom_intersection.as_ref()
        } else if use_left {
            self.borders.horizontal_left.as_ref()
        } else if use_right {
            self.borders.horizontal_right.as_ref()
        } else {
            self.borders.intersection.as_ref()
        }
    }

    fn get_override_border(&self, pos: &Position) -> Option<Border> {
        let top = self.override_borders.horizontal.get(pos);
        let bottom = self.override_borders.horizontal.get(&(pos.0 + 1, pos.1));
        let left = self.override_borders.vertical.get(pos);
        let right = self.override_borders.vertical.get(&(pos.0, pos.1 + 1));
        let left_top = self.override_borders.intersection.get(pos);
        let left_bottom = self.override_borders.intersection.get(&(pos.0 + 1, pos.1));
        let right_top = self.override_borders.intersection.get(&(pos.0, pos.1 + 1));
        let right_bottom = self
            .override_borders
            .intersection
            .get(&(pos.0 + 1, pos.1 + 1));

        if top.is_some()
            || bottom.is_some()
            || left.is_some()
            || right.is_some()
            || left_top.is_some()
            || left_bottom.is_some()
            || right_top.is_some()
            || right_bottom.is_some()
        {
            return Some(Border {
                top: top.cloned(),
                bottom: bottom.cloned(),
                left: left.cloned(),
                left_top_corner: left_top.cloned(),
                left_bottom_corner: left_bottom.cloned(),
                right: right.cloned(),
                right_top_corner: right_top.cloned(),
                right_bottom_corner: right_bottom.cloned(),
            });
        }

        None
    }
}

/// A single character representation.
///
/// It uses String to support ANSI colors.
#[cfg(feature = "color")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Symbol(InnerSymbol);

#[cfg(feature = "color")]
#[derive(Debug, Clone, Eq, PartialEq)]
enum InnerSymbol {
    Ansi(String),
    Char(char),
}

/// A single character representation.
///
/// It uses String to support ANSI colors.
#[cfg(not(feature = "color"))]
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Symbol(char);

impl Symbol {
    #[cfg(feature = "color")]
    pub fn ansi(s: String) -> Option<Self> {
        let mut chars = s.chars();
        let c = chars.next();
        let no_other_chars = chars.next().is_none();
        drop(chars);
        match c {
            Some(c) if no_other_chars => return Some(Self(InnerSymbol::Char(c))),
            _ => (),
        }

        if string_width(&s) != 1 {
            return None;
        }

        Some(Self(InnerSymbol::Ansi(s)))
    }

    pub const fn from_char(c: char) -> Self {
        #[cfg(feature = "color")]
        {
            Self(InnerSymbol::Char(c))
        }

        #[cfg(not(feature = "color"))]
        {
            Self(c)
        }
    }
}

#[cfg(feature = "color")]
impl Default for Symbol {
    fn default() -> Self {
        Self(InnerSymbol::Char(char::default()))
    }
}

impl From<char> for Symbol {
    fn from(c: char) -> Self {
        Self::from_char(c)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "color")]
        {
            match &self.0 {
                InnerSymbol::Ansi(s) => f.write_str(s),
                InnerSymbol::Char(c) => f.write_char(*c),
            }
        }

        #[cfg(not(feature = "color"))]
        {
            f.write_char(self.0)
        }
    }
}

fn bounds_to_usize(left: Bound<&usize>, right: Bound<&usize>, length: usize) -> (usize, usize) {
    match (left, right) {
        (Bound::Included(x), Bound::Included(y)) => (*x, y + 1),
        (Bound::Included(x), Bound::Excluded(y)) => (*x, *y),
        (Bound::Included(x), Bound::Unbounded) => (*x, length),
        (Bound::Unbounded, Bound::Unbounded) => (0, length),
        (Bound::Unbounded, Bound::Included(y)) => (0, y + 1),
        (Bound::Unbounded, Bound::Excluded(y)) => (0, *y),
        (Bound::Excluded(_), _) => {
            unreachable!("A start bound can't be excluded")
        }
    }
}

fn print_grid(
    f: &mut fmt::Formatter,
    grid: &Grid,
    contents: Vec<Vec<Vec<String>>>,
    styles: Vec<Vec<Style>>,
    widths: Vec<Vec<usize>>,
    normal_widths: Vec<usize>,
    heights: Vec<usize>,
) -> fmt::Result {
    let table_width = row_width_grid(grid, &widths, 0);
    print_margin_top(f, &grid.margin, table_width)?;

    for row in 0..grid.count_rows() {
        print_split_line(f, grid, &normal_widths, table_width, row)?;

        let height = heights[row];

        for i in 0..height {
            print_margin_left(f, &grid.margin)?;

            for col in 0..grid.count_columns() {
                let width = widths[row][col];
                let lines = contents[row][col].clone();
                let style = styles[row][col].clone();
                let border = grid.get_border(row, col);

                if is_cell_visible(&styles[row], col) {
                    if let Some(c) = border.left {
                        c.fmt(f)?;
                    }

                    build_line_cell(f, i, &lines, &style, width, height)?;
                }

                let is_last_column = col + 1 == grid.count_columns();
                if is_last_column {
                    if let Some(c) = border.right {
                        c.fmt(f)?;
                    }
                }
            }

            print_margin_right(f, &grid.margin)?;

            f.write_char('\n')?;
        }

        let is_last_row = row + 1 == grid.count_rows();
        if is_last_row {
            print_split_line(f, grid, &normal_widths, table_width, row + 1)?;
        }
    }

    print_margin_bottom(f, &grid.margin, table_width)?;

    Ok(())
}

fn print_margin_top(f: &mut fmt::Formatter, margin: &Margin, table_width: usize) -> fmt::Result {
    let size = table_width + margin.left.size + margin.right.size;
    let fill = Symbol::from_char(margin.top.fill);
    for _ in 0..margin.top.size {
        repeat_char(f, &fill, size)?;
        f.write_char('\n')?
    }

    Ok(())
}

fn print_margin_bottom(f: &mut fmt::Formatter, margin: &Margin, table_width: usize) -> fmt::Result {
    let size = table_width + margin.left.size + margin.right.size;
    let fill = Symbol::from_char(margin.bottom.fill);
    for _ in 0..margin.bottom.size {
        repeat_char(f, &fill, size)?;
        f.write_char('\n')?
    }

    Ok(())
}

fn print_margin_left(f: &mut fmt::Formatter, margin: &Margin) -> fmt::Result {
    repeat_char(f, &Symbol::from_char(margin.left.fill), margin.left.size)
}

fn print_margin_right(f: &mut fmt::Formatter, margin: &Margin) -> fmt::Result {
    repeat_char(f, &Symbol::from_char(margin.right.fill), margin.right.size)
}

fn print_split_line(
    f: &mut fmt::Formatter,
    grid: &Grid,
    widths: &[usize],
    max_width: usize,
    row: usize,
) -> fmt::Result {
    if !has_horizontal(grid, row) {
        return Ok(());
    }

    print_margin_left(f, &grid.margin)?;

    let mut char_skip = 0;
    let override_text = grid.override_split_lines.get(&row);
    if let Some(text) = override_text {
        let text = strip(text, max_width);
        let text = text.lines().next().unwrap();
        char_skip = string_width(text);
        f.write_str(text)?;
    }

    for (col, width) in widths.iter().enumerate() {
        if col == 0 {
            let left = if let Some(c) =
                grid.theme
                    .get_intersection((row, col), grid.count_rows(), grid.count_columns())
            {
                Some(c.clone())
            } else if has_vertical(grid, col) {
                Some(DEFAULT_BORDER_VERTICAL_CHAR)
            } else {
                None
            };

            if let Some(c) = left {
                if char_skip == 0 {
                    c.fmt(f)?;
                } else {
                    char_skip -= 1;
                }
            }
        }

        let main = grid
            .theme
            .get_horizontal((row, col), grid.count_rows())
            .cloned()
            .or(Some(DEFAULT_BORDER_HORIZONTAL_CHAR));

        if let Some(c) = main {
            let mut width = *width;
            if char_skip > 0 {
                let sub = cmp::min(width, char_skip);
                width -= sub;
                char_skip -= sub;
            }

            repeat_char(f, &c, width)?;
        }

        let right = grid
            .theme
            .get_intersection((row, col + 1), grid.count_rows(), grid.count_columns())
            .cloned()
            .or_else(|| {
                if has_vertical(grid, col + 1) {
                    Some(DEFAULT_BORDER_VERTICAL_CHAR)
                } else {
                    None
                }
            });

        if let Some(c) = right {
            if char_skip == 0 {
                c.fmt(f)?;
            } else {
                char_skip -= 1;
            }
        }
    }

    print_margin_right(f, &grid.margin)?;

    f.write_char('\n')?;

    Ok(())
}

// fn override_split_line(v: &mut Vec<Container>, text: String) {
//     let width = string_width(&text);

//     let mut i = width;
//     while !v.is_empty() {
//         if i == 0 {
//             break;
//         }

//         let mut c = v.remove(0);
//         let w = c.width;
//         if i < w {
//             c.width -= i;
//             v.insert(0, c);
//         }

//         i -= cmp::min(w, i);
//     }

//     v.insert(
//         0,
//         Container::new(
//             width,
//             1,
//             ContainerKind::Content {
//                 lines: vec![text],
//                 style: Style::default(),
//             },
//         ),
//     );
// }

fn row_width_grid(grid: &Grid, widths: &[Vec<usize>], row: usize) -> usize {
    let row_width = widths
        .get(row)
        .map(|l| l.iter().sum::<usize>())
        .unwrap_or(0);
    let count_borders = (0..grid.count_columns())
        .filter(|&col| has_vertical(grid, col))
        .count()
        + has_vertical(grid, grid.count_columns()) as usize;

    row_width + count_borders
}

fn has_vertical(grid: &Grid, col: usize) -> bool {
    (0..grid.count_rows())
        .map(|row| grid.theme.get_vertical((row, col), grid.count_columns()))
        .any(|c| c.is_some())
}

fn has_horizontal(grid: &Grid, row: usize) -> bool {
    (0..grid.count_columns())
        .map(|col| grid.theme.get_horizontal((row, col), grid.count_rows()))
        .any(|c| c.is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn container_print_test() {
    //     let c = Container::new(
    //         12,
    //         4,
    //         ContainerKind::Columns(vec![
    //             Container::new(1, 4, ContainerKind::Split('+'.into())),
    //             Container::new(
    //                 10,
    //                 4,
    //                 ContainerKind::Rows(vec![
    //                     Container::new(
    //                         10,
    //                         2,
    //                         ContainerKind::Content {
    //                             lines: vec!["Hello".to_owned(), "World".to_owned()],
    //                             style: Style::default(),
    //                         },
    //                     ),
    //                     Container::new(10, 1, ContainerKind::Split('-'.into())),
    //                     Container::new(
    //                         10,
    //                         1,
    //                         ContainerKind::Content {
    //                             lines: vec!["123".to_owned()],
    //                             style: Style::default(),
    //                         },
    //                     ),
    //                 ]),
    //             ),
    //             Container::new(1, 3, ContainerKind::Split('#'.into())),
    //         ]),
    //     );

    //     assert_eq!(
    //         c.to_string(),
    //         "+Hello     #\n\
    //          +World     #\n\
    //          +----------#\n\
    //          +123       #\n",
    //     )
    // }

    #[test]
    fn replace_tab_test() {
        assert_eq!(
            replace_tab(&mut "123\t\tabc\t".to_owned(), 3),
            "123      abc   "
        );

        assert_eq!(replace_tab(&mut "\t".to_owned(), 0), "");
        assert_eq!(replace_tab(&mut "\t".to_owned(), 3), "   ");
        assert_eq!(replace_tab(&mut "123\tabc".to_owned(), 3), "123   abc");
        assert_eq!(replace_tab(&mut "123\tabc\tzxc".to_owned(), 0), "123abczxc");

        assert_eq!(replace_tab(&mut "\\t".to_owned(), 0), "\\t");
        assert_eq!(replace_tab(&mut "\\t".to_owned(), 4), "\\t");
        assert_eq!(replace_tab(&mut "123\\tabc".to_owned(), 0), "123\\tabc");
        assert_eq!(replace_tab(&mut "123\\tabc".to_owned(), 4), "123\\tabc");
    }

    #[test]
    fn string_width_emojie_test() {
        // ...emojis such as “joy”, which normally take up two columns when printed in a terminal
        // https://github.com/mgeisler/textwrap/pull/276
        assert_eq!(string_width("🎩"), 2);
        assert_eq!(string_width("Rust 💕"), 7);
        assert_eq!(string_width("Go 👍\nC 😎"), 5);
    }

    #[test]
    fn horizontal_aligment_test() {
        use std::fmt;

        struct F<'a>(&'a str, AlignmentHorizontal, usize);

        impl fmt::Display for F<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let w = string_width(self.0);
                self.1.align_with_max_width(f, self.0, self.2, w, w)
            }
        }

        assert_eq!(F("AAA", AlignmentHorizontal::Right, 4).to_string(), " AAA");
        assert_eq!(F("AAA", AlignmentHorizontal::Left, 4).to_string(), "AAA ");
        assert_eq!(F("AAA", AlignmentHorizontal::Center, 4).to_string(), "AAA ");
        assert_eq!(F("🎩", AlignmentHorizontal::Center, 4).to_string(), " 🎩 ");
        assert_eq!(F("🎩", AlignmentHorizontal::Center, 3).to_string(), "🎩 ");

        #[cfg(feature = "color")]
        {
            use owo_colors::OwoColorize;
            let text = "Colored Text".red().to_string();
            assert_eq!(
                F(&text, AlignmentHorizontal::Center, 15).to_string(),
                format!(" {}  ", text)
            );
        }
    }

    #[test]
    fn vertical_aligment_test() {
        assert_eq!(AlignmentVertical::Bottom.top_ident(1, 1), 0);
        assert_eq!(AlignmentVertical::Top.top_ident(1, 1), 0);
        assert_eq!(AlignmentVertical::Center.top_ident(1, 1), 0);
        assert_eq!(AlignmentVertical::Bottom.top_ident(3, 1), 2);
        assert_eq!(AlignmentVertical::Top.top_ident(3, 1), 0);
        assert_eq!(AlignmentVertical::Center.top_ident(3, 1), 1);
        assert_eq!(AlignmentVertical::Center.top_ident(4, 1), 1);
    }

    #[cfg(feature = "color")]
    #[test]
    fn colored_string_width_test() {
        use owo_colors::OwoColorize;
        assert_eq!(string_width(&"hello world".red().to_string()), 11);
        assert_eq!(string_width(&"hello\nworld".blue().to_string()), 5);
        assert_eq!(string_width("\u{1b}[34m0\u{1b}[0m"), 1);
        assert_eq!(string_width(&"0".red().to_string()), 1);
    }
}
