use tabled::{
    object::{Cell, Columns, Frame, Object, Rows, Segment},
    style::Border,
    Highlight, Style, Table, Tabled,
};

use crate::util::{create_vector, static_table};

mod util;

#[test]
fn highlingt_object_exceeds_bounderies() {
    let data = create_vector::<3, 3>();
    let _table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Cell(1000, 0), Border::filled('+')))
        .to_string();
}

#[test]
fn highlingt_empty_table() {
    #[derive(Tabled)]
    struct EmptyStruct;

    let data: [EmptyStruct; 0] = [];
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Segment::all(), Border::filled('+')))
        .to_string();

    assert_eq!(table, "");
}

#[test]
fn highlingt_cell() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Cell(0, 0), Border::filled('+')))
        .with(Highlight::new(Cell(1, 1), Border::filled('*')))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+++++──────────┬──────────┬──────────┐"
            "+ N + column 0 │ column 1 │ column 2 │"
            "++++************──────────┼──────────┤"
            "│ 0 *   0-0    *   0-1    │   0-2    │"
            "├───************──────────┼──────────┤"
            "│ 1 │   1-0    │   1-1    │   1-2    │"
            "├───┼──────────┼──────────┼──────────┤"
            "│ 2 │   2-0    │   2-1    │   2-2    │"
            "└───┴──────────┴──────────┴──────────┘"
        )
    );
}

#[test]
fn highlingt_row() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Rows::single(0), Border::filled('+')))
        .with(Highlight::new(Rows::single(3), Border::filled('*')))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "++++++++++++++++++++++++++++++++++++++"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "++++++++++++++++++++++++++++++++++++++"
            "│ 0 │   0-0    │   0-1    │   0-2    │"
            "├───┼──────────┼──────────┼──────────┤"
            "│ 1 │   1-0    │   1-1    │   1-2    │"
            "**************************************"
            "* 2 │   2-0    │   2-1    │   2-2    *"
            "**************************************"
        )
    );
}

#[test]
fn highlingt_column() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Columns::single(0), Border::filled('+')))
        .with(Highlight::new(Columns::single(2), Border::filled('*')))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "+++++──────────************──────────┐"
            "+ N + column 0 * column 1 * column 2 │"
            "+───+──────────*──────────*──────────┤"
            "+ 0 +   0-0    *   0-1    *   0-2    │"
            "+───+──────────*──────────*──────────┤"
            "+ 1 +   1-0    *   1-1    *   1-2    │"
            "+───+──────────*──────────*──────────┤"
            "+ 2 +   2-0    *   2-1    *   2-2    │"
            "+++++──────────************──────────┘"
        )
    );
}

#[test]
fn highlingt_row_range() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Rows::new(1..3), Border::filled('+')))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "┌───┬──────────┬──────────┬──────────┐"
            "│ N │ column 0 │ column 1 │ column 2 │"
            "++++++++++++++++++++++++++++++++++++++"
            "+ 0 │   0-0    │   0-1    │   0-2    +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 1 │   1-0    │   1-1    │   1-2    +"
            "++++++++++++++++++++++++++++++++++++++"
            "│ 2 │   2-0    │   2-1    │   2-2    │"
            "└───┴──────────┴──────────┴──────────┘"
        )
    );
}

#[test]
fn highlingt_column_range() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Columns::new(..2), Border::filled('+')))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "++++++++++++++++──────────┬──────────┐"
            "+ N │ column 0 + column 1 │ column 2 │"
            "+───┼──────────+──────────┼──────────┤"
            "+ 0 │   0-0    +   0-1    │   0-2    │"
            "+───┼──────────+──────────┼──────────┤"
            "+ 1 │   1-0    +   1-1    │   1-2    │"
            "+───┼──────────+──────────┼──────────┤"
            "+ 2 │   2-0    +   2-1    │   2-2    │"
            "++++++++++++++++──────────┴──────────┘"
        )
    );
}

#[test]
fn highlingt_frame() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(
            Frame,
            Border::filled('+')
                .top_left_corner('*')
                .top_right_corner('#')
                .bottom_left_corner('@')
                .bottom_right_corner('.'),
        ))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 0 │   0-0    │   0-1    │   0-2    +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 1 │   1-0    │   1-1    │   1-2    +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 2 │   2-0    │   2-1    │   2-2    +"
            "@++++++++++++++++++++++++++++++++++++."
        )
    );
}

#[test]
fn highlingt_full() {
    let data = create_vector::<3, 3>();

    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(
            Segment::all(),
            Border::filled('+')
                .top_left_corner('*')
                .top_right_corner('#')
                .bottom_left_corner('@')
                .bottom_right_corner('.'),
        ))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 0 │   0-0    │   0-1    │   0-2    +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 1 │   1-0    │   1-1    │   1-2    +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 2 │   2-0    │   2-1    │   2-2    +"
            "@++++++++++++++++++++++++++++++++++++."
        )
    );
}

