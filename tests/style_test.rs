use crate::util::create_vector;
use tabled::style::{Line, TopBorderText};
use tabled::{Style, Table};

mod util;

#[test]
fn default_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::ASCII).to_string();

    let expected = concat!(
        "+---+----------+----------+----------+\n",
        "| N | column 0 | column 1 | column 2 |\n",
        "+---+----------+----------+----------+\n",
        "| 0 |   0-0    |   0-1    |   0-2    |\n",
        "+---+----------+----------+----------+\n",
        "| 1 |   1-0    |   1-1    |   1-2    |\n",
        "+---+----------+----------+----------+\n",
        "| 2 |   2-0    |   2-1    |   2-2    |\n",
        "+---+----------+----------+----------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn psql_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::PSQL).to_string();

    let expected = concat!(
        " N | column 0 | column 1 | column 2 \n",
        "---+----------+----------+----------\n",
        " 0 |   0-0    |   0-1    |   0-2    \n",
        " 1 |   1-0    |   1-1    |   1-2    \n",
        " 2 |   2-0    |   2-1    |   2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn github_markdown_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::GITHUB_MARKDOWN).to_string();

    let expected = concat!(
        "| N | column 0 | column 1 | column 2 |\n",
        "|---+----------+----------+----------|\n",
        "| 0 |   0-0    |   0-1    |   0-2    |\n",
        "| 1 |   1-0    |   1-1    |   1-2    |\n",
        "| 2 |   2-0    |   2-1    |   2-2    |\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn pseudo_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::PSEUDO).to_string();

    let expected = concat!(
        "┌───┬──────────┬──────────┬──────────┐\n",
        "│ N │ column 0 │ column 1 │ column 2 │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 0 │   0-0    │   0-1    │   0-2    │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
        "└───┴──────────┴──────────┴──────────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn pseudo_clean_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::PSEUDO_CLEAN).to_string();

    let expected = concat!(
        "┌───┬──────────┬──────────┬──────────┐\n",
        "│ N │ column 0 │ column 1 │ column 2 │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 0 │   0-0    │   0-1    │   0-2    │\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
        "└───┴──────────┴──────────┴──────────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn blank_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::BLANK).to_string();

    let expected = concat!(
        " N   column 0   column 1   column 2 \n",
        " 0     0-0        0-1        0-2    \n",
        " 1     1-0        1-1        1-2    \n",
        " 2     2-0        2-1        2-2    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn extended_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::EXTENDED).to_string();

    let expected = concat!(
        "╔═══╦══════════╦══════════╦══════════╗\n",
        "║ N ║ column 0 ║ column 1 ║ column 2 ║\n",
        "╠═══╬══════════╬══════════╬══════════╣\n",
        "║ 0 ║   0-0    ║   0-1    ║   0-2    ║\n",
        "╠═══╬══════════╬══════════╬══════════╣\n",
        "║ 1 ║   1-0    ║   1-1    ║   1-2    ║\n",
        "╠═══╬══════════╬══════════╬══════════╣\n",
        "║ 2 ║   2-0    ║   2-1    ║   2-2    ║\n",
        "╚═══╩══════════╩══════════╩══════════╝\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn ascii_dots_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data).with(Style::ASCII_DOTS).to_string();

    let expected = concat!(
        "......................................\n",
        ": N : column 0 : column 1 : column 2 :\n",
        ":...:..........:..........:..........:\n",
        ": 0 :   0-0    :   0-1    :   0-2    :\n",
        ": 1 :   1-0    :   1-1    :   1-2    :\n",
        ": 2 :   2-0    :   2-1    :   2-2    :\n",
        ":...:..........:..........:..........:\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn re_structured_text_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::RE_STRUCTURED_TEXT)
        .to_string();

    let expected = concat!(
        "=== ========== ========== ==========\n",
        " N   column 0   column 1   column 2 \n",
        "=== ========== ========== ==========\n",
        " 0     0-0        0-1        0-2    \n",
        " 1     1-0        1-1        1-2    \n",
        " 2     2-0        2-1        2-2    \n",
        "=== ========== ========== ==========\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn style_head_changes() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSEUDO_CLEAN.header(None))
        .to_string();

    let expected = concat!(
        "┌───┬──────────┬──────────┬──────────┐\n",
        "│ N │ column 0 │ column 1 │ column 2 │\n",
        "│ 0 │   0-0    │   0-1    │   0-2    │\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
        "└───┴──────────┴──────────┴──────────┘\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn style_frame_changes() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::PSEUDO_CLEAN.frame_bottom(None).frame_top(None))
        .to_string();

    let expected = concat!(
        "│ N │ column 0 │ column 1 │ column 2 │\n",
        "├───┼──────────┼──────────┼──────────┤\n",
        "│ 0 │   0-0    │   0-1    │   0-2    │\n",
        "│ 1 │   1-0    │   1-1    │   1-2    │\n",
        "│ 2 │   2-0    │   2-1    │   2-2    │\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn custom_style() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(
            Style::BLANK
                .frame_bottom(Some(Line::short('*', '\'')))
                .split(Some(Line::short('`', '\'')))
                .inner('\''),
        )
        .to_string();

    let expected = concat!(
        " N ' column 0 ' column 1 ' column 2 \n",
        "```'``````````'``````````'``````````\n",
        " 0 '   0-0    '   0-1    '   0-2    \n",
        "```'``````````'``````````'``````````\n",
        " 1 '   1-0    '   1-1    '   1-2    \n",
        "```'``````````'``````````'``````````\n",
        " 2 '   2-0    '   2-1    '   2-2    \n",
        "***'**********'**********'**********\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn style_single_cell() {
    let data = create_vector::<0, 0>();
    let table = Table::new(&data).with(Style::ASCII).to_string();

    let expected = concat!("+---+\n", "| N |\n", "+---+\n",);

    assert_eq!(table, expected);

    let table = Table::new(&data).with(Style::BLANK).to_string();

    let expected = " N \n";

    assert_eq!(table, expected);
}

#[test]
fn top_border_override_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ASCII)
        .with(TopBorderText::new("-Table"))
        .to_string();

    let expected = concat!(
        "-Table---------+----------+\n",
        "| N | column 0 | column 1 |\n",
        "+---+----------+----------+\n",
        "| 0 |   0-0    |   0-1    |\n",
        "+---+----------+----------+\n",
        "| 1 |   1-0    |   1-1    |\n",
        "+---+----------+----------+\n",
    );

    assert_eq!(table, expected);
}

#[test]
fn top_override_doesnt_work_with_style_with_no_top_border_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::PSQL)
        .with(TopBorderText::new("-Table"))
        .to_string();

    let expected = concat!(
        " N | column 0 | column 1 \n",
        "---+----------+----------\n",
        " 0 |   0-0    |   0-1    \n",
        " 1 |   1-0    |   1-1    \n",
    );

    assert_eq!(table, expected);
}

#[test]
fn top_border_override_cleared_after_restyling_test() {
    let data = create_vector::<2, 2>();
    let table = Table::new(&data)
        .with(Style::ASCII)
        .with(TopBorderText::new("-Table"))
        .with(Style::ASCII)
        .to_string();

    let expected = concat!(
        "+---+----------+----------+\n",
        "| N | column 0 | column 1 |\n",
        "+---+----------+----------+\n",
        "| 0 |   0-0    |   0-1    |\n",
        "+---+----------+----------+\n",
        "| 1 |   1-0    |   1-1    |\n",
        "+---+----------+----------+\n",
    );

    assert_eq!(table, expected);
}
