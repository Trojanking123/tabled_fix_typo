use tabled::{
    formatting_settings::TrimStrategy,
    object::{Cell, Columns, Object, Rows, Segment},
    width::{Justify, MinWidth, Width},
    width::{PriorityMax, PriorityMin},
    Alignment, Modify, Panel, Span, Style, Table, Tabled,
};

use crate::util::{create_vector, is_lines_equal, static_table};

mod util;

#[test]
fn max_width() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Columns::new(1..).not(Rows::single(0))).with(Width::truncate(1)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | column 0 | column 1 | column 2 |"
            "|---+----------+----------+----------|"
            "| 0 |    0     |    0     |    0     |"
            "| 1 |    1     |    1     |    1     |"
            "| 2 |    2     |    2     |    2     |"
        )
    );
}

#[test]
fn max_width_with_suffix() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(
            Modify::new(Columns::new(1..).not(Rows::single(0)))
                .with(Width::truncate(2).suffix("...")),
        )
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | column 0 | column 1 | column 2 |"
            "|---+----------+----------+----------|"
            "| 0 |  0-...   |  0-...   |  0-...   |"
            "| 1 |  1-...   |  1-...   |  1-...   |"
            "| 2 |  2-...   |  2-...   |  2-...   |"
        )
    );
}

#[test]
fn max_width_doesnt_icrease_width_if_it_is_smaller() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Columns::new(1..).not(Rows::single(0))).with(Width::truncate(50)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | column 0 | column 1 | column 2 |"
            "|---+----------+----------+----------|"
            "| 0 |   0-0    |   0-1    |   0-2    |"
            "| 1 |   1-0    |   1-1    |   1-2    |"
            "| 2 |   2-0    |   2-1    |   2-2    |"
        )
    );
}

#[test]
fn max_width_wrapped() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Columns::new(1..).not(Rows::single(0))).with(Width::wrap(2)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | column 0 | column 1 | column 2 |"
            "|---+----------+----------+----------|"
            "| 0 |    0-    |    0-    |    0-    |"
            "|   |    0     |    1     |    2     |"
            "| 1 |    1-    |    1-    |    1-    |"
            "|   |    0     |    1     |    2     |"
            "| 2 |    2-    |    2-    |    2-    |"
            "|   |    0     |    1     |    2     |"
        )
    );
}

#[test]
fn max_width_wrapped_does_nothing_if_str_is_smaller() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Columns::new(1..).not(Rows::single(0))).with(Width::wrap(100)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | column 0 | column 1 | column 2 |"
            "|---+----------+----------+----------|"
            "| 0 |   0-0    |   0-1    |   0-2    |"
            "| 1 |   1-0    |   1-1    |   1-2    |"
            "| 2 |   2-0    |   2-1    |   2-2    |"
        )
    );
}

#[test]
fn max_width_wrapped_keep_words() {
    let data = vec!["this is a long sentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| &str              |"
            "|-------------------|"
            "| this is a long    |"
            "| sentence          |"
        )
    );
    assert!(is_lines_equal(&table, 17 + 2 + 2));

    let data = vec!["this is a long  sentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| &str              |"
            "|-------------------|"
            "| this is a long    |"
            "| sentence          |"
        )
    );
    assert!(is_lines_equal(&table, 17 + 2 + 2));

    let data = vec!["this is a long   sentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| &str              |"
            "|-------------------|"
            "| this is a long    |"
            "| sentence          |"
        )
    );

    let data = vec!["this is a long    sentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
        .to_string();

    // 'sentence' doesn't have a space ' sentence' because we use left alignment
    assert_eq!(
        table,
        static_table!(
            "| &str              |"
            "|-------------------|"
            "| this is a long    |"
            "|  sentence         |"
        )
    );
    assert!(is_lines_equal(&table, 17 + 2 + 2));

    let data = vec!["this"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Width::wrap(10).keep_words()))
        .to_string();

    assert_eq!(table, static_table!("| &str |" "|------|" "| this |"));
    assert!(is_lines_equal(&table, 8));
}

#[cfg(feature = "color")]
#[test]
fn max_width_wrapped_keep_words_color() {
    use owo_colors::OwoColorize;

    let data = vec!["this is a long sentence".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| String            |"
            "|-------------------|"
            "| \u{1b}[32m\u{1b}[40mthis is a long \u{1b}[39m\u{1b}[49m   |"
            "| \u{1b}[32m\u{1b}[40mse\u{1b}[39m\u{1b}[49m\u{1b}[32m\u{1b}[40mntence\u{1b}[39m\u{1b}[49m          |"
        )
    );

    let data = vec!["this is a long  sentence".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| String            |"
            "|-------------------|"
            "| \u{1b}[32m\u{1b}[40mthis is a long  \u{1b}[39m\u{1b}[49m  |"
            "| \u{1b}[32m\u{1b}[40ms\u{1b}[39m\u{1b}[49m\u{1b}[32m\u{1b}[40mentence\u{1b}[39m\u{1b}[49m          |"
        )
    );

    let data = vec!["this is a long   sentence".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| String            |"
            "|-------------------|"
            "| \u{1b}[32m\u{1b}[40mthis is a long   \u{1b}[39m\u{1b}[49m |"
            "| \u{1b}[32m\u{1b}[40msentence\u{1b}[39m\u{1b}[49m          |"
        )
    );

    let data = vec!["this is a long    sentence".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| String            |"
            "|-------------------|"
            "| \u{1b}[32m\u{1b}[40mthis is a long   \u{1b}[39m\u{1b}[49m |"
            "| \u{1b}[32m\u{1b}[40m sentence\u{1b}[39m\u{1b}[49m         |"
        )
    );

    let data = vec!["this".on_black().green().to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Width::wrap(10).keep_words()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| String |"
            "|--------|"
            "|  \u{1b}[32m\u{1b}[40mthis\u{1b}[39m\u{1b}[49m  |"
        )
    );
}

