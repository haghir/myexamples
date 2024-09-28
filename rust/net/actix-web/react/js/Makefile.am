js_DATA = main.min.js
jsdir = $(prefix)/share/js

SRC = $(shell find . -type f -name "*.tsx")

main.min.js: main.built.js
	terser -m -c -o main.min.js -- main.built.js

main.built.js: main.jsx $(SRC)
	tsc --jsx react --lib "es6,dom" --allowJs --outFile main.built.js main.jsx $^
