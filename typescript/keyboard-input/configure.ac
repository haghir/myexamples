#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.71])
AC_INIT([myexamples-ts/keyboard-input], [1.0.0], [])
AM_INIT_AUTOMAKE([foreign])

# Checks for programs.
AC_CHECK_PROG([NPX_FOUND], [npx], [yes])
AS_IF([test x"$NPX_FOUND" != x"yes"], AC_MSG_ERROR([The required program 'npx' is not found.]))
AC_CHECK_PROG([TSC_FOUND], [tsc], [yes])
AS_IF([test x"$TSC_FOUND" != x"yes"], AC_MSG_ERROR([The required program 'tsc' is not found. (npm install -g typescript)]))

# Checks for libraries.

# Checks for header files.

# Checks for typedefs, structures, and compiler characteristics.

# Checks for library functions.
AC_CONFIG_FILES([Makefile])
AC_OUTPUT