#[test]
fn max_width_wrapped_keep_words_long_word() {
    let data = vec!["this is a long sentencesentencesentence"];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| &str              |"
            "|-------------------|"
            "| this is a long    |"
            "| sentencesentences |"
            "| entence           |"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn max_width_wrapped_keep_words_long_word_color() {
    use owo_colors::OwoColorize;

    let data = vec!["this is a long sentencesentencesentence"
        .on_black()
        .green()
        .to_string()];
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Segment::all()).with(Width::wrap(17).keep_words()))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| String            |"
            "|-------------------|"
            "| \u{1b}[32m\u{1b}[40mthis is a long \u{1b}[39m\u{1b}[49m   |"
            "| \u{1b}[32m\u{1b}[40mse\u{1b}[39m\u{1b}[49m\u{1b}[32m\u{1b}[40mntencesentences\u{1b}[39m\u{1b}[49m |"
            "| \u{1b}[32m\u{1b}[40mentence\u{1b}[39m\u{1b}[49m           |"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn max_width_wrapped_collored() {
    use owo_colors::OwoColorize;

    let data = &[
        "asd".red().to_string(),
        "zxc2".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Width::wrap(2)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| St |"
            "| ri |"
            "| ng |"
            "|----|"
            "| \u{1b}[31mas\u{1b}[39m |"
            "| \u{1b}[31md\u{1b}[39m  |"
            "| \u{1b}[34mzx\u{1b}[39m |"
            "| \u{1b}[34mc2\u{1b}[39m |"
            "| \u{1b}[32m\u{1b}[40mas\u{1b}[39m\u{1b}[49m |"
            "| \u{1b}[32m\u{1b}[40mda\u{1b}[39m\u{1b}[49m |"
            "| \u{1b}[32m\u{1b}[40msd\u{1b}[39m\u{1b}[49m |"
        )
    );
}

#[test]
fn dont_change_content_if_width_is_less_then_max_width() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Width::truncate(1000).suffix("...")))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | column 0 | column 1 | column 2 |"
            "|---+----------+----------+----------|"
            "| 0 |   0-0    |   0-1    |   0-2    |"
            "| 1 |   1-0    |   1-1    |   1-2    |"
            "| 2 |   2-0    |   2-1    |   2-2    |"
        )
    );
}

#[test]
fn max_width_with_emoji() {
    let data = &["🤠", "😳🥵🥶😱😨", "🚴🏻‍♀️🚴🏻🚴🏻‍♂️🚵🏻‍♀️🚵🏻🚵🏻‍♂️"];

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Width::truncate(3).suffix("...")))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| &st... |"
            "|--------|"
            "|   🤠   |"
            "| 😳�... |"
            "| 🚴�... |"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn color_chars_are_stripped() {
    use owo_colors::OwoColorize;

    let data = &[
        "asd".red().to_string(),
        "zxc".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Width::truncate(3).suffix("...")))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| Str... |"
            "|--------|"
            "|  \u{1b}[31masd\u{1b}[0m   |"
            "|  \u{1b}[34mzxc\u{1b}[0m   |"
            "| \u{1b}[32m\u{1b}[40masd\u{1b}[39m\u{1b}[49m... |"
        )
    );
}

#[test]
fn min_width() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(12)));

    assert_eq!(
        table.to_string(),
        static_table!(
            "|      N       |   column 0   |   column 1   |   column 2   |"
            "|--------------+--------------+--------------+--------------|"
            "|      0       |     0-0      |     0-1      |     0-2      |"
            "|      1       |     1-0      |     1-1      |     1-2      |"
            "|      2       |     2-0      |     2-1      |     2-2      |"
        ),
    );

    let table = table.with(Modify::new(Segment::all()).with(TrimStrategy::None));

    assert_eq!(
        table.to_string(),
        static_table!(
            "| N            | column 0     | column 1     | column 2     |"
            "|--------------+--------------+--------------+--------------|"
            "|      0       |     0-0      |     0-1      |     0-2      |"
            "|      1       |     1-0      |     1-1      |     1-2      |"
            "|      2       |     2-0      |     2-1      |     2-2      |"
        ),
    );
}

#[test]
fn min_width_with_filler() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(12).fill_with('.')))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N........... | column 0.... | column 1.... | column 2.... |"
            "|--------------+--------------+--------------+--------------|"
            "|      0       |     0-0      |     0-1      |     0-2      |"
            "|      1       |     1-0      |     1-1      |     1-2      |"
            "|      2       |     2-0      |     2-1      |     2-2      |"
        )
    );
}

#[test]
fn min_width_one_column() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(0, 0)).with(MinWidth::new(5)));

    assert_eq!(
        table.to_string(),
        static_table!(
            "|   N   | column 0 | column 1 | column 2 |"
            "|-------+----------+----------+----------|"
            "|   0   |   0-0    |   0-1    |   0-2    |"
            "|   1   |   1-0    |   1-1    |   1-2    |"
            "|   2   |   2-0    |   2-1    |   2-2    |"
        )
    );

    let table = table.with(Modify::new(Segment::all()).with(TrimStrategy::None));

    assert_eq!(
        table.to_string(),
        static_table!(
            "| N     | column 0 | column 1 | column 2 |"
            "|-------+----------+----------+----------|"
            "|   0   |   0-0    |   0-1    |   0-2    |"
            "|   1   |   1-0    |   1-1    |   1-2    |"
            "|   2   |   2-0    |   2-1    |   2-2    |"
        )
    );
}

#[test]
fn min_width_on_smaller_content() {
    let data = create_vector::<3, 3>();

    assert_eq!(
        Table::new(&data)
            .with(Style::github_markdown())
            .with(Modify::new(Rows::single(0)).with(MinWidth::new(1)))
            .to_string(),
        Table::new(&data).with(Style::github_markdown()).to_string()
    );
}

#[test]
fn min_with_max_width() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(3)))
        .with(Modify::new(Rows::single(0)).with(Width::truncate(3)));

    assert_eq!(
        table.to_string(),
        static_table!(
            "|  N  | col | col | col |"
            "|-----+-----+-----+-----|"
            "|  0  | 0-0 | 0-1 | 0-2 |"
            "|  1  | 1-0 | 1-1 | 1-2 |"
            "|  2  | 2-0 | 2-1 | 2-2 |"
        )
    );

    let table = table.with(Modify::new(Segment::all()).with(TrimStrategy::None));

    assert_eq!(
        table.to_string(),
        static_table!(
            "| N   | col | col | col |"
            "|-----+-----+-----+-----|"
            "|  0  | 0-0 | 0-1 | 0-2 |"
            "|  1  | 1-0 | 1-1 | 1-2 |"
            "|  2  | 2-0 | 2-1 | 2-2 |"
        )
    );
}

