js_DATA = main.js
jsdir = $(prefix)/share/js

SRC_FILES = main.ts
CLEANFILES = main.js

main.js: $(SRC_FILES)
	tsc --target es2017 --lib "es2017,dom" --strict --outFile main.js $^
