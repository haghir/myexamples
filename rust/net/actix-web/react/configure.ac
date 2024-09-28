#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.69])
AC_INIT([myexamples-rust/react], [1.0.0], [])
AM_INIT_AUTOMAKE([foreign])

# Checks for programs.
AC_CHECK_PROG([CARGO_FOUND], [cargo], [yes])
AS_IF([test x"$CARGO_FOUND" != x"yes"], AC_MSG_ERROR([The required program 'cargo' is not found.]))
AC_CHECK_PROG([NPX_FOUND], [npx], [yes])
AS_IF([test x"$NPX_FOUND" != x"yes"], AC_MSG_ERROR([The required program 'npx' is not found.]))
AC_CHECK_PROG([TSC_FOUND], [tsc], [yes])
AS_IF([test x"$TSC_FOUND" != x"yes"], AC_MSG_ERROR([The required program 'tsc' is not found. (npm install -g typescript)]))
AC_CHECK_PROG([TERSER_FOUND], [terser], [yes])
AS_IF([test x"$TERSER_FOUND" != x"yes"], AC_MSG_ERROR([The required program 'terser' is not found. (npm install -g terser)]))

# Checks for libraries.

# Checks for header files.

# Checks for typedefs, structures, and compiler characteristics.

# Checks for library functions.

AC_CONFIG_FILES([Makefile
                 js/Makefile])
AC_OUTPUT