#[test]
fn min_with_max_width_truncate_suffix() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Rows::single(0)).with(MinWidth::new(3)))
        .with(Modify::new(Rows::single(0)).with(Width::truncate(3).suffix("...")));

    assert_eq!(
        table.to_string(),
        static_table!(
            "|  N  | col... | col... | col... |"
            "|-----+--------+--------+--------|"
            "|  0  |  0-0   |  0-1   |  0-2   |"
            "|  1  |  1-0   |  1-1   |  1-2   |"
            "|  2  |  2-0   |  2-1   |  2-2   |"
        )
    );

    let table = table.with(Modify::new(Segment::all()).with(TrimStrategy::None));

    assert_eq!(
        table.to_string(),
        static_table!(
            "| N   | col... | col... | col... |"
            "|-----+--------+--------+--------|"
            "|  0  |  0-0   |  0-1   |  0-2   |"
            "|  1  |  1-0   |  1-1   |  1-2   |"
            "|  2  |  2-0   |  2-1   |  2-2   |"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn min_width_color() {
    use owo_colors::OwoColorize;

    let data = &[
        "asd".red().to_string(),
        "zxc".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(MinWidth::new(10)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|   String   |"
            "|------------|"
            "|    \u{1b}[31masd\u{1b}[0m     |"
            "|    \u{1b}[34mzxc\u{1b}[0m     |"
            "|   \u{1b}[32m\u{1b}[40masdasd\u{1b}[0m\u{1b}[0m   |"
        )
    );
}

#[cfg(feature = "color")]
#[test]
fn min_width_color_with_smaller_then_width() {
    use owo_colors::OwoColorize;

    let data = &[
        "asd".red().to_string(),
        "zxc".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    assert_eq!(
        Table::new(data)
            .with(Modify::new(Segment::all()).with(MinWidth::new(1)))
            .to_string(),
        Table::new(data).to_string()
    );
}

#[test]
fn total_width_big() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::truncate(80))
        .with(MinWidth::new(80))
        .to_string();

    assert_eq!(papergrid::string_width_multiline(&table), 80);
    assert_eq!(
        table,
        static_table!(
            "|      N       |      column 0       |      column 1      |      column 2      |"
            "|--------------+---------------------+--------------------+--------------------|"
            "|      0       |         0-0         |        0-1         |        0-2         |"
            "|      1       |         1-0         |        1-1         |        1-2         |"
            "|      2       |         2-0         |        2-1         |        2-2         |"
        )
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::truncate(80))
        .with(MinWidth::new(80))
        .with(Modify::new(Segment::all()).with(TrimStrategy::None))
        .to_string();

    assert_eq!(papergrid::string_width_multiline(&table), 80);
    assert_eq!(
        table,
        static_table!(
            "| N            | column 0            | column 1           | column 2           |"
            "|--------------+---------------------+--------------------+--------------------|"
            "| 0            | 0-0                 | 0-1                | 0-2                |"
            "| 1            | 1-0                 | 1-1                | 1-2                |"
            "| 2            | 2-0                 | 2-1                | 2-2                |"
        )
    );
}

#[test]
fn total_width_big_with_panel() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::truncate(80))
        .with(MinWidth::new(80))
        .to_string();

    assert!(is_lines_equal(&table, 80));
    assert_eq!(
        table,
        static_table!(
            "|Hello World                                                                   |"
            "|--------------+---------------------+--------------------+--------------------|"
            "| N            | column 0            | column 1           | column 2           |"
            "| 0            | 0-0                 | 0-1                | 0-2                |"
            "| 1            | 1-0                 | 1-1                | 1-2                |"
            "| 2            | 2-0                 | 2-1                | 2-2                |"
        )
    );
}

#[test]
fn total_width_big_with_panel_with_wrapping_doesnt_affect_increase() {
    let data = create_vector::<3, 3>();
    let table1 = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::wrap(80))
        .with(MinWidth::new(80))
        .to_string();

    let table2 = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::truncate(80))
        .with(MinWidth::new(80))
        .to_string();

    assert_eq!(table1, table2);
}

#[test]
fn total_width_small() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::truncate(14))
        .with(MinWidth::new(14))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  |  |  | c |"
            "|--+--+--+---|"
            "|  |  |  | 0 |"
            "|  |  |  | 1 |"
            "|  |  |  | 2 |"
        )
    );
    assert!(is_lines_equal(&table, 14));
}

#[test]
fn total_width_smaller_then_content() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::truncate(8))
        .with(MinWidth::new(8))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  |  |  |  |"
            "|--+--+--+--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
}

#[test]
fn total_width_small_with_panel() {
    let data = create_vector::<3, 3>();

    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::truncate(20))
        .with(MinWidth::new(20))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  | co | co | col |"
            "|--+----+----+-----|"
            "|  | 0- | 0- | 0-2 |"
            "|  | 1- | 1- | 1-2 |"
            "|  | 2- | 2- | 2-2 |"
        )
    );
    assert!(is_lines_equal(&table, 20));

    let table = Table::new(Vec::<usize>::new())
        .with(Panel("Hello World", 0))
        .with(Width::truncate(5))
        .with(MinWidth::new(5))
        .to_string();

    assert_eq!(
        table,
        static_table!("+---+" "|Hel|" "+---+" "| u |" "+---+")
    );
    assert!(is_lines_equal(&table, 5));

    let table = Table::new(&create_vector::<1, 2>())
        .with(Panel("Hello World", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::truncate(20))
        .with(MinWidth::new(20))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|Hello World       |"
            "|--+-------+-------|"
            "|  | colum | colum |"
            "|  | 0-0   | 0-1   |"
        )
    );
    assert!(is_lines_equal(&table, 20));

    let table = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::truncate(20))
        .with(MinWidth::new(20))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|Hello World       |"
            "|--+----+----+-----|"
            "|  | co | co | col |"
            "|  | 0- | 0- | 0-2 |"
            "|  | 1- | 1- | 1-2 |"
            "|  | 2- | 2- | 2-2 |"
        )
    );
    assert!(is_lines_equal(&table, 20));

    let table = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::truncate(6))
        .with(MinWidth::new(6))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|Hello World|"
            "|--+--+--+--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
    assert!(is_lines_equal(&table, 13));

    let table = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::truncate(14))
        .with(MinWidth::new(14))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|Hello World |"
            "|--+--+--+---|"
            "|  |  |  | c |"
            "|  |  |  | 0 |"
            "|  |  |  | 1 |"
            "|  |  |  | 2 |"
        )
    );
    assert!(is_lines_equal(&table, 14));

    let table = Table::new(&data)
        .with(Panel("Hello World 123", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::truncate(14))
        .with(MinWidth::new(14))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|Hello World |"
            "|--+--+--+---|"
            "|  |  |  | c |"
            "|  |  |  | 0 |"
            "|  |  |  | 1 |"
            "|  |  |  | 2 |"
        )
    );
    assert!(is_lines_equal(&table, 14));
}

