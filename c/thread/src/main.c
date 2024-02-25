#include <unistd.h>
#include <pthread.h>
#include <ncurses.h>
#include <stdbool.h>

static int range(int x, int min, int max)
{
	if (x < min)
		return min;
	else if (x > max)
		return max;
	else
		return x;
}

void *thread_entry(void *args)
{
	WINDOW *wnd = initscr();
	noecho();
	nodelay(wnd, TRUE);
	keypad(wnd, TRUE);
	int x = 0, y = 0;
	bool loop = true;
	while (loop) {
		clear();
		int c = getch();
		switch (c) {
		case KEY_UP:
			y = range(y - 1, 0, 25);
			break;
		case KEY_DOWN:
			y = range(y + 1, 0, 25);
			break;
		case KEY_LEFT:
			x = range(x - 1, 0, 80);
			break;
		case KEY_RIGHT:
			x = range(x + 1, 0, 80);
			break;
		case 'q':
			loop = false;
			break;
		}
		mvprintw(y, x, "%d", c);
		refresh();
		sleep(1);
	}
	endwin();
	return NULL;
}

int main()
{
	int x = 11;
	pthread_t pthread;
	pthread_create(&pthread, NULL, &thread_entry, &x);
	pthread_join(pthread, NULL);

	return 0;
}
