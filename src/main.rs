extern crate ncurses;

use ncurses::*;

// static WINDOW_HEIGHT: i32 = 3;
// static WINDOW_WIDTH: i32 = 10;

/**
 *  1. Any live cell with two or three live neighbours survives.
 *  2. Any dead cell with three live neighbours becomes a live cell.
 *  3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
 */

fn main() {

    /* Create the screen */
    initscr();
    raw();
    /* invisible cursor */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* initialize more keys */
    keypad(stdscr(), true);
    noecho();

    /* Get the screen bounds */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    /* Print a char at the max */
    mv(max_y - 1,max_x - 1);
    addstr("~");
    refresh();


    /* Wait for input before killing the screen */
    getch();

    endwin();
    let mut vec = vec![vec!['#'; max_x as usize]; max_y as usize];
    for y in 1 ..max_y {
        for x in 1 ..max_x {
            print!("{}", vec[y as usize][x as usize]);
        }
        println!("");
    }

}