#[test]
fn total_width_wrapping() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::wrap(20))
        .with(MinWidth::new(20))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  | co | co | col |"
            "|  | lu | lu | umn |"
            "|  | mn | mn |  2  |"
            "|  |  0 |  1 |     |"
            "|--+----+----+-----|"
            "|  | 0- | 0- | 0-2 |"
            "|  | 0  | 1  |     |"
            "|  | 1- | 1- | 1-2 |"
            "|  | 0  | 1  |     |"
            "|  | 2- | 2- | 2-2 |"
            "|  | 0  | 1  |     |"
        )
    );
    assert!(is_lines_equal(&table, 20));

    let mut data = create_vector::<3, 3>();
    data[2][2] = "some loong string".to_owned();
    let table = Table::new(&data)
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::wrap(20).keep_words())
        .with(MinWidth::new(20))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  |  | column  |  |"
            "|  |  | 1       |  |"
            "|--+--+---------+--|"
            "|  |  |   0-1   |  |"
            "|  |  |   1-1   |  |"
            "|  |  | some    |  |"
            "|  |  | loong   |  |"
            "|  |  | string  |  |"
        )
    );
    assert!(is_lines_equal(&table, 20));
}

#[test]
fn total_width_small_with_panel_using_wrapping() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::wrap(20))
        .with(MinWidth::new(20))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|   Hello World    |"
            "|--+----+----+-----|"
            "|  | co | co | col |"
            "|  | lu | lu | umn |"
            "|  | mn | mn |  2  |"
            "|  |  0 |  1 |     |"
            "|  | 0- | 0- | 0-2 |"
            "|  | 0  | 1  |     |"
            "|  | 1- | 1- | 1-2 |"
            "|  | 0  | 1  |     |"
            "|  | 2- | 2- | 2-2 |"
            "|  | 0  | 1  |     |"
        )
    );
    assert!(is_lines_equal(&table, 20));

    let table = Table::new(&data)
        .with(Panel("Hello World", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::wrap(14))
        .with(MinWidth::new(14))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|Hello World |"
            "|--+--+--+---|"
            "|  |  |  | c |"
            "|  |  |  | o |"
            "|  |  |  | l |"
            "|  |  |  | u |"
            "|  |  |  | m |"
            "|  |  |  | n |"
            "|  |  |  |   |"
            "|  |  |  | 2 |"
            "|  |  |  | 0 |"
            "|  |  |  | - |"
            "|  |  |  | 2 |"
            "|  |  |  | 1 |"
            "|  |  |  | - |"
            "|  |  |  | 2 |"
            "|  |  |  | 2 |"
            "|  |  |  | - |"
            "|  |  |  | 2 |"
        )
    );
    assert!(is_lines_equal(&table, 14));

    let table = Table::new(&data)
        .with(Panel("Hello World 123", 0))
        .with(Modify::new(Segment::all()).with(Alignment::center()))
        .with(Style::github_markdown())
        .with(Width::wrap(14))
        .with(MinWidth::new(14))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|Hello World |"
            "|123         |"
            "|--+--+--+---|"
            "|  |  |  | c |"
            "|  |  |  | o |"
            "|  |  |  | l |"
            "|  |  |  | u |"
            "|  |  |  | m |"
            "|  |  |  | n |"
            "|  |  |  |   |"
            "|  |  |  | 2 |"
            "|  |  |  | 0 |"
            "|  |  |  | - |"
            "|  |  |  | 2 |"
            "|  |  |  | 1 |"
            "|  |  |  | - |"
            "|  |  |  | 2 |"
            "|  |  |  | 2 |"
            "|  |  |  | - |"
            "|  |  |  | 2 |"
        )
    );
    assert!(is_lines_equal(&table, 14));
}

#[test]
fn max_width_with_span() {
    let mut data = create_vector::<3, 3>();
    data[0][1] = "a long string".to_string();

    let table = Table::new(&data)
        .with(Style::psql())
        .with(Modify::new(Cell(1, 1)).with(Span::column(2)))
        .with(Modify::new(Cell(2, 2)).with(Span::column(2)));

    let table = table.with(Width::truncate(40));

    assert_eq!(
        table.to_string(),
        static_table!(
            " N | column 0 | column 1 | column 2 "
            "---+----------+----------+----------"
            " 0 |    a long string    |   0-2    "
            " 1 |   1-0    |         1-1         "
            " 2 |   2-0    |   2-1    |   2-2    "
        )
    );
    assert!(is_lines_equal(&table.to_string(), 36));

    let table = table.with(Width::truncate(20));

    assert_eq!(
        table.to_string(),
        static_table!(
            "  | col | col | col "
            "--+-----+-----+-----"
            "  | a long st | 0-2 "
            "  | 1-0 |    1-1    "
            "  | 2-0 | 2-1 | 2-2 "
        )
    );
    assert!(is_lines_equal(&table.to_string(), 20));

    let table = table.with(Width::truncate(10));

    assert_eq!(
        table.to_string(),
        static_table!(
            "  |  |  |  "
            "--+--+--+--"
            "  | a l |  "
            "  |  | 1-1 "
            "  |  |  |  "
        )
    );
    assert!(is_lines_equal(&table.to_string(), 11));
}

#[test]
fn wrapping_as_total_multiline() {
    #[derive(Tabled)]
    struct D<'a>(
        #[tabled(rename = "version")] &'a str,
        #[tabled(rename = "published_date")] &'a str,
        #[tabled(rename = "is_active")] &'a str,
        #[tabled(rename = "major_feature")] &'a str,
    );

    let data = vec![
        D("0.2.1", "2021-06-23", "true", "#[header(inline)] attribute"),
        D("0.2.0", "2021-06-19", "false", "API changes"),
        D("0.1.4", "2021-06-07", "false", "display_with attribute"),
    ];

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Width::wrap(57))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| ver | published_d | is_act | major_feature            |"
            "| sio | ate         | ive    |                          |"
            "| n   |             |        |                          |"
            "|-----+-------------+--------+--------------------------|"
            "| 0.2 | 2021-06-23  | true   | #[header(inline)] attrib |"
            "| .1  |             |        | ute                      |"
            "| 0.2 | 2021-06-19  | false  | API changes              |"
            "| .0  |             |        |                          |"
            "| 0.1 | 2021-06-07  | false  | display_with attribute   |"
            "| .4  |             |        |                          |"
        )
    );
    assert!(is_lines_equal(&table, 57));

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Width::wrap(57).keep_words())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| ver | published_d | is_act | major_feature            |"
            "| sio | ate         | ive    |                          |"
            "| n   |             |        |                          |"
            "|-----+-------------+--------+--------------------------|"
            "| 0.2 | 2021-06-23  | true   | #[header(inline)]        |"
            "| .1  |             |        | attribute                |"
            "| 0.2 | 2021-06-19  | false  | API changes              |"
            "| .0  |             |        |                          |"
            "| 0.1 | 2021-06-07  | false  | display_with attribute   |"
            "| .4  |             |        |                          |"
        )
    );
    assert!(is_lines_equal(&table, 57));
}

