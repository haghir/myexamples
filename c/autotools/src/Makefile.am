SUBDIRS = bar

lib_LTLIBRARIES = libfoo.la
libfoo_la_SOURCES = foo.c
libfoo_la_LIBADD = bar/libbar.la

include_HEADERS = foo.h
includedir = $(prefix)/include/my-examples

bin_PROGRAMS = foo
foo_SOURCES = main.c
foo_LDADD = libfoo.la
