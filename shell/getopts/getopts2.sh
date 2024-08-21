#!/bin/bash

while getopts a:bc: OOPPTT; do
	case ${OOPPTT} in
		a) echo "a = ${OPTARG}";;
		b) echo b;;
		c) echo "c = ${OPTARG}";;
		*) exit 1;;
	esac
done
