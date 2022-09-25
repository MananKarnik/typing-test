extern crate ncurses;
use ncurses::*;

fn main() {
    // Start ncurses
    initscr();
    
    // Print to buffer
    addstr("Hello, world!");

    // Update screen
    refresh();

    // Wait for keypress
    getch();

    // Terminate ncurses
    endwin();
}