#[test]
fn highlingt_single_column() {
    let data = create_vector::<3, 0>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(
            Cell(0, 0),
            Border::default().left('*').top('x'),
        ))
        .with(Highlight::new(Rows::new(1..3), Border::default().left('n')))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "┌xxx┐"
            "* N │"
            "├───┤"
            "n 0 │"
            "n───┤"
            "n 1 │"
            "├───┤"
            "│ 2 │"
            "└───┘"
        )
    );
}

#[test]
fn highlingt_complex_figures() {
    macro_rules! test_highlight {
        ($object:expr, $expected:expr,) => {
            let data = create_vector::<3, 3>();
            let border = Border::filled('+')
                .top_left_corner('*')
                .top_right_corner('#')
                .bottom_left_corner('@')
                .bottom_right_corner('.');

            let table = Table::new(&data)
                .with(Style::modern())
                .with(Highlight::new($object, border))
                .to_string();

            assert_eq!(table, $expected);
        };
    }

    test_highlight!(
        Segment::all().not(Segment::new(2.., 1..3)),
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 0 │   0-0    │   0-1    │   0-2    +"
            "+───*+++++++++++++++++++++#──────────+"
            "+ 1 +   1-0    │   1-1    +   1-2    +"
            "+───+──────────┼──────────+──────────+"
            "+ 2 +   2-0    │   2-1    +   2-2    +"
            "@+++.──────────┴──────────@++++++++++."
        ),
    );

    test_highlight!(
        Segment::all()
            .not(Segment::new(0..1, 1..3))
            .not(Columns::single(0)),
        static_table!(
            "┌───┬──────────┬──────────*++++++++++#"
            "│ N │ column 0 │ column 1 + column 2 +"
            "├───*+++++++++++++++++++++.──────────+"
            "│ 0 +   0-0    │   0-1    │   0-2    +"
            "├───+──────────┼──────────┼──────────+"
            "│ 1 +   1-0    │   1-1    │   1-2    +"
            "├───+──────────┼──────────┼──────────+"
            "│ 2 +   2-0    │   2-1    │   2-2    +"
            "└───@++++++++++++++++++++++++++++++++."
        ),
    );

    test_highlight!(
        Segment::all().not(Segment::new(0..1, 1..3)),
        static_table!(
            "*+++#──────────┬──────────*++++++++++#"
            "+ N + column 0 │ column 1 + column 2 +"
            "+───@+++++++++++++++++++++.──────────+"
            "+ 0 │   0-0    │   0-1    │   0-2    +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 1 │   1-0    │   1-1    │   1-2    +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 2 │   2-0    │   2-1    │   2-2    +"
            "@++++++++++++++++++++++++++++++++++++."
        ),
    );

    test_highlight!(
        Segment::all().not(Segment::new(1..2, 1..3)),
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "+───*+++++++++++++++++++++#──────────+"
            "+ 0 +   0-0    │   0-1    +   0-2    +"
            "+───@+++++++++++++++++++++.──────────+"
            "+ 1 │   1-0    │   1-1    │   1-2    +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 2 │   2-0    │   2-1    │   2-2    +"
            "@++++++++++++++++++++++++++++++++++++."
        ),
    );

    test_highlight!(
        Cell(0, 0).and(Cell(3, 3)).and(Cell(0, 3)).and(Cell(3, 0)),
        static_table!(
            "*+++#──────────┬──────────*++++++++++#"
            "+ N + column 0 │ column 1 + column 2 +"
            "@+++.──────────┼──────────@++++++++++."
            "│ 0 │   0-0    │   0-1    │   0-2    │"
            "├───┼──────────┼──────────┼──────────┤"
            "│ 1 │   1-0    │   1-1    │   1-2    │"
            "*+++#──────────┼──────────*++++++++++#"
            "+ 2 +   2-0    │   2-1    +   2-2    +"
            "@+++.──────────┴──────────@++++++++++."
        ),
    );

    test_highlight!(
        Rows::single(0).and(Rows::single(3)),
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "@++++++++++++++++++++++++++++++++++++."
            "│ 0 │   0-0    │   0-1    │   0-2    │"
            "├───┼──────────┼──────────┼──────────┤"
            "│ 1 │   1-0    │   1-1    │   1-2    │"
            "*++++++++++++++++++++++++++++++++++++#"
            "+ 2 │   2-0    │   2-1    │   2-2    +"
            "@++++++++++++++++++++++++++++++++++++."
        ),
    );

    test_highlight!(
        Columns::single(0).and(Columns::single(3)),
        static_table!(
            "*+++#──────────┬──────────*++++++++++#"
            "+ N + column 0 │ column 1 + column 2 +"
            "+───+──────────┼──────────+──────────+"
            "+ 0 +   0-0    │   0-1    +   0-2    +"
            "+───+──────────┼──────────+──────────+"
            "+ 1 +   1-0    │   1-1    +   1-2    +"
            "+───+──────────┼──────────+──────────+"
            "+ 2 +   2-0    │   2-1    +   2-2    +"
            "@+++.──────────┴──────────@++++++++++."
        ),
    );

    test_highlight!(
        Segment::all().not(Cell(3, 1).and(Cell(3, 2))),
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 0 │   0-0    │   0-1    │   0-2    +"
            "+───┼──────────┼──────────┼──────────+"
            "+ 1 │   1-0    │   1-1    │   1-2    +"
            "+───*+++++++++++++++++++++#──────────+"
            "+ 2 +   2-0    │   2-1    +   2-2    +"
            "@+++.──────────┴──────────@++++++++++."
        ),
    );

    test_highlight!(
        Rows::single(0)
            .and(Cell(1, 1).and(Cell(1, 2)))
            .and(Cell(2, 3)),
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "@+++#──────────┼──────────*++++++++++."
            "│ 0 +   0-0    │   0-1    +   0-2    │"
            "├───@+++++++++++++++++++++*++++++++++#"
            "│ 1 │   1-0    │   1-1    +   1-2    +"
            "├───┼──────────┼──────────@++++++++++."
            "│ 2 │   2-0    │   2-1    │   2-2    │"
            "└───┴──────────┴──────────┴──────────┘"
        ),
    );

    test_highlight!(
        Segment::all().not(Segment::new(2.., 0..3)).not(Cell(1, 0)),
        static_table!(
            "*++++++++++++++++++++++++++++++++++++#"
            "+ N │ column 0 │ column 1 │ column 2 +"
            "@+++#──────────┼──────────┼──────────+"
            "│ 0 +   0-0    │   0-1    │   0-2    +"
            "├───@+++++++++++++++++++++#──────────+"
            "│ 1 │   1-0    │   1-1    +   1-2    +"
            "├───┼──────────┼──────────+──────────+"
            "│ 2 │   2-0    │   2-1    +   2-2    +"
            "└───┴──────────┴──────────@++++++++++."
        ),
    );

    test_highlight!(
        Segment::all()
            .not(Segment::new(..1, 1..))
            .not(Segment::new(1..2, 2..))
            .not(Cell(2, 3)),
        static_table!(
            "*+++#──────────┬──────────┬──────────┐"
            "+ N + column 0 │ column 1 │ column 2 │"
            "+───@++++++++++#──────────┼──────────┤"
            "+ 0 │   0-0    +   0-1    │   0-2    │"
            "+───┼──────────@++++++++++#──────────┤"
            "+ 1 │   1-0    │   1-1    +   1-2    │"
            "+───┼──────────┼──────────@++++++++++#"
            "+ 2 │   2-0    │   2-1    │   2-2    +"
            "@++++++++++++++++++++++++++++++++++++."
        ),
    );
}

