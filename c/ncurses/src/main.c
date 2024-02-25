#include <ncurses.h>

static int range(int x, int min, int max)
{
	if (x < min)
		x = min;
	if (x > max)
		x = max;
	return x;
}

int main()
{
	initscr();
	noecho();
	curs_set(0);
	keypad(stdscr, TRUE);
	mousemask(ALL_MOUSE_EVENTS, NULL);
	MEVENT e;
	int px = 0, py = 0;
	while (1) {
		int ch = getch();
		if (ch == 'q')
			break;
		if (ch == KEY_MOUSE) {
			if (getmouse(&e) == OK) {
				clear();
				int px = range(e.x, 0, 80);
				int py = range(e.y, 0, 25);
				mvprintw(py, px, "X");
				refresh();
			}
		}
	}
	endwin();
	return 0;
}
