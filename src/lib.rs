#![allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};

mod puzzle;

use puzzle::*;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        board: Board::new(1, 1).unwrap(),
        solve_button_enabled: false,
        solve_button_text: "Solve!".to_string(),
        date_month: 0,
        date_day: 0,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    board: Board,
    solve_button_enabled: bool,
    solve_button_text: String,
    date_month: i32,
    date_day: i32,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
// #[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    // in month, day format
    SetMonth(String),
    SetDay(String),
    Solve,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetMonth(month_string) => {
            if month_string.len() == 0 {
                return;
            }

            let month = month_string.parse::<i32>().unwrap();
            model.date_month = month;

            // check if date is valid
            if date_ok(&month, &model.date_day) {
                // if it's valid update the model and create a new board
                if let Ok(new_board) = Board::new(model.date_month, model.date_day) {
                    model.board = new_board;
                    model.solve_button_enabled = true;
                    model.solve_button_text = "Solve!".to_string();
                }
            } else {
                model.solve_button_enabled = false;
                model.solve_button_text = "Date Invalid!".to_string();
            }
        }
        Msg::SetDay(day_string) => {
            if day_string.len() == 0 {
                return;
            }
            let day = day_string.parse::<i32>().unwrap();
            model.date_day = day;

            if date_ok(&model.date_month, &day) {
                if let Ok(new_board) = Board::new(model.date_month, model.date_day) {
                    model.board = new_board;
                    model.solve_button_enabled = true;
                    model.solve_button_text = "Solve!".to_string();
                }
            } else {
                model.solve_button_enabled = false;
                model.solve_button_text = "Date Invalid!".to_string();
            }
        }
        Msg::Solve => {
            model.solve_button_text = "Solving!".to_string();
            model.solve_button_enabled = false;
            model.board.solve();
            model.solve_button_text = "Solve!".to_string();
        }
    }
}

fn date_ok(month: &i32, day: &i32) -> bool {
    (1..=12).contains(month) && (1..=31).contains(day)
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        div![cells(model)],
        br!(),
        div![
            span!("Month: "),
        input![
            input_ev(Ev::Input, Msg::SetMonth),
            attrs!(
                At::Type => "number"
            )
        ],
            C!["date-input"]
        ]
        div![
            span!(" Day: "),
        input![
            input_ev(Ev::Input, Msg::SetDay),
            attrs!(
                At::Type => "number",
            )
        ],
            C!["date-input"]
        ]
        button![
            format!("{}", model.solve_button_text),
            ev(Ev::Click, move |_| Msg::Solve),
            // attrs!(
            //     IF!(!model.solve_button_enabled => At::Disabled => true)
            // )
            IF!(!model.solve_button_enabled => attrs!(At::Disabled => true)),
            C!["date-input"]
        ],
        br!()
    ]
}

enum BoxType {
    Empty,
    Filled(String),
}

fn small_box(content: String, color: &str) -> Node<Msg> {
    td!(content, style!(St::Background => color), C!["cell"])
}

fn empty_box() -> Node<Msg> {
    td!(style!(St::Background => "#878f99"), C!["cell"])
}

fn cells(model: &Model) -> Node<Msg> {
    table![
        tr![
            small_box("Jan".to_string(), model.board.with_color[0]),
            small_box("Feb".to_string(), model.board.with_color[1]),
            small_box("Mar".to_string(), model.board.with_color[2]),
            small_box("Apr".to_string(), model.board.with_color[3]),
            small_box("May".to_string(), model.board.with_color[4]),
            small_box("Jun".to_string(), model.board.with_color[5]),
            empty_box()
        ],
        tr![
            small_box("Jul".to_string(), model.board.with_color[7]),
            small_box("Aug".to_string(), model.board.with_color[8]),
            small_box("Sep".to_string(), model.board.with_color[9]),
            small_box("Oct".to_string(), model.board.with_color[10]),
            small_box("Nov".to_string(), model.board.with_color[11]),
            small_box("Dec".to_string(), model.board.with_color[12]),
            empty_box()
        ],
        tr![
            small_box("1".to_string(), model.board.with_color[14]),
            small_box("2".to_string(), model.board.with_color[15]),
            small_box("3".to_string(), model.board.with_color[16]),
            small_box("4".to_string(), model.board.with_color[17]),
            small_box("5".to_string(), model.board.with_color[18]),
            small_box("6".to_string(), model.board.with_color[19]),
            small_box("7".to_string(), model.board.with_color[20]),
        ],
        tr![
            small_box("8".to_string(), model.board.with_color[21]),
            small_box("9".to_string(), model.board.with_color[22]),
            small_box("10".to_string(), model.board.with_color[23]),
            small_box("11".to_string(), model.board.with_color[24]),
            small_box("12".to_string(), model.board.with_color[25]),
            small_box("13".to_string(), model.board.with_color[26]),
            small_box("14".to_string(), model.board.with_color[27]),
        ],
        tr![
            small_box("15".to_string(), model.board.with_color[28]),
            small_box("16".to_string(), model.board.with_color[29]),
            small_box("17".to_string(), model.board.with_color[30]),
            small_box("18".to_string(), model.board.with_color[31]),
            small_box("19".to_string(), model.board.with_color[32]),
            small_box("20".to_string(), model.board.with_color[33]),
            small_box("21".to_string(), model.board.with_color[34]),
        ],
        tr![
            small_box("22".to_string(), model.board.with_color[35]),
            small_box("23".to_string(), model.board.with_color[36]),
            small_box("24".to_string(), model.board.with_color[37]),
            small_box("25".to_string(), model.board.with_color[38]),
            small_box("26".to_string(), model.board.with_color[39]),
            small_box("27".to_string(), model.board.with_color[40]),
            small_box("28".to_string(), model.board.with_color[41]),
        ],
        tr![
            small_box("29".to_string(), model.board.with_color[42]),
            small_box("30".to_string(), model.board.with_color[43]),
            small_box("31".to_string(), model.board.with_color[44]),
            empty_box(),
            empty_box(),
            empty_box(),
            empty_box(),
        ],
        C!["puzzle"]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
