use cudd::*;

#[test]
pub fn basic_functionality_test() {
    unsafe {
        let cudd = Cudd_Init(0, 0, CUDD_UNIQUE_SLOTS, CUDD_CACHE_SLOTS, 0);
        // Check that the basic identity (a & b) <=> !(!a | !b) holds.
        let a = Cudd_bddIthVar(cudd, 1);
        let b = Cudd_bddIthVar(cudd, 2);
        // There is a Cudd_Not macro, but that is not available through bindings.
        let not_a = Cudd_bddNand(cudd, a, a);
        let not_b = Cudd_bddNand(cudd, b, b);
        Cudd_Ref(not_a);
        Cudd_Ref(not_b);

        let a_and_b = Cudd_bddAnd(cudd, a, b);
        Cudd_Ref(a_and_b);

        let not_a_or_b = Cudd_bddNor(cudd, not_a, not_b);
        Cudd_Ref(not_a_or_b);

        assert_eq!(a_and_b, not_a_or_b);
        Cudd_Quit(cudd);
    }
}
