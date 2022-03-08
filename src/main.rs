use gtk::{prelude::*, Label};
use gtk::{Align, Application, ApplicationWindow, Button, Grid};
use gtk4 as gtk;
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let app = Application::builder()
        .application_id("org.beribus.tictactoe")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &gtk::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(360)
        .default_height(360)
        .title("GTK Tac Toe")
        .build();

    let grid = Grid::builder()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    window.set_child(Some(&grid));

    let game_grid: Rc<[[Button; 3]; 3]> = Rc::new([
        [new_game_button(), new_game_button(), new_game_button()],
        [new_game_button(), new_game_button(), new_game_button()],
        [new_game_button(), new_game_button(), new_game_button()],
    ]);
    let xturn = Rc::new(Cell::new(true));
    let game_done = Rc::new(Cell::new(false));
    let text = Rc::new(Label::builder().label("It's X's turn!").build());

    for i in 0..3 {
        for o in 0..3 {
            let xturn = xturn.clone();
            let current_button = &game_grid[i][o];

            let game_grid = game_grid.clone();
            let game_done = game_done.clone();
            let text = text.clone();

            grid.attach(
                current_button,
                i.try_into().unwrap(),
                o.try_into().unwrap(),
                1,
                1,
            );

            current_button.connect_clicked(move |current_button| {
                let mut tie: bool = false;
                if current_button.label().unwrap() == "" {
                    if xturn.get() {
                        current_button.set_label("X");
                        text.set_label("It's O's turn!");
                    } else {
                        current_button.set_label("O");
                        text.set_label("It's X's turn!");
                    };
                    for i in 0..3 {
                        if game_grid[i][0].label().unwrap() != ""
                            && game_grid[i][0].label() == game_grid[i][1].label()
                            && game_grid[i][1].label() == game_grid[i][2].label()
                        {
                            println!("game done! {}", if xturn.get() { "X" } else { "O" });
                            game_done.set(true);
                        }
                        if game_grid[0][i].label().unwrap() != ""
                            && game_grid[0][i].label() == game_grid[1][i].label()
                            && game_grid[1][i].label() == game_grid[2][i].label()
                        {
                            println!("game done! {}", if xturn.get() { "X" } else { "O" });
                            game_done.set(true);
                        }
                    }
                    if game_grid[1][1].label().unwrap() != ""
                        && game_grid[0][0].label() == game_grid[1][1].label()
                        && game_grid[1][1].label() == game_grid[2][2].label()
                    {
                        println!("game done! {}", if xturn.get() { "X" } else { "O" });
                        game_done.set(true);
                    }
                    if game_grid[1][1].label().unwrap() != ""
                        && game_grid[0][2].label() == game_grid[1][1].label()
                        && game_grid[1][1].label() == game_grid[2][0].label()
                    {
                        println!("game done! {}", if xturn.get() { "X" } else { "O" });
                        game_done.set(true);
                    }
                    xturn.set(!xturn.get());
                } else {
                    println!("This spot is taken! Go Again.");
                }
                'outerer: loop {
                    for i in 0..3 {
                        for o in 0..3 {
                            if game_grid[i][o].label().unwrap() == "" {
                                break 'outerer;
                            }
                        }
                    }
                    tie = true;
                    game_done.set(true);
                    break 'outerer;
                }
                if game_done.get() {
                    for i in 0..3 {
                        for o in 0..3 {
                            game_grid[i][o].set_sensitive(false);
                        }
                    }
                    if tie {
                        text.set_label("It's a tie!")
                    } else {
                        text.set_label(
                            &(format!("Game over! {} won!", if xturn.get() { "O" } else { "X" })),
                        );
                    }
                }
            });
        }
    }

    grid.attach(text.as_ref(), 0, 4, 3, 1); // move these into button closurue to ensure that it runs
                                            //and text changes, idk how it changed before but know it's not

    let reset_button = Button::builder()
        .halign(Align::Center)
        .valign(Align::Center)
        .label("Reset Game")
        .build();

    reset_button.connect_clicked(move |_| {
        for i in 0..3 {
            for o in 0..3 {
                let current_button = &game_grid[i][o];
                current_button.set_label("");
                current_button.set_sensitive(true);
                game_done.set(false);
                text.set_label("It's X's turn!")
            }
        }
    });
    grid.attach(&reset_button, 0, 3, 3, 1);

    window.show();
}

fn new_game_button() -> Button {
    let button = Button::builder()
        .halign(Align::Center)
        .valign(Align::Center)
        .label("")
        .width_request(90)
        .height_request(90)
        .build();
    button
}
