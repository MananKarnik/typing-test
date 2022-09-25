extern crate ncurses;
use ncurses::*;

fn main() {
    // Start ncurses
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLACK);
    init_pair(2, COLOR_GREEN, COLOR_BLACK);
    init_pair(3, COLOR_RED, COLOR_BLACK);


    let mut quit = false;
    let mut text = "Hello_World";
    let mut idx = 0;
    let mut mistakes = 0;

    // Print to buffer
    attron(COLOR_PAIR(1));
    addstr(text);
    mv(2, 0);
    addstr("Press '~' to exit");
    mv(0, 0);

    while !quit {
        // Wait for keypress
        let key = getch();

        if idx < text.len() {
            let mut ch = text.chars().nth(idx).unwrap();
            if key as u8 as char == ch  || key as u8 as char == ' ' && ch == '_' {
                attron(COLOR_PAIR(2));
                addch(ch as u32);
                mv(0, (idx + 1).try_into().unwrap());
                idx += 1;
            } else {
                attron(COLOR_PAIR(3));
                addch(ch as u32);
                mv(0, (idx).try_into().unwrap());
                mistakes += 1;
            }
            if idx == text.len() {
                mvprintw(1, 0, "Completed with ");
                printw(&mistakes.to_string().to_owned());
                printw(" mistakes");
            }
        }
        if key as u8 as char == '~' {
            quit = true;
        }
        // Update screen
        refresh();
    }
    // Terminate ncurses
    endwin();
}

