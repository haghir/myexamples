/**
 * @file
 * List declarations.
 */
#ifndef __FOO_LIST_H__
#define __FOO_LIST_H__

/**
 * Node of list.
 */
struct foo_list_node {
	/**
	 * previous node index
	 */
	int prev;

	/**
	 * next node index
	 */
	int next;

	/**
	 * the value
	 */
	int data;

	/**
	 * indicates this node is the head or not
	 */
	int head;

	/**
	 * indicates this node has been deleted or not
	 */
	int deleted;
};

/**
 * List.
 */
struct foo_list {
	/**
	 * array of nodes
	 */
	struct foo_list_node *table;

	/**
	 * the number of nodes that are stored in the table
	 */
	int size;

	/**
	 * the number of nodes that can be stored in the table
	 */
	int capacity;

	/**
	 * if size >= capacity_ratio * capacity / 100, the capacity of
	 * the table will be extended
	 */
	int capacity_ratio;

	/**
	 * if the number of deleted nodes > delete_ratio * size / 100,
	 * the deleted nodes will be delete physically
	 */
	int delete_ratio;
};

/**
 * Initialize the list.
 * If something went wrong in this function,
 * it will return except 0.
 */
extern int foo_list_init(struct foo_list *list);

/**
 * Release the list.
 */
extern void foo_list_release(struct foo_list *list);

/**
 * Insert a data at specific index in the list.
 */
extern int foo_list_insert(struct foo_list *list, int index, int data);

/**
 * Remove an item from list by index.
 */
extern void foo_list_remove(struct foo_list *list, int index);

/**
 * Get a data by index in the list.
 */
extern int foo_list_get(struct foo_list *list, int index);

/**
 * Get the previous index of the node at specific index.
 */
extern int foo_list_prev(struct foo_list *list, int index);

/**
 * Get the next index of the node at specific index.
 */
extern int foo_list_next(struct foo_list *list, int index);

#endif
