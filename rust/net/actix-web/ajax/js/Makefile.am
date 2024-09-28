js_DATA = main.min.js
jsdir = $(prefix)/share/js

SRC = $(shell find . -type f -name "*.ts")

MOSTLYCLEANFILES = main.built.js

CLEANFILES = main.min.js

main.min.js: main.built.js
	terser -m -c -o main.min.js -- main.built.js

main.built.js: $(SRC)
	tsc --outFile main.built.js $^