#[cfg(feature = "color")]
#[test]
fn wrapping_as_total_multiline_color() {
    use owo_colors::{AnsiColors, OwoColorize};

    #[derive(Tabled)]
    struct D(
        #[tabled(rename = "version")] String,
        #[tabled(rename = "published_date")] String,
        #[tabled(rename = "is_active")] String,
        #[tabled(rename = "major_feature")] String,
    );

    let data = vec![
        D(
            "0.2.1".red().to_string(),
            "2021-06-23".red().on_truecolor(8, 10, 30).to_string(),
            "true".to_string(),
            "#[header(inline)] attribute"
                .blue()
                .on_color(AnsiColors::Green)
                .to_string(),
        ),
        D(
            "0.2.0".red().to_string(),
            "2021-06-19".green().on_truecolor(8, 100, 30).to_string(),
            "false".to_string(),
            "API changes".yellow().to_string(),
        ),
        D(
            "0.1.4".white().to_string(),
            "2021-06-07".red().on_truecolor(8, 10, 30).to_string(),
            "false".to_string(),
            "display_with attribute"
                .red()
                .on_color(AnsiColors::Black)
                .to_string(),
        ),
    ];

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Width::wrap(57))
        .to_string();

    assert_eq!(
        table,
        "| ver | published_d | is_act | major_feature            |\n\
         | sio | ate         | ive    |                          |\n\
         | n   |             |        |                          |\n\
         |-----+-------------+--------+--------------------------|\n\
         | \u{1b}[31m0.2\u{1b}[39m | \u{1b}[31m\u{1b}[48;2;8;10;30m2021-06-23\u{1b}[39m\u{1b}[49m  | true   | \u{1b}[34m\u{1b}[42m#[header(inline)] attrib\u{1b}[39m\u{1b}[49m |\n\
         | \u{1b}[31m.1\u{1b}[39m  |             |        | \u{1b}[34m\u{1b}[42mute\u{1b}[39m\u{1b}[49m                      |\n\
         | \u{1b}[31m0.2\u{1b}[39m | \u{1b}[32m\u{1b}[48;2;8;100;30m2021-06-19\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[33mAPI changes\u{1b}[39m              |\n\
         | \u{1b}[31m.0\u{1b}[39m  |             |        |                          |\n\
         | \u{1b}[37m0.1\u{1b}[39m | \u{1b}[31m\u{1b}[48;2;8;10;30m2021-06-07\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[31m\u{1b}[40mdisplay_with attribute\u{1b}[39m\u{1b}[49m   |\n\
         | \u{1b}[37m.4\u{1b}[39m  |             |        |                          |\n"
    );
    assert!(is_lines_equal(&table, 57));

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Width::wrap(57).keep_words())
        .to_string();

    assert_eq!(
        table,
        "| ver | published_d | is_act | major_feature            |\n\
         | sio | ate         | ive    |                          |\n\
         | n   |             |        |                          |\n\
         |-----+-------------+--------+--------------------------|\n\
         | \u{1b}[31m0.2\u{1b}[39m | \u{1b}[31m\u{1b}[48;2;8;10;30m2021-06-23\u{1b}[39m\u{1b}[49m  | true   | \u{1b}[34m\u{1b}[42m#[header(inline)] \u{1b}[39m\u{1b}[49m       |\n\
         | \u{1b}[31m.1\u{1b}[39m  |             |        | \u{1b}[34m\u{1b}[42mattrib\u{1b}[39m\u{1b}[49m\u{1b}[34m\u{1b}[42mute\u{1b}[39m\u{1b}[49m                |\n\
         | \u{1b}[31m0.2\u{1b}[39m | \u{1b}[32m\u{1b}[48;2;8;100;30m2021-06-19\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[33mAPI changes\u{1b}[39m              |\n\
         | \u{1b}[31m.0\u{1b}[39m  |             |        |                          |\n\
         | \u{1b}[37m0.1\u{1b}[39m | \u{1b}[31m\u{1b}[48;2;8;10;30m2021-06-07\u{1b}[39m\u{1b}[49m  | false  | \u{1b}[31m\u{1b}[40mdisplay_with attribute\u{1b}[39m\u{1b}[49m   |\n\
         | \u{1b}[37m.4\u{1b}[39m  |             |        |                          |\n"
    );
    assert!(is_lines_equal(&table, 57));
}

#[cfg(feature = "color")]
#[test]
fn truncating_as_total_multiline_color() {
    use owo_colors::{AnsiColors, OwoColorize};

    #[derive(Tabled)]
    struct D(
        #[tabled(rename = "version")] String,
        #[tabled(rename = "published_date")] String,
        #[tabled(rename = "is_active")] String,
        #[tabled(rename = "major_feature")] String,
    );

    let data = vec![
        D(
            "0.2.1".red().to_string(),
            "2021-06-23".red().on_truecolor(8, 10, 30).to_string(),
            "true".to_string(),
            "#[header(inline)] attribute"
                .blue()
                .on_color(AnsiColors::Green)
                .to_string(),
        ),
        D(
            "0.2.0".red().to_string(),
            "2021-06-19".green().on_truecolor(8, 100, 30).to_string(),
            "false".to_string(),
            "API changes".yellow().to_string(),
        ),
        D(
            "0.1.4".white().to_string(),
            "2021-06-07".red().on_truecolor(8, 10, 30).to_string(),
            "false".to_string(),
            "display_with attribute"
                .red()
                .on_color(AnsiColors::Black)
                .to_string(),
        ),
    ];

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Width::truncate(57))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| ver | published_d | is_act | major_feature            |"
            "|-----+-------------+--------+--------------------------|"
            "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-23\u{1b}[0m\u{1b}[0m  | true   | \u{1b}[42m\u{1b}[34m#[header(inline)] attrib\u{1b}[39m\u{1b}[49m |"
            "| \u{1b}[31m0.2\u{1b}[39m | \u{1b}[48;2;8;100;30m\u{1b}[32m2021-06-19\u{1b}[0m\u{1b}[0m  | false  | \u{1b}[33mAPI changes\u{1b}[0m              |"
            "| \u{1b}[37m0.1\u{1b}[39m | \u{1b}[48;2;8;10;30m\u{1b}[31m2021-06-07\u{1b}[0m\u{1b}[0m  | false  | \u{1b}[40m\u{1b}[31mdisplay_with attribute\u{1b}[0m\u{1b}[0m   |"
        )
    );
    assert!(is_lines_equal(&table, 57));
}

