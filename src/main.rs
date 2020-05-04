use ncurses::*;

mod app;
mod sort;
mod ui;
mod util;

fn main() {
    initscr();
    noecho();
    keypad(stdscr(), true);
    let mut app = app::Application::new();
    let mut user_interface = ui::UserInterface::new();
    app.load_data();
    user_interface.init_color_pairs();
    user_interface.populate_screen(&app);
    loop {
        let user_input = get_wch();
        match user_input.unwrap() {
            WchResult::Char(ch) => {
                match ch {
                    5 => { // C-e
                        app.toggle_match(&mut user_interface);
                        user_interface.populate_screen(&app);
                    },
                    9 => { // TAB
                        let command = user_interface.get_selected(&app);
                        util::echo(command);
                        break;
                    },
                    10 => { // ENTER ("\n")
                        let command = user_interface.get_selected(&app);
                        util::echo(command);
                        util::echo("\n".to_string());
                        break;
                    },
                    20 => { // C-t
                        app.toggle_case();
                        user_interface.populate_screen(&app);
                    },
                    27 => break, // ESC
                    31 => { // C-/
                        app.toggle_view(&mut user_interface);
                        user_interface.populate_screen(&app);
                    }
                    _ => {
                        app.search_string += &std::char::from_u32(ch as u32).unwrap().to_string();
                        app.search(&mut user_interface);
                    },
                }
            },
            WchResult::KeyCode(code) => {
                match code {
                    KEY_UP => {
                        user_interface.move_selected(
                            &app.all_entries
                                .as_ref()
                                .unwrap()
                                .get(&app.view)
                                .unwrap(), -1
                            );
                        user_interface.populate_screen(&app);
                    },
                    KEY_DOWN => {
                        user_interface.move_selected(
                            &app.all_entries
                            .as_ref()
                            .unwrap()
                            .get(&app.view)
                            .unwrap(), 1
                        );
                        user_interface.populate_screen(&app);
                    },
                    KEY_BACKSPACE => {
                        app.search_string.pop();
                        app.all_entries = app.to_restore.clone();
                        app.search(&mut user_interface);
                    }
                    _ => {}
                }
            }
        }
    }
    endwin();
}