#[test]
fn highlingt_several_times() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::modern())
        .with(Highlight::new(Frame, Border::filled('*')))
        .with(Highlight::new(Cell(1, 1), Border::filled('#')))
        .with(Highlight::new(Columns::single(3), Border::filled('x')))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "**************************xxxxxxxxxxxx"
            "* N │ column 0 │ column 1 x column 2 x"
            "*───############──────────x──────────x"
            "* 0 #   0-0    #   0-1    x   0-2    x"
            "*───############──────────x──────────x"
            "* 1 │   1-0    │   1-1    x   1-2    x"
            "*───┼──────────┼──────────x──────────x"
            "* 2 │   2-0    │   2-1    x   2-2    x"
            "**************************xxxxxxxxxxxx"
        )
    );
}

// @todo
//
// #[test]
// fn highlingt_empty_border() {
//     let data = create_vector::<3, 3>();
//     let table = Table::new(&data)
//         .with(Style::modern())
//         .with(Highlight::new(Frame, Border::empty()))
//         .to_string();

//     let expected = static_table!(
//         " N │ column 0 │ column 1 │ column 2 "
//         "───                       ──────────"
//         " 0     0-0    │   0-1        0-2    "
//         "─── ──────────┼────────── ──────────"
//         " 1     1-0    │   1-1        1-2    "
//         "───                       ──────────"
//         " 2 │   2-0    │   2-1    │   2-2    "
//     );

//     println!("{}", table);

//     assert_eq!(table, expected);
// }