#[test]
fn min_width_works_with_right_alignment() {
    let json = r#"
    {
        "some": "random",
        "json": [
            { "1": "2" },
            { "1": "2" },
            { "1": "2" }
        ]
    }
    "#;

    let table = Table::new([json])
        .with(Style::github_markdown())
        .with(MinWidth::new(50))
        .with(
            Modify::new(Segment::all())
                .with(Alignment::right())
                .with(TrimStrategy::None),
        );

    assert_eq!(
        table.to_string(),
        static_table!(
            "| &str                                           |"
            "|------------------------------------------------|"
            "|                                                |"
            "|     {                                          |"
            "|         \"some\": \"random\",                      |"
            "|         \"json\": [                              |"
            "|             { \"1\": \"2\" },                      |"
            "|             { \"1\": \"2\" },                      |"
            "|             { \"1\": \"2\" }                       |"
            "|         ]                                      |"
            "|     }                                          |"
            "|                                                |"
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));

    let table = table.with(Modify::new(Segment::all()).with(TrimStrategy::Horizontal));

    assert_eq!(
        table.to_string(),
        static_table!(
            "|                                           &str |"
            "|------------------------------------------------|"
            "|                                                |"
            "|                          {                     |"
            "|                              \"some\": \"random\", |"
            "|                              \"json\": [         |"
            "|                                  { \"1\": \"2\" }, |"
            "|                                  { \"1\": \"2\" }, |"
            "|                                  { \"1\": \"2\" }  |"
            "|                              ]                 |"
            "|                          }                     |"
            "|                                                |"
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));

    let table = table.with(Modify::new(Segment::all()).with(TrimStrategy::Both));

    assert_eq!(
        table.to_string(),
        static_table!(
            "|                                           &str |"
            "|------------------------------------------------|"
            "|                          {                     |"
            "|                              \"some\": \"random\", |"
            "|                              \"json\": [         |"
            "|                                  { \"1\": \"2\" }, |"
            "|                                  { \"1\": \"2\" }, |"
            "|                                  { \"1\": \"2\" }  |"
            "|                              ]                 |"
            "|                          }                     |"
            "|                                                |"
            "|                                                |"
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));

    let table = Table::new([json])
        .with(Style::github_markdown())
        .with(MinWidth::new(50))
        .with(
            Modify::new(Segment::all())
                .with(Alignment::center())
                .with(TrimStrategy::None),
        );

    assert_eq!(
        table.to_string(),
        static_table!(
            "| &str                                           |"
            "|------------------------------------------------|"
            "|                                                |"
            "|     {                                          |"
            "|         \"some\": \"random\",                      |"
            "|         \"json\": [                              |"
            "|             { \"1\": \"2\" },                      |"
            "|             { \"1\": \"2\" },                      |"
            "|             { \"1\": \"2\" }                       |"
            "|         ]                                      |"
            "|     }                                          |"
            "|                                                |"
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));

    let table = table.with(Modify::new(Segment::all()).with(TrimStrategy::Horizontal));

    assert_eq!(
        table.to_string(),
        static_table!(
            "|                      &str                      |"
            "|------------------------------------------------|"
            "|                                                |"
            "|               {                                |"
            "|                   \"some\": \"random\",            |"
            "|                   \"json\": [                    |"
            "|                       { \"1\": \"2\" },            |"
            "|                       { \"1\": \"2\" },            |"
            "|                       { \"1\": \"2\" }             |"
            "|                   ]                            |"
            "|               }                                |"
            "|                                                |"
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));

    let table = table.with(Modify::new(Segment::all()).with(TrimStrategy::Both));

    assert_eq!(
        table.to_string(),
        static_table!(
            "|                      &str                      |"
            "|------------------------------------------------|"
            "|               {                                |"
            "|                   \"some\": \"random\",            |"
            "|                   \"json\": [                    |"
            "|                       { \"1\": \"2\" },            |"
            "|                       { \"1\": \"2\" },            |"
            "|                       { \"1\": \"2\" }             |"
            "|                   ]                            |"
            "|               }                                |"
            "|                                                |"
            "|                                                |"
        )
    );
    assert!(is_lines_equal(&table.to_string(), 50));
}

#[test]
fn min_width_with_span_1() {
    let data = [
        ["0", "1"],
        ["a long string which will affect min width logic", ""],
        ["2", "3"],
    ];

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(1, 0)).with(Span::column(2)))
        .with(MinWidth::new(100))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|                                   0                                    |            1            |"
            "|------------------------------------------------------------------------+-------------------------|"
            "|                                                0                                                 |"
            "|            a long string which will affect min width logic             |                         |"
            "|                                   2                                    |            3            |"
        )
    );
    assert!(is_lines_equal(&table, 100));
}

#[test]
fn min_width_with_span_2() {
    let data = [
        ["0", "1"],
        ["a long string which will affect min width logic", ""],
        ["2", "3"],
    ];

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
        .with(MinWidth::new(100))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|                        0                        |                       1                        |"
            "|-------------------------------------------------+------------------------------------------------|"
            "|                        0                        |                       1                        |"
            "|                         a long string which will affect min width logic                          |"
            "|                        2                        |                       3                        |"
        )
    );
    assert!(is_lines_equal(&table, 100));
}

#[test]
fn justify_width_constant_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Justify::new(3))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  N  | col | col | col |"
            "|-----+-----+-----+-----|"
            "|  0  | 0-0 | 0-1 | 0-2 |"
            "|  1  | 1-0 | 1-1 | 1-2 |"
            "|  2  | 2-0 | 2-1 | 2-2 |"
        )
    );
}

#[test]
fn justify_width_constant_different_sizes_test() {
    let mut data = create_vector::<3, 3>();
    data[0][1] = "Hello World".to_owned();
    data[2][2] = "multi\nline string\n".to_owned();

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Justify::new(3))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  N  | col | col | col |"
            "|-----+-----+-----+-----|"
            "|  0  | Hel | 0-1 | 0-2 |"
            "|  1  | 1-0 | 1-1 | 1-2 |"
            "|  2  | 2-0 | mul | 2-2 |"
        )
    );
}

#[test]
fn justify_width_constant_0_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Justify::new(0))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  |  |  |  |"
            "|--+--+--+--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
}

