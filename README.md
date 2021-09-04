[![Crates.io](https://img.shields.io/crates/v/cudd-sys?style=flat-square)](https://crates.io/crates/cudd-sys)
[![Api Docs](https://img.shields.io/badge/docs-api-yellowgreen?style=flat-square)](https://docs.rs/cudd-sys/)
[![Continuous integration](https://img.shields.io/github/workflow/status/pclewis/cudd-sys/build?style=flat-square)](https://github.com/pclewis/cudd-sys/actions?query=workflow%3Abuild)

# Rust Bindings for the CUDD library

This crate provides unsafe Rust bindings for the University of Colorado decision diagram
package (CUDD), including the DDDMP serialisation library. It uses version `3.0.0` of CUDD
available from the unofficial [Github mirror](https://github.com/ivmai/cudd) and compiles on
Linux and MacOS (you should be also able to build CUDD on Windows using cygwin, but the project
is not set-up to do it automatically).

In the root module, you will find declarations of the C structs and types used
throughout CUDD. The main API of the CUDD package is then exported in `::cudd`. However,
CUDD also includes other "public" functionality (multiway-branching trees, extended
double precision numbers, serialisation, ...) which can be found in the remaining modules.

In some cases, there are macro and constant definitions which cannot be directly exported
to Rust. These have been re-implemented and should have their own documentation.
For the functions which are re-exported without change, please refer to the original
[CUDD doxygen](https://add-lib.scce.info/assets/doxygen-cudd-documentation/) and
[manual](https://add-lib.scce.info/assets/documents/cudd-manual.pdf). The documentation
of the DDDMP library is available
[here](https://www.cs.rice.edu/~lm30/RSynth/CUDD/dddmp/doc/dddmpExt.html).

**Completeness:** The main CUDD API should be fully reproduced here (except for one small
issue with `f128` numbers). The remaining modules may still be incomplete: if you need
a function that isn't exported yet, let us know in the issues.

**Correctness:** Unfortunately, CUDD cannot be processed using `bindgen`, so the API was
reproduced using a semi-automated method with a manual validation step (bunch of regexes
that a human makes sure didn't break anything ;)). As such, it is possible that there
are some minor problems that need to be sorted out. Please file an issue if you see any
unexpected behaviour or segfaults.
