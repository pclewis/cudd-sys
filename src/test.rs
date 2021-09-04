use cudd::*;
use libc::{c_int, c_void};
use std::ptr::null_mut;

static mut CALLED: bool = false;

extern "C" fn termination_handler(_data: *const c_void) -> c_int {
    unsafe {
        CALLED = true;
    }
    0
}

#[test]
pub fn basic_functionality_test() {
    unsafe {
        let cudd = Cudd_Init(0, 0, CUDD_UNIQUE_SLOTS, CUDD_CACHE_SLOTS, 0);
        Cudd_RegisterTerminationCallback(cudd, termination_handler, null_mut());

        // Check that the basic identity (a & b) <=> !(!a | !b) holds.
        let a = Cudd_bddIthVar(cudd, 1);
        let b = Cudd_bddIthVar(cudd, 2);

        // Check the complement "macros".
        let not_a = Cudd_Complement(a);
        let not_b = Cudd_Complement(b);
        Cudd_Ref(not_a);
        Cudd_Ref(not_b);

        let a_and_b = Cudd_bddAnd(cudd, a, b);
        Cudd_Ref(a_and_b);

        let not_a_or_b = Cudd_bddNor(cudd, not_a, not_b);
        Cudd_Ref(not_a_or_b);

        assert_eq!(a_and_b, not_a_or_b);

        // 20 variable pairs should be enough to trigger termination handler at some point.
        let mut big_bdd = Cudd_ReadOne(cudd);
        for i in 1..20 {
            let x = Cudd_bddIthVar(cudd, i);
            let y = Cudd_bddIthVar(cudd, 2 * i);
            let x_iff_y = Cudd_bddXor(cudd, x, y);
            Cudd_Ref(x_iff_y);
            big_bdd = Cudd_bddAnd(cudd, big_bdd, x_iff_y);
            Cudd_Ref(big_bdd);

            let reported = Cudd_DagSize(big_bdd);
            let mut counted = 0;
            Cudd_ForeachNode(cudd, big_bdd, |_| {
                counted += 1;
            });
            assert_eq!(reported, counted);
            assert!(reported > 0)
        }

        assert!(CALLED);
        assert_eq!(4091, Cudd_DagSize(big_bdd));

        Cudd_Quit(cudd);
    }
}
