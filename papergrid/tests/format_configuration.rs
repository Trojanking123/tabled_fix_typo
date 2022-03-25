use papergrid::{AlignmentHorizontal, AlignmentVertical, Entity, Formatting, Settings};

mod util;

#[test]
fn formatting_test() {
    let tests = [
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Top,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: false,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |A long string|          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |    string|\n\
             |             |with      |\n\
             |             | new      |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |...       |\n\
             |    string   |          |\n\
             |with         |          |\n\
             | new         |          |\n\
             |line         |          |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Top,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |A long string|          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |string    |\n\
             |             |with      |\n\
             |             |new       |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |...       |\n\
             |string       |          |\n\
             |with         |          |\n\
             |new          |          |\n\
             |line         |          |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Top,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: true,
            },
            "+-------------+----------+\n\
             |A long string|A         |\n\
             |             |string    |\n\
             |             |with      |\n\
             |             |new       |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |...       |\n\
             |string       |          |\n\
             |with         |          |\n\
             |new          |          |\n\
             |line         |          |\n\
             +-------------+----------+\n",
        ),
        //
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Top,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: false,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |A long string|          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |    string|\n\
             |             |   with   |\n\
             |             |    new   |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |   ...    |\n\
             |     string  |          |\n\
             |    with     |          |\n\
             |     new     |          |\n\
             |    line     |          |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Top,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |A long string|          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |  string  |\n\
             |             |   with   |\n\
             |             |   new    |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |   ...    |\n\
             |   string    |          |\n\
             |    with     |          |\n\
             |     new     |          |\n\
             |    line     |          |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Top,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: true,
            },
            "+-------------+----------+\n\
             |A long string|    A     |\n\
             |             |  string  |\n\
             |             |   with   |\n\
             |             |   new    |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |   ...    |\n\
             |   string    |          |\n\
             |    with     |          |\n\
             |     new     |          |\n\
             |    line     |          |\n\
             +-------------+----------+\n",
        ),
        //
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Top,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: false,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |A long string|          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |             |      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|       ...|\n\
             |       string|          |\n\
             |         with|          |\n\
             |          new|          |\n\
             |         line|          |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Top,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |A long string|          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |             |      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|       ...|\n\
             |       string|          |\n\
             |         with|          |\n\
             |          new|          |\n\
             |         line|          |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Top,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: true,
            },
            "+-------------+----------+\n\
             |A long string|         A|\n\
             |             |    string|\n\
             |             |      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|       ...|\n\
             |       string|          |\n\
             |         with|          |\n\
             |          new|          |\n\
             |         line|          |\n\
             +-------------+----------+\n",
        ),
        //
        // asd
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Center,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: false,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |    string|\n\
             |A long string|with      |\n\
             |             | new      |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |          |\n\
             |    string   |          |\n\
             |with         |...       |\n\
             | new         |          |\n\
             |line         |          |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Center,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |string    |\n\
             |A long string|with      |\n\
             |             |new       |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |          |\n\
             |string       |          |\n\
             |with         |...       |\n\
             |new          |          |\n\
             |line         |          |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Center,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: true,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |string    |\n\
             |A long string|with      |\n\
             |             |new       |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |          |\n\
             |string       |          |\n\
             |with         |...       |\n\
             |new          |          |\n\
             |line         |          |\n\
             +-------------+----------+\n",
        ),
        //
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Center,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: false,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |    string|\n\
             |A long string|   with   |\n\
             |             |    new   |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |          |\n\
             |     string  |          |\n\
             |    with     |   ...    |\n\
             |     new     |          |\n\
             |    line     |          |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Center,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |  string  |\n\
             |A long string|   with   |\n\
             |             |   new    |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |          |\n\
             |   string    |          |\n\
             |    with     |   ...    |\n\
             |     new     |          |\n\
             |    line     |          |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Center,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: true,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |  string  |\n\
             |A long string|   with   |\n\
             |             |   new    |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |          |\n\
             |   string    |          |\n\
             |    with     |   ...    |\n\
             |     new     |          |\n\
             |    line     |          |\n\
             +-------------+----------+\n",
        ),
        //
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Center,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: false,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |A long string|      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|          |\n\
             |       string|          |\n\
             |         with|       ...|\n\
             |          new|          |\n\
             |         line|          |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Center,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |A long string|      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|          |\n\
             |       string|          |\n\
             |         with|       ...|\n\
             |          new|          |\n\
             |         line|          |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Center,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: true,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |A long string|      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|          |\n\
             |       string|          |\n\
             |         with|       ...|\n\
             |          new|          |\n\
             |         line|          |\n\
             +-------------+----------+\n",
        ),
        //
        // asd
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Bottom,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: false,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |    string|\n\
             |             |with      |\n\
             |             | new      |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |A long string|          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |          |\n\
             |    string   |          |\n\
             |with         |          |\n\
             | new         |          |\n\
             |line         |...       |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Bottom,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |string    |\n\
             |             |with      |\n\
             |             |new       |\n\
             |             |line      |\n\
             |             |          |\n\
             |             |          |\n\
             |A long string|          |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |          |\n\
             |string       |          |\n\
             |with         |          |\n\
             |new          |          |\n\
             |line         |...       |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Left,
            AlignmentVertical::Bottom,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: true,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |A         |\n\
             |             |string    |\n\
             |             |with      |\n\
             |             |new       |\n\
             |A long string|line      |\n\
             +-------------+----------+\n\
             |1-0          |1-1       |\n\
             +-------------+----------+\n\
             |A one more   |          |\n\
             |string       |          |\n\
             |with         |          |\n\
             |new          |          |\n\
             |line         |...       |\n\
             +-------------+----------+\n",
        ),
        //
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Bottom,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: false,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |    string|\n\
             |             |   with   |\n\
             |             |    new   |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |A long string|          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |          |\n\
             |     string  |          |\n\
             |    with     |          |\n\
             |     new     |          |\n\
             |    line     |   ...    |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Bottom,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |  string  |\n\
             |             |   with   |\n\
             |             |   new    |\n\
             |             |   line   |\n\
             |             |          |\n\
             |             |          |\n\
             |A long string|          |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |          |\n\
             |   string    |          |\n\
             |    with     |          |\n\
             |     new     |          |\n\
             |    line     |   ...    |\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Center,
            AlignmentVertical::Bottom,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: true,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |    A     |\n\
             |             |  string  |\n\
             |             |   with   |\n\
             |             |   new    |\n\
             |A long string|   line   |\n\
             +-------------+----------+\n\
             |     1-0     |   1-1    |\n\
             +-------------+----------+\n\
             | A one more  |          |\n\
             |   string    |          |\n\
             |    with     |          |\n\
             |     new     |          |\n\
             |    line     |   ...    |\n\
             +-------------+----------+\n",
        ),
        //
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Bottom,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: false,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |             |      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |A long string|          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|          |\n\
             |       string|          |\n\
             |         with|          |\n\
             |          new|          |\n\
             |         line|       ...|\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Bottom,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: false,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |             |      with|\n\
             |             |       new|\n\
             |             |      line|\n\
             |             |          |\n\
             |             |          |\n\
             |A long string|          |\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|          |\n\
             |       string|          |\n\
             |         with|          |\n\
             |          new|          |\n\
             |         line|       ...|\n\
             +-------------+----------+\n",
        ),
        (
            AlignmentHorizontal::Right,
            AlignmentVertical::Bottom,
            Formatting {
                allow_lines_alignement: true,
                horizontal_trim: true,
                vertical_trim: true,
            },
            "+-------------+----------+\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |          |\n\
             |             |         A|\n\
             |             |    string|\n\
             |             |      with|\n\
             |             |       new|\n\
             |A long string|      line|\n\
             +-------------+----------+\n\
             |          1-0|       1-1|\n\
             +-------------+----------+\n\
             |   A one more|          |\n\
             |       string|          |\n\
             |         with|          |\n\
             |          new|          |\n\
             |         line|       ...|\n\
             +-------------+----------+\n",
        ),
    ];

    let mut grid = util::new_grid::<3, 2>();
    grid.set(&Entity::Cell(0, 0), Settings::new().text("A long string"));
    grid.set(
        &Entity::Cell(0, 1),
        Settings::new().text("\n\n\nA\n    string\nwith\n new\nline\n\n\n\n"),
    );
    grid.set(
        &Entity::Cell(2, 0),
        Settings::new().text("A one more\n    string\nwith\n new\nline"),
    );
    grid.set(&Entity::Cell(2, 1), Settings::new().text("..."));

    for (i, test) in tests.iter().enumerate() {
        let halignemnt = test.0;
        let valignemnt = test.1;
        let formatting = test.2;
        let expected = test.3;

        grid.set(
            &Entity::Global,
            Settings::new()
                .alignment(halignemnt)
                .vertical_alignment(valignemnt)
                .formatting(formatting),
        );

        assert_eq!(
            grid.to_string(),
            expected,
            "test case #{:?} failed;\nah={:?}\nav={:?}\nf={:?}",
            i,
            halignemnt,
            valignemnt,
            formatting
        );
    }
}

#[test]
fn formatting_empty_test() {
    let mut grid = util::new_grid::<0, 0>();
    grid.set(
        &Entity::Global,
        Settings::new().formatting(Formatting {
            allow_lines_alignement: true,
            horizontal_trim: true,
            vertical_trim: true,
        }),
    );

    assert_eq!(grid.to_string(), "");

    let mut grid = util::new_grid::<4, 0>();
    grid.set(
        &Entity::Global,
        Settings::new().formatting(Formatting {
            allow_lines_alignement: true,
            horizontal_trim: true,
            vertical_trim: true,
        }),
    );

    assert_eq!(grid.to_string(), "");

    let mut grid = util::new_grid::<0, 4>();
    grid.set(
        &Entity::Global,
        Settings::new().formatting(Formatting {
            allow_lines_alignement: true,
            horizontal_trim: true,
            vertical_trim: true,
        }),
    );

    assert_eq!(grid.to_string(), "");
}