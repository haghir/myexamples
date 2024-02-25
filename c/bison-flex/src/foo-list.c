/**
 * @file
 * List implementation.
 */
#include <stdlib.h>

#include "foo-list.h"

/**
 * Extend and vacuum nodes in the list.
 */
static void _extend(struct foo_list *list)
{
}

/**
 * Initialize the list.
 * If something went wrong in this function,
 * it will return except 0.
 */
int foo_list_init(struct foo_list *list)
{
}

/**
 * Release the list.
 */
void foo_list_release(struct foo_list *list)
{
}

/**
 * Insert a data at specific index in the list.
 */
int foo_list_insert(struct foo_list *list, int index, int data)
{
	return -1;
}

/**
 * Remove an item from list by index.
 */
void foo_list_remove(struct foo_list *list, int index)
{
}

/**
 * Get a data by index in the list.
 */
int foo_list_get(struct foo_list *list, int index)
{
	return -1;
}

/**
 * Get the previous index of the node at specific index.
 */
int foo_list_prev(struct foo_list *list, int index)
{
	return -1;
}

/**
 * Get the next index of the node at specific index.
 */
int foo_list_next(struct foo_list *list, int index)
{
	return -1;
}
