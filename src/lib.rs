//! This crate provides unsafe Rust bindings for the University of Colorado decision diagram
//! package (CUDD), including the DDDMP serialisation library. It uses version `3.0.0` of CUDD
//! available from the unofficial [Github mirror](https://github.com/ivmai/cudd) and compiles on
//! Linux and MacOS (you should be also able to build CUDD on Windows using cygwin, but the project
//! is not set-up to do it automatically).
//!
//! In the root module, you will find declarations of the C structs and types used
//! throughout CUDD. The main API of the CUDD package is then exported in `::cudd`. However,
//! CUDD also includes other "public" functionality (multiway-branching trees, extended
//! double precision numbers, serialisation, ...) which can be found in the remaining modules.
//!
//! In some cases, there are macro and constant definitions which cannot be directly exported
//! to Rust. These have been re-implemented and should have their own documentation.
//! For the functions which are re-exported without change, please refer to the original
//! [CUDD doxygen](https://add-lib.scce.info/assets/doxygen-cudd-documentation/) and
//! [manual](https://add-lib.scce.info/assets/documents/cudd-manual.pdf). The documentation
//! of the DDDMP library is available
//! [here](https://www.cs.rice.edu/~lm30/RSynth/CUDD/dddmp/doc/dddmpExt.html).
//!
//! **Completeness:** The main CUDD API should be fully reproduced here (except for one small
//! issue with `f128` numbers). The remaining modules may still be incomplete: if you need
//! a function that isn't exported yet, let us know in the issues.
//!
//! **Correctness:** Unfortunately, CUDD cannot be processed using `bindgen`, so the API was
//! reproduced using a semi-automated method with a manual validation step (bunch of regexes
//! that a human makes sure didn't break anything ;)). As such, it is possible that there
//! are some minor problems that need to be sorted out. Please file an issue if you see any
//! unexpected behaviour or segfaults.
//!

// Allow non-idiomatic names in the whole crate.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

#[cfg(test)]
mod test;

/// Contains the declarations present in `cudd.h` (main CUDD API).
pub mod cudd;

/// Contains the declarations present in `mtr.h` (multiway-branching trees).
pub mod mtr;

/// Contains the declarations present in `epd.h` (extended double precision numbers).
pub mod epd;

/// Declarations from `dddmp.h` (serialisation of decision diagrams).
///
/// Currently, the error checking macros are not implemented.
pub mod dddmp;

use std::marker::{PhantomData, PhantomPinned};

/// An opaque C struct used to represent the decision diagram node.
#[repr(C)]
pub struct DdNode {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// An opaque C struct used to represent the CUDD manager.
#[repr(C)]
pub struct DdManager {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// An opaque C struct used to represent the CUDD generator.
#[repr(C)]
pub struct DdGen {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// The type of an arbitrary precision "digit".
pub type DdApaDigit = u32;

/// The type of an arbitrary precision integer, corresponding to an array of digits.
pub type DdApaNumber = *mut DdApaDigit;

/// A const-qualified version of `DdApaNumber`.
pub type DdConstApaNumber = *const DdApaDigit;

/// An opaque C struct used to represent the result of the computation of two-literal clauses.
///
/// See `Cudd_FindTwoLiteralClauses`.
#[repr(C)]
pub struct DdTlcInfo {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// An opaque C struct representing an extended double precision floating point number.
#[repr(C)]
pub struct EpDouble {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// An opaque C struct representing a multi-way branch tree node.
#[repr(C)]
pub struct MtrNode {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}
