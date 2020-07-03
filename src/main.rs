extern crate ncurses;

use ncurses::*;

static WINDOW_HEIGHT: i32 = 3;
static WINDOW_WIDTH: i32 = 10;

/**
 *  1. Any live cell with two or three live neighbours survives.
 *  2. Any dead cell with three live neighbours becomes a live cell.
 *  3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
 */

fn main() {

    initscr();
    raw();

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    keypad(stdscr(), true);
    noecho();

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* Status/help info. */
    addstr("To start press s, to quit at any time press q");
    // mvprintw(LINES() - 1, 0, "Press F1 to exit"); bottom of the screen
    refresh();

    let mut start_y = (max_y - WINDOW_HEIGHT) / 2;
    let mut start_x = (max_x - WINDOW_WIDTH) / 2;
    let mut win = create_win(start_y, start_x);

    let mut ch = getch();
    while ch != KEY_F(1) {
        match ch {
            KEY_LEFT => {
                start_x -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            }
            KEY_RIGHT => {
                start_x += 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            KEY_UP => {
                start_y -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            KEY_DOWN => {
                start_y += 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            _ => { }
        }

        ch = getch();
    }

    endwin();
}

fn create_win(start_y: i32, start_x: i32) -> WINDOW {
    mvprintw(start_y, start_x, "*");

    let win = newwin(WINDOW_HEIGHT, WINDOW_WIDTH, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win
}

fn destroy_win(win: WINDOW) {
    let ch = ' ' as chtype;

    // clear the previous screen
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    
    wrefresh(win);
    delwin(win);
}