#[test]
fn justify_width_min_test() {
    let data = create_vector::<3, 3>();
    let m = Justify::min();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(m)
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | c | c | c |"
            "|---+---+---+---|"
            "| 0 | 0 | 0 | 0 |"
            "| 1 | 1 | 1 | 1 |"
            "| 2 | 2 | 2 | 2 |"
        )
    );
}

#[test]
fn justify_width_max_test() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Justify::max())
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|    N     | column 0 | column 1 | column 2 |"
            "|----------+----------+----------+----------|"
            "|    0     |   0-0    |   0-1    |   0-2    |"
            "|    1     |   1-0    |   1-1    |   1-2    |"
            "|    2     |   2-0    |   2-1    |   2-2    |"
        )
    );
}

#[test]
fn max_width_when_cell_has_tabs() {
    let mut data = create_vector::<3, 3>();
    data[1][1] = String::from("\tHello\tWorld\t");

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Columns::new(..)).with(Width::truncate(1)))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "| N | c | c | c |"
            "|---+---+---+---|"
            "| 0 | 0 | 0 | 0 |"
            "| 1 |   | 1 | 1 |"
            "| 2 | 2 | 2 | 2 |"
        )
    );
}

#[test]
fn max_width_table_when_cell_has_tabs() {
    let mut data = create_vector::<3, 3>();
    data[1][1] = String::from("\tHello\tWorld\t");

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::truncate(15))
        .to_string();

    assert_eq!(
        table,
        static_table!(
            "|  | co |  |  |"
            "|--+----+--+--|"
            "|  | 0- |  |  |"
            "|  |    |  |  |"
            "|  | 2- |  |  |"
        )
    );
}

#[test]
fn max_width_truncate_with_big_span() {
    let mut data = create_vector::<3, 3>();
    data[1][1] = String::from("Hello World With Big Line; Here we gooooooo");

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(2, 1)).with(Span::column(3)))
        .with(Width::truncate(40))
        .to_string();

    assert_eq!(papergrid::string_width_multiline(&table), 40);
    assert_eq!(
        table,
        static_table!(
            "|  | column 0  | column 1  | column 2  |"
            "|--+-----------+-----------+-----------|"
            "|  |    0-0    |    0-1    |    0-2    |"
            "|  | Hello World With Big Line; Here w |"
            "|  |    2-0    |    2-1    |    2-2    |"
        )
    );

    let mut data = create_vector::<3, 3>();
    data[1][1] = String::from("Hello World With Big Line; Here we gooooooo");
    data[2][2] = String::from("Hello World With Big Line; Here");

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(2, 1)).with(Span::column(3)))
        .with(Modify::new(Cell(3, 2)).with(Span::column(2)))
        .with(Width::truncate(40))
        .to_string();

    // bigest influence

    assert_eq!(papergrid::string_width_multiline(&table), 40);
    assert_eq!(
        table,
        static_table!(
            "|  | column 0 | column 1  |  column 2  |"
            "|--+----------+-----------+------------|"
            "|  |   0-0    |    0-1    |    0-2     |"
            "|  | Hello World With Big Line; Here w |"
            "|  |   2-0    | Hello World With Big L |"
        )
    );

    let mut data = create_vector::<3, 3>();
    data[1][1] = String::from("Hello World With Big Line; Here we gooooooo");
    data[2][2] = String::from("Hello World With Big Line; Here");

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(2, 1)).with(Span::column(2)))
        .with(Modify::new(Cell(3, 2)).with(Span::column(2)))
        .with(Width::truncate(40))
        .to_string();

    assert_eq!(papergrid::string_width_multiline(&table), 40);
    assert_eq!(
        table,
        static_table!(
            "|  |   column 0    |   column 1    | c |"
            "|--+---------------+---------------+---|"
            "|  |      0-0      |      0-1      | 0 |"
            "|  | Hello World With Big Line; He | 1 |"
            "|  |      2-0      | Hello World With  |"
        )
    );
}

#[test]
fn max_width_truncate_priority_max() {
    let mut data = create_vector::<3, 3>();
    data[1][1] = String::from("Hello World With Big Line");

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::truncate(35).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 35));
    assert_eq!(
        table,
        static_table!(
            "| N | column  | column  | column  |"
            "|---+---------+---------+---------|"
            "| 0 |   0-0   |   0-1   |   0-2   |"
            "| 1 | Hello W |   1-1   |   1-2   |"
            "| 2 |   2-0   |   2-1   |   2-2   |"
        )
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::truncate(20).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 20));
    assert_eq!(
        table,
        static_table!(
            "| N | co | co | co |"
            "|---+----+----+----|"
            "| 0 | 0- | 0- | 0- |"
            "| 1 | He | 1- | 1- |"
            "| 2 | 2- | 2- | 2- |"
        )
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::truncate(0).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 13));
    assert_eq!(
        table,
        static_table!(
            "|  |  |  |  |"
            "|--+--+--+--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
}

#[test]
fn max_width_truncate_priority_max_with_span() {
    let mut data = create_vector::<3, 3>();
    data[1][1] = String::from("Hello World With Big Line");

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(2, 1)).with(Span::column(2)))
        .with(Width::truncate(15).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 15));
    assert_eq!(
        table,
        static_table!(
            "| N | c |  |  |"
            "|---+---+--+--|"
            "| 0 | 0 |  |  |"
            "| 1 | Hell |  |"
            "| 2 | 2 |  |  |"
        )
    );
}

