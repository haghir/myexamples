#!/bin/bash

while getopts abc OPT; do
	case ${OPT} in
		a) echo a;;
		b) echo b;;
		c) echo c;;
		*) exit 1;;
	esac
done
