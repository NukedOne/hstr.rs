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
    user_interface.init_color_pairs();
    user_interface.populate_screen(&app);
    loop {
        let user_input = get_wch();
        match user_input.unwrap() {
            WchResult::Char(ch) => {
                match ch {
                    5 => { // C-e
                        app.toggle_match();
                        user_interface.selected = 0;
                        user_interface.populate_screen(&app);
                    },
                    6 => { // C-f
                        let entries = app.get_entries(app.view);
                        let command = user_interface.get_selected(&entries);
                        app.add_to_or_remove_from_favorites(command);
                    },
                    9 => { // TAB
                        let entries = app.get_entries(app.view);
                        let command = user_interface.get_selected(&entries);
                        util::echo(command);
                        break;
                    },
                    10 => { // ENTER ("\n")
                    let entries = app.get_entries(app.view);
                    let command = user_interface.get_selected(&entries);
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
                        app.toggle_view();
                        user_interface.selected = 0;
                        user_interface.populate_screen(&app);
                    }
                    _ => {
                        app.search_string += &std::char::from_u32(ch as u32).unwrap().to_string();
                        user_interface.selected = 0;
                        user_interface.page = 1;               
                        app.search();
                        user_interface.populate_screen(&app);
                    },
                }
            },
            WchResult::KeyCode(code) => {
                match code {
                    KEY_UP => {
                        let entries = app.get_entries(app.view);
                        user_interface.move_selected(entries, -1);
                        user_interface.populate_screen(&app);
                    },
                    KEY_DOWN => {
                        let entries = app.get_entries(app.view);
                        user_interface.move_selected(entries, 1);
                        user_interface.populate_screen(&app);
                    },
                    KEY_BACKSPACE => {
                        app.search_string.pop();
                        app.all_entries = app.to_restore.clone();
                        app.search();
                        user_interface.populate_screen(&app);
                    },
                    KEY_DC => {
                        let entries = app.get_entries(app.view);
                        let command = user_interface.get_selected(&entries);
                        user_interface.prompt_for_deletion(&command);
                        app.delete_from_history(command);
                        user_interface.populate_screen(&app);
                    }
                    _ => {}
                }
            }
        }
    }
    endwin();
}