#[test]
fn max_width_wrap_priority_max() {
    let mut data = create_vector::<3, 3>();
    data[1][1] = String::from("Hello World With Big Line");

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::wrap(35).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 35));
    assert_eq!(
        table,
        static_table!(
            "| N | column  | column  | column  |"
            "|   | 0       | 1       | 2       |"
            "|---+---------+---------+---------|"
            "| 0 |   0-0   |   0-1   |   0-2   |"
            "| 1 | Hello W |   1-1   |   1-2   |"
            "|   | orld Wi |         |         |"
            "|   | th Big  |         |         |"
            "|   | Line    |         |         |"
            "| 2 |   2-0   |   2-1   |   2-2   |"
        )
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::wrap(20).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 20));
    assert_eq!(
        table,
        static_table!(
            "| N | co | co | co |"
            "|   | lu | lu | lu |"
            "|   | mn | mn | mn |"
            "|   |  0 |  1 |  2 |"
            "|---+----+----+----|"
            "| 0 | 0- | 0- | 0- |"
            "|   | 0  | 1  | 2  |"
            "| 1 | He | 1- | 1- |"
            "|   | ll | 1  | 2  |"
            "|   | o  |    |    |"
            "|   | Wo |    |    |"
            "|   | rl |    |    |"
            "|   | d  |    |    |"
            "|   | Wi |    |    |"
            "|   | th |    |    |"
            "|   |  B |    |    |"
            "|   | ig |    |    |"
            "|   |  L |    |    |"
            "|   | in |    |    |"
            "|   | e  |    |    |"
            "| 2 | 2- | 2- | 2- |"
            "|   | 0  | 1  | 2  |"
        )
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::wrap(0).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 13));
    assert_eq!(
        table,
        static_table!(
            "|  |  |  |  |"
            "|--+--+--+--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
}

#[test]
fn max_width_wrap_priority_max_with_span() {
    let mut data = create_vector::<3, 3>();
    data[1][1] = String::from("Hello World With Big Line");

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(2, 1)).with(Span::column(2)))
        .with(Width::wrap(15).priority::<PriorityMax>())
        .to_string();

    assert!(is_lines_equal(&table, 15));
    assert_eq!(
        table,
        static_table!(
            "| N | c |  |  |"
            "|   | o |  |  |"
            "|   | l |  |  |"
            "|   | u |  |  |"
            "|   | m |  |  |"
            "|   | n |  |  |"
            "|   |   |  |  |"
            "|   | 0 |  |  |"
            "|---+---+--+--|"
            "| 0 | 0 |  |  |"
            "|   | - |  |  |"
            "|   | 0 |  |  |"
            "| 1 | Hell |  |"
            "|   | o Wo |  |"
            "|   | rld  |  |"
            "|   | With |  |"
            "|   |  Big |  |"
            "|   |  Lin |  |"
            "|   | e    |  |"
            "| 2 | 2 |  |  |"
            "|   | - |  |  |"
            "|   | 0 |  |  |"
        )
    );
}

#[test]
fn max_width_truncate_priority_min() {
    let mut data = create_vector::<3, 3>();
    data[1][1] = String::from("Hello World With Big Line");

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::truncate(35).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 35));
    assert_eq!(
        table,
        static_table!(
            "|  |        column 0        |  |  |"
            "|--+------------------------+--+--|"
            "|  |          0-0           |  |  |"
            "|  | Hello World With Big L |  |  |"
            "|  |          2-0           |  |  |"
        )
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::truncate(20).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 20));
    assert_eq!(
        table,
        static_table!(
            "|  | column  |  |  |"
            "|--+---------+--+--|"
            "|  |   0-0   |  |  |"
            "|  | Hello W |  |  |"
            "|  |   2-0   |  |  |"
        )
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::truncate(0).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 13));
    assert_eq!(
        table,
        static_table!(
            "|  |  |  |  |"
            "|--+--+--+--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
}

#[test]
fn max_width_truncate_priority_min_with_span() {
    let mut data = create_vector::<3, 3>();
    data[1][1] = String::from("Hello World With Big Line");

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(2, 1)).with(Span::column(2)))
        .with(Width::truncate(15).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 15));
    assert_eq!(
        table,
        static_table!(
            "|  |  | co |  |"
            "|--+--+----+--|"
            "|  |  | 0- |  |"
            "|  | Hello |  |"
            "|  |  | 2- |  |"
        )
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(2, 1)).with(Span::column(2)))
        .with(Width::truncate(17).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 17));
    assert_eq!(
        table,
        static_table!(
            "|  |  | colu |  |"
            "|--+--+------+--|"
            "|  |  | 0-1  |  |"
            "|  | Hello W |  |"
            "|  |  | 2-1  |  |"
        )
    );
}

#[test]
fn max_width_wrap_priority_min() {
    let mut data = create_vector::<3, 3>();
    data[1][1] = String::from("Hello World With Big Line");

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::wrap(35).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 35));
    assert_eq!(
        table,
        static_table!(
            "|  |        column 0        |  |  |"
            "|--+------------------------+--+--|"
            "|  |          0-0           |  |  |"
            "|  | Hello World With Big L |  |  |"
            "|  | ine                    |  |  |"
            "|  |          2-0           |  |  |"
        )
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::wrap(20).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 20));
    assert_eq!(
        table,
        static_table!(
            "|  | column  |  |  |"
            "|  | 0       |  |  |"
            "|--+---------+--+--|"
            "|  |   0-0   |  |  |"
            "|  | Hello W |  |  |"
            "|  | orld Wi |  |  |"
            "|  | th Big  |  |  |"
            "|  | Line    |  |  |"
            "|  |   2-0   |  |  |"
        )
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Width::wrap(0).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 13));
    assert_eq!(
        table,
        static_table!(
            "|  |  |  |  |"
            "|--+--+--+--|"
            "|  |  |  |  |"
            "|  |  |  |  |"
            "|  |  |  |  |"
        )
    );
}

#[test]
fn max_width_wrap_priority_min_with_span() {
    let mut data = create_vector::<3, 3>();
    data[1][1] = String::from("Hello World With Big Line");

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Cell(2, 1)).with(Span::column(2)))
        .with(Width::wrap(15).priority::<PriorityMin>())
        .to_string();

    assert!(is_lines_equal(&table, 15));
    assert_eq!(
        table,
        static_table!(
            "|  |  | co |  |"
            "|  |  | lu |  |"
            "|  |  | mn |  |"
            "|  |  |  1 |  |"
            "|--+--+----+--|"
            "|  |  | 0- |  |"
            "|  |  | 1  |  |"
            "|  | Hello |  |"
            "|  |  Worl |  |"
            "|  | d Wit |  |"
            "|  | h Big |  |"
            "|  |  Line |  |"
            "|  |  | 2- |  |"
            "|  |  | 1  |  |"
        )
    );
}

#[test]
fn min_width_priority_max() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(MinWidth::new(60).priority::<PriorityMax>());

    assert_eq!(
        table.to_string(),
        static_table!(
            "| N | column 0 | column 1 |            column 2            |"
            "|---+----------+----------+--------------------------------|"
            "| 0 |   0-0    |   0-1    |              0-2               |"
            "| 1 |   1-0    |   1-1    |              1-2               |"
            "| 2 |   2-0    |   2-1    |              2-2               |"
        ),
    );
}

#[test]
fn min_width_priority_min() {
    let data = create_vector::<3, 3>();
    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(MinWidth::new(60).priority::<PriorityMin>());

    assert_eq!(
        table.to_string(),
        static_table!(
            "|      N       |   column 0   |   column 1   |  column 2   |"
            "|--------------+--------------+--------------+-------------|"
            "|      0       |     0-0      |     0-1      |     0-2     |"
            "|      1       |     1-0      |     1-1      |     1-2     |"
            "|      2       |     2-0      |     2-1      |     2-2     |"
        ),
    );
}
