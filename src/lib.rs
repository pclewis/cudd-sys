// Allow non-idiomatic names in the whole crate.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;

#[cfg(test)]
mod test;

use libc::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use std::marker::{PhantomData, PhantomPinned};
use std::ops::Not;

/// An integer representation of a Boolean `true` constant. (This is not a DD construct!)
pub const CUDD_TRUE: c_uint = 1;
/// An integer representation of a Boolean `false` constant. (This is not a DD construct!)
pub const CUDD_FALSE: c_uint = 0;

/// An special return value indicating an out-of-memory error.
pub const CUDD_OUT_OF_MEM: c_int = -1;

/// Recommended default size of the unique node table.
pub const CUDD_UNIQUE_SLOTS: c_uint = (1 << 8);
/// Recommended default size of the operation cache table.
pub const CUDD_CACHE_SLOTS: c_uint = (1 << 18);

/// Default option for `Cudd_addResidue`: specifies that the least-significant-bit is on top,
/// and the number is interpreted as unsigned.
pub const CUDD_RESIDUE_DEFAULT: c_int = 0;

/// Used with `Cudd_addResidue`: add (logical or) this flag to the default options to specify that
/// the most-significant-bit is on top.
pub const CUDD_RESIDUE_MSB: c_int = 1;

/// Used with `Cudd_addResidue`: add (logical or) this flag to the default options to specify that
/// the number should be interpreted as a signed two's complement.
pub const CUDD_RESIDUE_TC: c_int = 2;

/// Types of variable reordering algorithms.
#[repr("C")]
pub enum Cudd_ReorderingType {
    CUDD_REORDER_SAME,
    CUDD_REORDER_NONE,
    CUDD_REORDER_RANDOM,
    CUDD_REORDER_RANDOM_PIVOT,
    CUDD_REORDER_SIFT,
    CUDD_REORDER_SIFT_CONVERGE,
    CUDD_REORDER_SYMM_SIFT,
    CUDD_REORDER_SYMM_SIFT_CONV,
    CUDD_REORDER_WINDOW2,
    CUDD_REORDER_WINDOW3,
    CUDD_REORDER_WINDOW4,
    CUDD_REORDER_WINDOW2_CONV,
    CUDD_REORDER_WINDOW3_CONV,
    CUDD_REORDER_WINDOW4_CONV,
    CUDD_REORDER_GROUP_SIFT,
    CUDD_REORDER_GROUP_SIFT_CONV,
    CUDD_REORDER_ANNEALING,
    CUDD_REORDER_GENETIC,
    CUDD_REORDER_LINEAR,
    CUDD_REORDER_LINEAR_CONVERGE,
    CUDD_REORDER_LAZY_SIFT,
    CUDD_REORDER_EXACT,
}

/// Type of aggregation method algorithm.
#[repr("C")]
pub enum Cudd_AggregationType {
    CUDD_NO_CHECK,
    CUDD_GROUP_CHECK,
    CUDD_GROUP_CHECK2,
    CUDD_GROUP_CHECK3,
    CUDD_GROUP_CHECK4,
    CUDD_GROUP_CHECK5,
    CUDD_GROUP_CHECK6,
    CUDD_GROUP_CHECK7,
    CUDD_GROUP_CHECK8,
    CUDD_GROUP_CHECK9,
}

/// Type of a hook.
#[repr("C")]
pub enum Cudd_HookType {
    CUDD_PRE_GC_HOOK,
    CUDD_POST_GC_HOOK,
    CUDD_PRE_REORDERING_HOOK,
    CUDD_POST_REORDERING_HOOK,
}

// Type of an error code.
#[repr("C")]
pub enum Cudd_ErrorType {
    CUDD_NO_ERROR,
    CUDD_MEMORY_OUT,
    CUDD_TOO_MANY_NODES,
    CUDD_MAX_MEM_EXCEEDED,
    CUDD_TIMEOUT_EXPIRED,
    CUDD_TERMINATION,
    CUDD_INVALID_ARG,
    CUDD_INTERNAL_ERROR,
}

/// Type of grouping used during lazy sifting.
#[repr("C")]
pub enum Cudd_LazyGroupType {
    CUDD_LAZY_NONE,
    CUDD_LAZY_SOFT_GROUP,
    CUDD_LAZY_HARD_GROUP,
    CUDD_LAZY_UNGROUP,
}

/// Type of variable used during lazy sifting.
#[repr("C")]
pub enum Cudd_VariableType {
    CUDD_VAR_PRIMARY_INPUT,
    CUDD_VAR_PRESENT_STATE,
    CUDD_VAR_NEXT_STATE,
}

/// Type of the value of a terminal node.
pub type CUDD_VALUE_TYPE = c_double;

/// An opaque C struct used to represent the decision diagram nodes.
#[repr("C")]
pub struct DdNode {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// An opaque C struct used to represent the CUDD manager.
#[repr("C")]
pub struct DdManager {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// An opaque C struct used to represent the CUDD generator.
#[repr("C")]
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

/// An opaque C struct used to represent the result of computation of two-literal clauses.
///
/// See `Cudd_FindTwoLiteralClauses`.
#[repr("C")]
pub struct DdTlcInfo {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// Type of the hook function.
pub type DD_HOOK_FUNCTION = extern "C" fn(*mut DdManager, *const c_char, *mut c_void) -> c_int;

/// Type of the priority function.
pub type DD_PRIORITY_FUNCTION = extern "C" fn(
    *mut DdManager,
    c_int,
    *mut *mut DdNode,
    *mut *mut DdNode,
    *mut *mut DdNode,
) -> *mut DdNode;

/// Type of the apply operator function.
pub type DD_APPLY_OPERATOR =
    extern "C" fn(*mut DdManager, *mut *mut DdNode, *mut *mut DdNode) -> *mut DdNode;

/// Type of the monadic apply operator function.
pub type DD_MONADIC_APPLY_OPERATOR = extern "C" fn(*mut DdManager, *mut DdNode) -> *mut DdNode;

/// Type of the two-operand cache tag function.
pub type DD_CACHE_TAG_FUNCTION_2 =
    extern "C" fn(*mut DdManager, *mut DdNode, *mut DdNode) -> *mut DdNode;

/// Type of the one-operand cache tag function.
pub type DD_CACHE_TAG_FUNCTION_1 = extern "C" fn(*mut DdManager, *mut DdNode) -> *mut DdNode;

/// Type of the out-of-memory function.
pub type DD_OUT_OF_MEMORY_FUNCTION = extern "C" fn(size_t) -> c_void;

/// Type of the Q-sort comparison function.
pub type DD_Q_SORT_FUNCTION = extern "C" fn(*const c_void, *const c_void) -> c_int;

/// Type of the termination handler function.
pub type DD_TERMINATION_HANDLER = extern "C" fn(*const c_void) -> c_int;

/// Type of the time-out handler function.
pub type DD_TIME_OUT_HANDLER = extern "C" fn(*mut DdManager, *mut c_void) -> c_void;

/// Complements a DD node by flipping the complement attribute of
/// the pointer (the least significant bit).
#[inline]
pub const unsafe fn Cudd_Not(node: *mut DdNode) -> *mut DdNode {
    ((node as usize) ^ 01) as *mut DdNode
}

/// Complements a DD node by flipping the complement attribute
/// of the pointer if a condition is satisfied. The `condition`
/// argument must be always either `1` or `0`.
#[inline]
pub const unsafe fn Cudd_NotCond(node: *mut DdNode, condition: c_int) -> *mut DdNode {
    ((node as usize) ^ (condition as usize)) as *mut DdNode
}

/// Computes the regular version of a node pointer (i.e. without the complement
/// bit set, regardless of its previous value).
pub const unsafe fn Cudd_Regular(node: *mut DdNode) -> *mut DdNode {
    ((node as usize) & 01.not()) as *mut DdNode
}

/// Computes the complemented version of a node pointer (i.e. with a complement
/// bit set, regardless of its previous value).
pub const unsafe fn Cudd_Complement(node: *mut DdNode) -> *mut DdNode {
    ((node as usize) | 01) as *mut DdNode
}

/// Returns 1 if a pointer is complemented.
pub const unsafe fn Cudd_IsComplement(node: *mut DdNode) -> c_int {
    ((node as usize) & 1) as c_int
}

/*
/**
@brief Returns the current position in the order of variable
index.

@details Returns the current position in the order of variable
index. This macro is obsolete and is kept for compatibility. New
applications should use Cudd_ReadPerm instead.

@sideeffect none

@see Cudd_ReadPerm

 */
#define Cudd_ReadIndex(dd,index) (Cudd_ReadPerm(dd,index))


/**
@brief Iterates over the cubes of a decision diagram.

@details Iterates over the cubes of a decision diagram f.
<ul>
<li> DdManager *manager;
<li> DdNode *f;
<li> DdGen *gen;
<li> int *cube;
<li> CUDD_VALUE_TYPE value;
</ul>
Cudd_ForeachCube allocates and frees the generator. Therefore the
application should not try to do that. Also, the cube is freed at the
end of Cudd_ForeachCube and hence is not available outside of the loop.<p>
CAUTION: It is assumed that dynamic reordering will not occur while
there are open generators. It is the user's responsibility to make sure
that dynamic reordering does not occur. As long as new nodes are not created
during generation, and dynamic reordering is not called explicitly,
dynamic reordering will not occur. Alternatively, it is sufficient to
disable dynamic reordering. It is a mistake to dispose of a diagram
on which generation is ongoing.

@sideeffect none

@see Cudd_ForeachNode Cudd_FirstCube Cudd_NextCube Cudd_GenFree
Cudd_IsGenEmpty Cudd_AutodynDisable

 */
#define Cudd_ForeachCube(manager, f, gen, cube, value)\
for((gen) = Cudd_FirstCube(manager, f, &cube, &value);\
Cudd_IsGenEmpty(gen) ? Cudd_GenFree(gen) : CUDD_TRUE;\
(void) Cudd_NextCube(gen, &cube, &value))


/**
@brief Iterates over the primes of a Boolean function.

@details Iterates over the primes of a Boolean function producing
a prime, but not necessarily irredundant, cover.
<ul>
<li> DdManager *manager;
<li> DdNode *l;
<li> DdNode *u;
<li> DdGen *gen;
<li> int *cube;
</ul>
The Boolean function is described by an upper bound and a lower bound.  If
the function is completely specified, the two bounds coincide.
Cudd_ForeachPrime allocates and frees the generator.  Therefore the
application should not try to do that.  Also, the cube is freed at the
end of Cudd_ForeachPrime and hence is not available outside of the loop.<p>
CAUTION: It is a mistake to change a diagram on which generation is ongoing.

@sideeffect none

@see Cudd_ForeachCube Cudd_FirstPrime Cudd_NextPrime Cudd_GenFree
Cudd_IsGenEmpty

 */
#define Cudd_ForeachPrime(manager, l, u, gen, cube)\
for((gen) = Cudd_FirstPrime(manager, l, u, &cube);\
Cudd_IsGenEmpty(gen) ? Cudd_GenFree(gen) : CUDD_TRUE;\
(void) Cudd_NextPrime(gen, &cube))


/**
@brief Iterates over the nodes of a decision diagram.

@details Iterates over the nodes of a decision diagram f.
<ul>
<li> DdManager *manager;
<li> DdNode *f;
<li> DdGen *gen;
<li> DdNode *node;
</ul>
The nodes are returned in a seemingly random order.
Cudd_ForeachNode allocates and frees the generator. Therefore the
application should not try to do that.<p>
CAUTION: It is assumed that dynamic reordering will not occur while
there are open generators. It is the user's responsibility to make sure
that dynamic reordering does not occur. As long as new nodes are not created
during generation, and dynamic reordering is not called explicitly,
dynamic reordering will not occur. Alternatively, it is sufficient to
disable dynamic reordering. It is a mistake to dispose of a diagram
on which generation is ongoing.

@sideeffect none

@see Cudd_ForeachCube Cudd_FirstNode Cudd_NextNode Cudd_GenFree
Cudd_IsGenEmpty Cudd_AutodynDisable

 */
#define Cudd_ForeachNode(manager, f, gen, node)\
for((gen) = Cudd_FirstNode(manager, f, &node);\
Cudd_IsGenEmpty(gen) ? Cudd_GenFree(gen) : CUDD_TRUE;\
(void) Cudd_NextNode(gen, &node))


/**
@brief Iterates over the paths of a %ZDD.

@details Iterates over the paths of a %ZDD f.
<ul>
<li> DdManager *manager;
<li> DdNode *f;
<li> DdGen *gen;
<li> int *path;
</ul>
Cudd_zddForeachPath allocates and frees the generator. Therefore the
application should not try to do that. Also, the path is freed at the
end of Cudd_zddForeachPath and hence is not available outside of the loop.<p>
CAUTION: It is assumed that dynamic reordering will not occur while
there are open generators.  It is the user's responsibility to make sure
that dynamic reordering does not occur.  As long as new nodes are not created
during generation, and dynamic reordering is not called explicitly,
dynamic reordering will not occur.  Alternatively, it is sufficient to
disable dynamic reordering.  It is a mistake to dispose of a diagram
on which generation is ongoing.

@sideeffect none

@see Cudd_zddFirstPath Cudd_zddNextPath Cudd_GenFree
Cudd_IsGenEmpty Cudd_AutodynDisable

 */
#define Cudd_zddForeachPath(manager, f, gen, path)\
for((gen) = Cudd_zddFirstPath(manager, f, &path);\
Cudd_IsGenEmpty(gen) ? Cudd_GenFree(gen) : CUDD_TRUE;\
(void) Cudd_zddNextPath(gen, &path))


 */

extern "C" {
    pub fn Cudd_addNewVar(dd: *mut DdManager) -> *mut DdNode;
    pub fn Cudd_addNewVarAtLevel(dd: *mut DdManager, level: c_int) -> *mut DdNode;
    pub fn Cudd_bddNewVar(dd: *mut DdManager) -> *mut DdNode;
    pub fn Cudd_bddNewVarAtLevel(dd: *mut DdManager, level: c_int) -> *mut DdNode;
    pub fn Cudd_bddIsVar(dd: *mut DdManager, f: *mut DdNode) -> c_int;
    pub fn Cudd_addIthVar(dd: *mut DdManager, i: c_int) -> *mut DdNode;
    pub fn Cudd_bddIthVar(dd: *mut DdManager, i: c_int) -> *mut DdNode;
    pub fn Cudd_zddIthVar(dd: *mut DdManager, i: c_int) -> *mut DdNode;
    pub fn Cudd_zddVarsFromBddVars(dd: *mut DdManager, multiplicity: c_int) -> c_int;
    pub fn Cudd_ReadMaxIndex() -> c_uint;
    pub fn Cudd_addConst(dd: *mut DdManager, c: CUDD_VALUE_TYPE) -> *mut DdNode;
    pub fn Cudd_IsConstant(node: *mut DdNode) -> c_int;
    pub fn Cudd_IsNonConstant(f: *mut DdNode) -> c_int;
    pub fn Cudd_T(node: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_E(node: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_V(node: *mut DdNode) -> CUDD_VALUE_TYPE;
    pub fn Cudd_ReadStartTime(unique: *mut DdManager) -> c_ulong;
    pub fn Cudd_ReadElapsedTime(unique: *mut DdManager) -> c_ulong;
    pub fn Cudd_SetStartTime(unique: *mut DdManager, st: c_ulong) -> c_void;
    pub fn Cudd_ResetStartTime(unique: *mut DdManager) -> c_void;
    pub fn Cudd_ReadTimeLimit(unique: *mut DdManager) -> c_ulong;
    pub fn Cudd_SetTimeLimit(unique: *mut DdManager, tl: c_ulong) -> c_ulong;
    pub fn Cudd_UpdateTimeLimit(unique: *mut DdManager) -> c_void;
    pub fn Cudd_IncreaseTimeLimit(unique: *mut DdManager, increase: c_ulong) -> c_void;
    pub fn Cudd_UnsetTimeLimit(unique: *mut DdManager) -> c_void;
    pub fn Cudd_TimeLimited(unique: *mut DdManager) -> c_int;
    pub fn Cudd_RegisterTerminationCallback(
        unique: *mut DdManager,
        callback: DD_TERMINATION_HANDLER,
        callback_arg: *mut c_void,
    ) -> c_void;
    pub fn Cudd_UnregisterTerminationCallback(unique: *mut DdManager) -> c_void;
    pub fn Cudd_RegisterOutOfMemoryCallback(
        unique: *mut DdManager,
        callback: DD_OUT_OF_MEMORY_FUNCTION,
    ) -> DD_OUT_OF_MEMORY_FUNCTION;
    pub fn Cudd_UnregisterOutOfMemoryCallback(unique: *mut DdManager) -> c_void;
    pub fn Cudd_RegisterTimeoutHandler(
        unique: *mut DdManager,
        handler: DD_TIME_OUT_HANDLER,
        arg: *mut c_void,
    ) -> c_void;
    pub fn Cudd_ReadTimeoutHandler(
        unique: *mut DdManager,
        argp: *mut *mut c_void,
    ) -> DD_TIME_OUT_HANDLER;
    pub fn Cudd_AutodynEnable(unique: *mut DdManager, method: Cudd_ReorderingType) -> c_void;
    pub fn Cudd_AutodynDisable(unique: *mut DdManager) -> c_void;
    pub fn Cudd_ReorderingStatus(unique: *mut DdManager, method: *mut Cudd_ReorderingType)
        -> c_int;
    pub fn Cudd_AutodynEnableZdd(unique: *mut DdManager, method: Cudd_ReorderingType) -> c_void;
    pub fn Cudd_AutodynDisableZdd(unique: *mut DdManager) -> c_void;
    pub fn Cudd_ReorderingStatusZdd(
        unique: *mut DdManager,
        method: *mut Cudd_ReorderingType,
    ) -> c_int;
    pub fn Cudd_zddRealignmentEnabled(unique: *mut DdManager) -> c_int;
    pub fn Cudd_zddRealignEnable(unique: *mut DdManager) -> c_void;
    pub fn Cudd_zddRealignDisable(unique: *mut DdManager) -> c_void;
    pub fn Cudd_bddRealignmentEnabled(unique: *mut DdManager) -> c_int;
    pub fn Cudd_bddRealignEnable(unique: *mut DdManager) -> c_void;
    pub fn Cudd_bddRealignDisable(unique: *mut DdManager) -> c_void;
    pub fn Cudd_ReadOne(dd: *mut DdManager) -> *mut DdNode;
    pub fn Cudd_ReadZddOne(dd: *mut DdManager, i: c_int) -> *mut DdNode;
    pub fn Cudd_ReadZero(dd: *mut DdManager) -> *mut DdNode;
    pub fn Cudd_ReadLogicZero(dd: *mut DdManager) -> *mut DdNode;
    pub fn Cudd_ReadPlusInfinity(dd: *mut DdManager) -> *mut DdNode;
    pub fn Cudd_ReadMinusInfinity(dd: *mut DdManager) -> *mut DdNode;
    pub fn Cudd_ReadBackground(dd: *mut DdManager) -> *mut DdNode;
    pub fn Cudd_SetBackground(dd: *mut DdManager, bck: *mut DdNode) -> c_void;
    pub fn Cudd_ReadCacheSlots(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_ReadCacheUsedSlots(dd: *mut DdManager) -> c_double;
    pub fn Cudd_ReadCacheLookUps(dd: *mut DdManager) -> c_double;
    pub fn Cudd_ReadCacheHits(dd: *mut DdManager) -> c_double;
    pub fn Cudd_ReadRecursiveCalls(dd: *mut DdManager) -> c_double;
    pub fn Cudd_ReadMinHit(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_SetMinHit(dd: *mut DdManager, hr: c_uint) -> c_void;
    pub fn Cudd_ReadLooseUpTo(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_SetLooseUpTo(dd: *mut DdManager, lut: c_uint) -> c_void;
    pub fn Cudd_ReadMaxCache(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_ReadMaxCacheHard(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_SetMaxCacheHard(dd: *mut DdManager, mc: c_uint) -> c_void;
    pub fn Cudd_ReadSize(dd: *mut DdManager) -> c_int;
    pub fn Cudd_ReadZddSize(dd: *mut DdManager) -> c_int;
    pub fn Cudd_ReadSlots(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_ReadUsedSlots(dd: *mut DdManager) -> c_double;
    pub fn Cudd_ExpectedUsedSlots(dd: *mut DdManager) -> c_double;
    pub fn Cudd_ReadKeys(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_ReadDead(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_ReadMinDead(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_ReadReorderings(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_ReadMaxReorderings(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_SetMaxReorderings(dd: *mut DdManager, mr: c_uint) -> c_void;
    pub fn Cudd_ReadReorderingTime(dd: *mut DdManager) -> c_long;
    pub fn Cudd_ReadGarbageCollections(dd: *mut DdManager) -> c_int;
    pub fn Cudd_ReadGarbageCollectionTime(dd: *mut DdManager) -> c_long;
    pub fn Cudd_ReadNodesFreed(dd: *mut DdManager) -> c_double;
    pub fn Cudd_ReadNodesDropped(dd: *mut DdManager) -> c_double;
    pub fn Cudd_ReadUniqueLookUps(dd: *mut DdManager) -> c_double;
    pub fn Cudd_ReadUniqueLinks(dd: *mut DdManager) -> c_double;
    pub fn Cudd_ReadSiftMaxVar(dd: *mut DdManager) -> c_int;
    pub fn Cudd_SetSiftMaxVar(dd: *mut DdManager, smv: c_int) -> c_void;
    pub fn Cudd_ReadSiftMaxSwap(dd: *mut DdManager) -> c_int;
    pub fn Cudd_SetSiftMaxSwap(dd: *mut DdManager, sms: c_int) -> c_void;
    pub fn Cudd_ReadMaxGrowth(dd: *mut DdManager) -> c_double;
    pub fn Cudd_SetMaxGrowth(dd: *mut DdManager, mg: c_double) -> c_void;
    pub fn Cudd_ReadMaxGrowthAlternate(dd: *mut DdManager) -> c_double;
    pub fn Cudd_SetMaxGrowthAlternate(dd: *mut DdManager, mg: c_double) -> c_void;
    pub fn Cudd_ReadReorderingCycle(dd: *mut DdManager) -> c_int;
    pub fn Cudd_SetReorderingCycle(dd: *mut DdManager, cycle: c_int) -> c_void;
    pub fn Cudd_NodeReadIndex(dd: *mut DdNode) -> c_uint;
    pub fn Cudd_ReadPerm(dd: *mut DdManager, i: c_int) -> c_int;
    pub fn Cudd_ReadPermZdd(dd: *mut DdManager, i: c_int) -> c_int;
    pub fn Cudd_ReadInvPerm(dd: *mut DdManager, i: c_int) -> c_int;
    pub fn Cudd_ReadInvPermZdd(dd: *mut DdManager, i: c_int) -> c_int;
    pub fn Cudd_ReadVars(dd: *mut DdManager, i: c_int) -> *mut DdNode;
    pub fn Cudd_ReadEpsilon(dd: *mut DdManager) -> CUDD_VALUE_TYPE;
    pub fn Cudd_SetEpsilon(dd: *mut DdManager, ep: CUDD_VALUE_TYPE) -> c_void;
    pub fn Cudd_ReadGroupcheck(dd: *mut DdManager) -> Cudd_AggregationType;
    pub fn Cudd_SetGroupcheck(dd: *mut DdManager, gc: Cudd_AggregationType) -> c_void;
    pub fn Cudd_GarbageCollectionEnabled(dd: *mut DdManager) -> c_int;
    pub fn Cudd_EnableGarbageCollection(dd: *mut DdManager) -> c_void;
    pub fn Cudd_DisableGarbageCollection(dd: *mut DdManager) -> c_void;
    pub fn Cudd_DeadAreCounted(dd: *mut DdManager) -> c_int;
    pub fn Cudd_TurnOnCountDead(dd: *mut DdManager) -> c_void;
    pub fn Cudd_TurnOffCountDead(dd: *mut DdManager) -> c_void;
    pub fn Cudd_ReadRecomb(dd: *mut DdManager) -> c_int;
    pub fn Cudd_SetRecomb(dd: *mut DdManager, recomb: c_int) -> c_void;
    pub fn Cudd_ReadSymmviolation(dd: *mut DdManager) -> c_int;
    pub fn Cudd_SetSymmviolation(dd: *mut DdManager, symmviolation: c_int) -> c_void;
    pub fn Cudd_ReadArcviolation(dd: *mut DdManager) -> c_int;
    pub fn Cudd_SetArcviolation(dd: *mut DdManager, arcviolation: c_int) -> c_void;
    pub fn Cudd_ReadPopulationSize(dd: *mut DdManager) -> c_int;
    pub fn Cudd_SetPopulationSize(dd: *mut DdManager, populationSize: c_int) -> c_void;
    pub fn Cudd_ReadNumberXovers(dd: *mut DdManager) -> c_int;
    pub fn Cudd_SetNumberXovers(dd: *mut DdManager, numberXovers: c_int) -> c_void;
    pub fn Cudd_ReadOrderRandomization(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_SetOrderRandomization(dd: *mut DdManager, factor: c_uint) -> c_void;
    pub fn Cudd_ReadMemoryInUse(dd: *mut DdManager) -> c_ulong;
    pub fn Cudd_PrintInfo(dd: *mut DdManager, fp: *mut FILE) -> c_int;
    pub fn Cudd_ReadPeakNodeCount(dd: *mut DdManager) -> c_long;
    pub fn Cudd_ReadPeakLiveNodeCount(dd: *mut DdManager) -> c_int;
    pub fn Cudd_ReadNodeCount(dd: *mut DdManager) -> c_long;
    pub fn Cudd_zddReadNodeCount(dd: *mut DdManager) -> c_long;
    //pub fn Cudd_AddHook(dd: *mut DdManager, f: c_DD_HFP, where: Cudd_HookType ) -> c_int;
    //pub fn Cudd_RemoveHook(dd: *mut DdManager, f: c_DD_HFP, where: Cudd_HookType ) -> c_int;
    //pub fn Cudd_IsInHook(dd: *mut DdManager, f: c_DD_HFP, where: Cudd_HookType ) -> c_int;
    pub fn Cudd_StdPreReordHook(dd: *mut DdManager, str: *mut c_char, data: *mut c_void) -> c_int;
    pub fn Cudd_StdPostReordHook(dd: *mut DdManager, str: *mut c_char, data: *mut c_void) -> c_int;
    pub fn Cudd_EnableReorderingReporting(dd: *mut DdManager) -> c_int;
    pub fn Cudd_DisableReorderingReporting(dd: *mut DdManager) -> c_int;
    pub fn Cudd_ReorderingReporting(dd: *mut DdManager) -> c_int;
    pub fn Cudd_PrintGroupedOrder(dd: *mut DdManager, str: *mut c_char, data: *mut c_void)
        -> c_int;
    pub fn Cudd_EnableOrderingMonitoring(dd: *mut DdManager) -> c_int;
    pub fn Cudd_DisableOrderingMonitoring(dd: *mut DdManager) -> c_int;
    pub fn Cudd_OrderingMonitoring(dd: *mut DdManager) -> c_int;
    pub fn Cudd_SetApplicationHook(dd: *mut DdManager, value: *mut c_void) -> c_void;
    pub fn Cudd_ReadApplicationHook(dd: *mut DdManager) -> *mut c_void;
    pub fn Cudd_ReadErrorCode(dd: *mut DdManager) -> Cudd_ErrorType;
    pub fn Cudd_ClearErrorCode(dd: *mut DdManager) -> c_void;
    pub fn Cudd_ReadStdout(dd: *mut DdManager) -> *mut FILE;
    pub fn Cudd_SetStdout(dd: *mut DdManager, fp: *mut FILE) -> c_void;
    pub fn Cudd_ReadStderr(dd: *mut DdManager) -> *mut FILE;
    pub fn Cudd_SetStderr(dd: *mut DdManager, fp: *mut FILE) -> c_void;
    pub fn Cudd_ReadNextReordering(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_SetNextReordering(dd: *mut DdManager, next: c_uint) -> c_void;
    pub fn Cudd_ReadSwapSteps(dd: *mut DdManager) -> c_double;
    pub fn Cudd_ReadMaxLive(dd: *mut DdManager) -> c_uint;
    pub fn Cudd_SetMaxLive(dd: *mut DdManager, maxLive: c_uint) -> c_void;
    pub fn Cudd_ReadMaxMemory(dd: *mut DdManager) -> c_ulong;
    pub fn Cudd_SetMaxMemory(dd: *mut DdManager, maxMemory: c_ulong) -> c_void;
    pub fn Cudd_bddBindVar(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddUnbindVar(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddVarIsBound(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_addExistAbstract(
        manager: *mut DdManager,
        f: *mut DdNode,
        cube: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addUnivAbstract(
        manager: *mut DdManager,
        f: *mut DdNode,
        cube: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addOrAbstract(
        manager: *mut DdManager,
        f: *mut DdNode,
        cube: *mut DdNode,
    ) -> *mut DdNode;
    //M1: extern DdNode * Cudd_addApply (DdManager *dd, DdNode * (*)(DdManager *, DdNode **, DdNode **), DdNode *f, DdNode *g);

    pub fn Cudd_addPlus(
        dd: *mut DdManager,
        f: *mut *mut DdNode,
        g: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addTimes(
        dd: *mut DdManager,
        f: *mut *mut DdNode,
        g: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addThreshold(
        dd: *mut DdManager,
        f: *mut *mut DdNode,
        g: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addSetNZ(
        dd: *mut DdManager,
        f: *mut *mut DdNode,
        g: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addDivide(
        dd: *mut DdManager,
        f: *mut *mut DdNode,
        g: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addMinus(
        dd: *mut DdManager,
        f: *mut *mut DdNode,
        g: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addMinimum(
        dd: *mut DdManager,
        f: *mut *mut DdNode,
        g: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addMaximum(
        dd: *mut DdManager,
        f: *mut *mut DdNode,
        g: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addOneZeroMaximum(
        dd: *mut DdManager,
        f: *mut *mut DdNode,
        g: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addDiff(
        dd: *mut DdManager,
        f: *mut *mut DdNode,
        g: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addAgreement(
        dd: *mut DdManager,
        f: *mut *mut DdNode,
        g: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addOr(dd: *mut DdManager, f: *mut *mut DdNode, g: *mut *mut DdNode) -> *mut DdNode;
    pub fn Cudd_addNand(
        dd: *mut DdManager,
        f: *mut *mut DdNode,
        g: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addNor(dd: *mut DdManager, f: *mut *mut DdNode, g: *mut *mut DdNode)
        -> *mut DdNode;
    pub fn Cudd_addXor(dd: *mut DdManager, f: *mut *mut DdNode, g: *mut *mut DdNode)
        -> *mut DdNode;
    pub fn Cudd_addXnor(
        dd: *mut DdManager,
        f: *mut *mut DdNode,
        g: *mut *mut DdNode,
    ) -> *mut DdNode;
    //M1: extern DdNode * Cudd_addMonadicApply (DdManager * dd, DdNode * (*op)(DdManager *, DdNode *), DdNode * f);

    pub fn Cudd_addLog(dd: *mut DdManager, f: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_addFindMax(dd: *mut DdManager, f: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_addFindMin(dd: *mut DdManager, f: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_addIthBit(dd: *mut DdManager, f: *mut DdNode, bit: c_int) -> *mut DdNode;
    pub fn Cudd_addScalarInverse(
        dd: *mut DdManager,
        f: *mut DdNode,
        epsilon: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addIte(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        h: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addIteConstant(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        h: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addEvalConst(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_addLeq(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> c_int;
    pub fn Cudd_addCmpl(dd: *mut DdManager, f: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_addNegate(dd: *mut DdManager, f: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_addRoundOff(dd: *mut DdManager, f: *mut DdNode, N: c_int) -> *mut DdNode;
    pub fn Cudd_addWalsh(
        dd: *mut DdManager,
        x: *mut *mut DdNode,
        y: *mut *mut DdNode,
        n: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_addResidue(
        dd: *mut DdManager,
        n: c_int,
        m: c_int,
        options: c_int,
        top: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_bddAndAbstract(
        manager: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        cube: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_bddAndAbstractLimit(
        manager: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        cube: *mut DdNode,
        limit: c_uint,
    ) -> *mut DdNode;
    pub fn Cudd_ApaNumberOfDigits(binaryDigits: c_int) -> c_int;
    /*
    pub fn Cudd_NewApaNumber(digits: c_int) -> c_DdApaNumber;
    pub fn Cudd_ApaCopy(digits: c_int, source: c_DdApaNumber, dest: c_DdApaNumber) -> c_void;
    pub fn Cudd_ApaAdd(digits: c_int, a: c_DdApaNumber, b: c_DdApaNumber, sum: c_DdApaNumber) -> c_DdApaDigit;
    pub fn Cudd_ApaSubtract(digits: c_int, a: c_DdApaNumber, b: c_DdApaNumber, diff: c_DdApaNumber) -> c_DdApaDigit;
    pub fn Cudd_ApaShortDivision(digits: c_int, dividend: c_DdApaNumber, divisor: c_DdApaDigit, quotient: c_DdApaNumber) -> c_DdApaDigit;
    pub fn Cudd_ApaIntDivision(digits: c_int, dividend: c_DdApaNumber, divisor: c_uint, quotient: c_DdApaNumber) -> c_uint;
    pub fn Cudd_ApaShiftRight(digits: c_int, in: c_DdApaDigit, a: c_DdApaNumber, b: c_DdApaNumber) -> c_void;
    pub fn Cudd_ApaSetToLiteral(digits: c_int, number: c_DdApaNumber, literal: c_DdApaDigit) -> c_void;
    pub fn Cudd_ApaPowerOfTwo(digits: c_int, number: c_DdApaNumber, power: c_int) -> c_void;
    pub fn Cudd_ApaCompare(digitsFirst: c_int, first: c_DdApaNumber, digitsSecond: c_int, second: c_DdApaNumber) -> c_int;
    pub fn Cudd_ApaCompareRatios(digitsFirst: c_int, firstNum: c_DdApaNumber, firstDen: c_uint, digitsSecond: c_int, secondNum: c_DdApaNumber, secondDen: c_uint) -> c_int;
    pub fn Cudd_ApaPrintHex(fp: *mut FILE, digits: c_int, number: c_DdApaNumber) -> c_int;
    pub fn Cudd_ApaPrintDecimal(fp: *mut FILE, digits: c_int, number: c_DdApaNumber) -> c_int;
    pub fn Cudd_ApaPrintExponential(fp: *mut FILE, digits: c_int, number: c_DdApaNumber, precision: c_int) -> c_int;
    pub fn Cudd_ApaCountMinterm(manager: *mut DdManager, node: *mut DdNode, nvars: c_int, digits: *mut c_int) -> c_DdApaNumber;
    pub fn Cudd_ApaPrintMinterm(fp: *mut FILE, dd: *mut DdManager, node: *mut DdNode, nvars: c_int) -> c_int;
    pub fn Cudd_ApaPrintMintermExp(fp: *mut FILE, dd: *mut DdManager, node: *mut DdNode, nvars: c_int, precision: c_int) -> c_int;
    pub fn Cudd_ApaPrintDensity(fp: *mut FILE, dd: *mut DdManager, node: *mut DdNode, nvars: c_int) -> c_int;
    */
    pub fn Cudd_UnderApprox(
        dd: *mut DdManager,
        f: *mut DdNode,
        numVars: c_int,
        threshold: c_int,
        safe: c_int,
        quality: c_double,
    ) -> *mut DdNode;
    pub fn Cudd_OverApprox(
        dd: *mut DdManager,
        f: *mut DdNode,
        numVars: c_int,
        threshold: c_int,
        safe: c_int,
        quality: c_double,
    ) -> *mut DdNode;
    pub fn Cudd_RemapUnderApprox(
        dd: *mut DdManager,
        f: *mut DdNode,
        numVars: c_int,
        threshold: c_int,
        quality: c_double,
    ) -> *mut DdNode;
    pub fn Cudd_RemapOverApprox(
        dd: *mut DdManager,
        f: *mut DdNode,
        numVars: c_int,
        threshold: c_int,
        quality: c_double,
    ) -> *mut DdNode;
    pub fn Cudd_BiasedUnderApprox(
        dd: *mut DdManager,
        f: *mut DdNode,
        b: *mut DdNode,
        numVars: c_int,
        threshold: c_int,
        quality1: c_double,
        quality0: c_double,
    ) -> *mut DdNode;
    pub fn Cudd_BiasedOverApprox(
        dd: *mut DdManager,
        f: *mut DdNode,
        b: *mut DdNode,
        numVars: c_int,
        threshold: c_int,
        quality1: c_double,
        quality0: c_double,
    ) -> *mut DdNode;
    pub fn Cudd_bddExistAbstract(
        manager: *mut DdManager,
        f: *mut DdNode,
        cube: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_bddExistAbstractLimit(
        manager: *mut DdManager,
        f: *mut DdNode,
        cube: *mut DdNode,
        limit: c_uint,
    ) -> *mut DdNode;
    pub fn Cudd_bddXorExistAbstract(
        manager: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        cube: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_bddUnivAbstract(
        manager: *mut DdManager,
        f: *mut DdNode,
        cube: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_bddBooleanDiff(manager: *mut DdManager, f: *mut DdNode, x: c_int) -> *mut DdNode;
    pub fn Cudd_bddVarIsDependent(dd: *mut DdManager, f: *mut DdNode, var: *mut DdNode) -> c_int;
    pub fn Cudd_bddCorrelation(manager: *mut DdManager, f: *mut DdNode, g: *mut DdNode)
        -> c_double;
    pub fn Cudd_bddCorrelationWeights(
        manager: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        prob: *mut c_double,
    ) -> c_double;
    pub fn Cudd_bddIte(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        h: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_bddIteConstant(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        h: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_bddIntersect(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddAnd(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddAndLimit(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        limit: c_uint,
    ) -> *mut DdNode;
    pub fn Cudd_bddOr(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddOrLimit(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        limit: c_uint,
    ) -> *mut DdNode;
    pub fn Cudd_bddNand(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddNor(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddXor(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddXnor(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddXnorLimit(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        limit: c_uint,
    ) -> *mut DdNode;
    pub fn Cudd_bddLeq(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> c_int;
    pub fn Cudd_addBddThreshold(
        dd: *mut DdManager,
        f: *mut DdNode,
        value: CUDD_VALUE_TYPE,
    ) -> *mut DdNode;
    pub fn Cudd_addBddStrictThreshold(
        dd: *mut DdManager,
        f: *mut DdNode,
        value: CUDD_VALUE_TYPE,
    ) -> *mut DdNode;
    pub fn Cudd_addBddInterval(
        dd: *mut DdManager,
        f: *mut DdNode,
        lower: CUDD_VALUE_TYPE,
        upper: CUDD_VALUE_TYPE,
    ) -> *mut DdNode;
    pub fn Cudd_addBddIthBit(dd: *mut DdManager, f: *mut DdNode, bit: c_int) -> *mut DdNode;
    pub fn Cudd_BddToAdd(dd: *mut DdManager, B: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_addBddPattern(dd: *mut DdManager, f: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddTransfer(
        ddSource: *mut DdManager,
        ddDestination: *mut DdManager,
        f: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_DebugCheck(table: *mut DdManager) -> c_int;
    pub fn Cudd_CheckKeys(table: *mut DdManager) -> c_int;
    pub fn Cudd_bddClippingAnd(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        maxDepth: c_int,
        direction: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_bddClippingAndAbstract(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        cube: *mut DdNode,
        maxDepth: c_int,
        direction: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_Cofactor(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_CheckCube(dd: *mut DdManager, g: *mut DdNode) -> c_int;
    pub fn Cudd_bddCompose(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        v: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_addCompose(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        v: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_addPermute(
        manager: *mut DdManager,
        node: *mut DdNode,
        permut: *mut c_int,
    ) -> *mut DdNode;
    pub fn Cudd_addSwapVariables(
        dd: *mut DdManager,
        f: *mut DdNode,
        x: *mut *mut DdNode,
        y: *mut *mut DdNode,
        n: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_bddPermute(
        manager: *mut DdManager,
        node: *mut DdNode,
        permut: *mut c_int,
    ) -> *mut DdNode;
    pub fn Cudd_bddVarMap(manager: *mut DdManager, f: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_SetVarMap(
        manager: *mut DdManager,
        x: *mut *mut DdNode,
        y: *mut *mut DdNode,
        n: c_int,
    ) -> c_int;
    pub fn Cudd_bddSwapVariables(
        dd: *mut DdManager,
        f: *mut DdNode,
        x: *mut *mut DdNode,
        y: *mut *mut DdNode,
        n: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_bddAdjPermuteX(
        dd: *mut DdManager,
        B: *mut DdNode,
        x: *mut *mut DdNode,
        n: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_addVectorCompose(
        dd: *mut DdManager,
        f: *mut DdNode,
        vector: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addGeneralVectorCompose(
        dd: *mut DdManager,
        f: *mut DdNode,
        vectorOn: *mut *mut DdNode,
        vectorOff: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addNonSimCompose(
        dd: *mut DdManager,
        f: *mut DdNode,
        vector: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_bddVectorCompose(
        dd: *mut DdManager,
        f: *mut DdNode,
        vector: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_bddApproxConjDecomp(
        dd: *mut DdManager,
        f: *mut DdNode,
        conjuncts: *mut *mut *mut DdNode,
    ) -> c_int;
    pub fn Cudd_bddApproxDisjDecomp(
        dd: *mut DdManager,
        f: *mut DdNode,
        disjuncts: *mut *mut *mut DdNode,
    ) -> c_int;
    pub fn Cudd_bddIterConjDecomp(
        dd: *mut DdManager,
        f: *mut DdNode,
        conjuncts: *mut *mut *mut DdNode,
    ) -> c_int;
    pub fn Cudd_bddIterDisjDecomp(
        dd: *mut DdManager,
        f: *mut DdNode,
        disjuncts: *mut *mut *mut DdNode,
    ) -> c_int;
    pub fn Cudd_bddGenConjDecomp(
        dd: *mut DdManager,
        f: *mut DdNode,
        conjuncts: *mut *mut *mut DdNode,
    ) -> c_int;
    pub fn Cudd_bddGenDisjDecomp(
        dd: *mut DdManager,
        f: *mut DdNode,
        disjuncts: *mut *mut *mut DdNode,
    ) -> c_int;
    pub fn Cudd_bddVarConjDecomp(
        dd: *mut DdManager,
        f: *mut DdNode,
        conjuncts: *mut *mut *mut DdNode,
    ) -> c_int;
    pub fn Cudd_bddVarDisjDecomp(
        dd: *mut DdManager,
        f: *mut DdNode,
        disjuncts: *mut *mut *mut DdNode,
    ) -> c_int;
    pub fn Cudd_FindEssential(dd: *mut DdManager, f: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddIsVarEssential(
        manager: *mut DdManager,
        f: *mut DdNode,
        id: c_int,
        phase: c_int,
    ) -> c_int;
    pub fn Cudd_FindTwoLiteralClauses(dd: *mut DdManager, f: *mut DdNode) -> *mut DdTlcInfo;
    pub fn Cudd_PrintTwoLiteralClauses(
        dd: *mut DdManager,
        f: *mut DdNode,
        names: *mut *mut c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Cudd_ReadIthClause(
        tlc: *mut DdTlcInfo,
        i: c_int,
        var1: *mut DdHalfWord,
        var2: *mut DdHalfWord,
        phase1: *mut c_int,
        phase2: *mut c_int,
    ) -> c_int;
    pub fn Cudd_tlcInfoFree(t: *mut DdTlcInfo) -> c_void;
    pub fn Cudd_DumpBlif(
        dd: *mut DdManager,
        n: c_int,
        f: *mut *mut DdNode,
        inames: *const *const c_char,
        onames: *const *const c_char,
        mname: *mut c_char,
        fp: *mut FILE,
        mv: c_int,
    ) -> c_int;
    pub fn Cudd_DumpBlifBody(
        dd: *mut DdManager,
        n: c_int,
        f: *mut *mut DdNode,
        inames: *const *const c_char,
        onames: *const *const c_char,
        fp: *mut FILE,
        mv: c_int,
    ) -> c_int;
    pub fn Cudd_DumpDot(
        dd: *mut DdManager,
        n: c_int,
        f: *mut *mut DdNode,
        inames: *const *const c_char,
        onames: *const *const c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Cudd_DumpDaVinci(
        dd: *mut DdManager,
        n: c_int,
        f: *mut *mut DdNode,
        inames: *const *const c_char,
        onames: *const *const c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Cudd_DumpDDcal(
        dd: *mut DdManager,
        n: c_int,
        f: *mut *mut DdNode,
        inames: *const *const c_char,
        onames: *const *const c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Cudd_DumpFactoredForm(
        dd: *mut DdManager,
        n: c_int,
        f: *mut *mut DdNode,
        inames: *const *const c_char,
        onames: *const *const c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Cudd_FactoredFormString(
        dd: *mut DdManager,
        f: *mut DdNode,
        inames: *const *const c_char,
    ) -> *mut c_char;
    pub fn Cudd_bddConstrain(dd: *mut DdManager, f: *mut DdNode, c: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddRestrict(dd: *mut DdManager, f: *mut DdNode, c: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddNPAnd(dd: *mut DdManager, f: *mut DdNode, c: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_addConstrain(dd: *mut DdManager, f: *mut DdNode, c: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddConstrainDecomp(dd: *mut DdManager, f: *mut DdNode) -> *mut *mut DdNode;
    pub fn Cudd_addRestrict(dd: *mut DdManager, f: *mut DdNode, c: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddCharToVect(dd: *mut DdManager, f: *mut DdNode) -> *mut *mut DdNode;
    pub fn Cudd_bddLICompaction(dd: *mut DdManager, f: *mut DdNode, c: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddSqueeze(dd: *mut DdManager, l: *mut DdNode, u: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddMinimize(dd: *mut DdManager, f: *mut DdNode, c: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_SubsetCompress(
        dd: *mut DdManager,
        f: *mut DdNode,
        nvars: c_int,
        threshold: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_SupersetCompress(
        dd: *mut DdManager,
        f: *mut DdNode,
        nvars: c_int,
        threshold: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_MakeTreeNode(
        dd: *mut DdManager,
        low: c_uint,
        size: c_uint,
        ttype: c_uint,
    ) -> *mut MtrNode;
    pub fn Cudd_addHarwell(
        fp: *mut FILE,
        dd: *mut DdManager,
        E: *mut *mut DdNode,
        x: *mut *mut *mut DdNode,
        y: *mut *mut *mut DdNode,
        xn: *mut *mut *mut DdNode,
        yn_: *mut *mut *mut DdNode,
        nx: *mut c_int,
        ny: *mut c_int,
        m: *mut c_int,
        n: *mut c_int,
        bx: c_int,
        sx: c_int,
        by: c_int,
        sy: c_int,
        pr: c_int,
    ) -> c_int;
    pub fn Cudd_Init(
        numVars: c_uint,
        numVarsZ: c_uint,
        numSlots: c_uint,
        cacheSize: c_uint,
        maxMemory: c_ulong,
    ) -> *mut DdManager;
    pub fn Cudd_Quit(unique: *mut DdManager) -> c_void;
    pub fn Cudd_PrintLinear(table: *mut DdManager) -> c_int;
    pub fn Cudd_ReadLinear(table: *mut DdManager, x: c_int, y: c_int) -> c_int;
    pub fn Cudd_bddLiteralSetIntersection(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addMatrixMultiply(
        dd: *mut DdManager,
        A: *mut DdNode,
        B: *mut DdNode,
        z: *mut *mut DdNode,
        nz: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_addTimesPlus(
        dd: *mut DdManager,
        A: *mut DdNode,
        B: *mut DdNode,
        z: *mut *mut DdNode,
        nz: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_addTriangle(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        z: *mut *mut DdNode,
        nz: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_addOuterSum(
        dd: *mut DdManager,
        M: *mut DdNode,
        r: *mut DdNode,
        c: *mut DdNode,
    ) -> *mut DdNode;
    //M1: extern DdNode * Cudd_PrioritySelect (DdManager *dd, DdNode *R, DdNode **x, DdNode **y, DdNode **z, DdNode *Pi, int n, DdNode * (*)(DdManager *, int, DdNode **, DdNode **, DdNode **));

    pub fn Cudd_Xgty(
        dd: *mut DdManager,
        N: c_int,
        z: *mut *mut DdNode,
        x: *mut *mut DdNode,
        y: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_Xeqy(
        dd: *mut DdManager,
        N: c_int,
        x: *mut *mut DdNode,
        y: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_addXeqy(
        dd: *mut DdManager,
        N: c_int,
        x: *mut *mut DdNode,
        y: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_Dxygtdxz(
        dd: *mut DdManager,
        N: c_int,
        x: *mut *mut DdNode,
        y: *mut *mut DdNode,
        z: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_Dxygtdyz(
        dd: *mut DdManager,
        N: c_int,
        x: *mut *mut DdNode,
        y: *mut *mut DdNode,
        z: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_Inequality(
        dd: *mut DdManager,
        N: c_int,
        c: c_int,
        x: *mut *mut DdNode,
        y: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_Disequality(
        dd: *mut DdManager,
        N: c_int,
        c: c_int,
        x: *mut *mut DdNode,
        y: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_bddInterval(
        dd: *mut DdManager,
        N: c_int,
        x: *mut *mut DdNode,
        lowerB: c_uint,
        upperB: c_uint,
    ) -> *mut DdNode;
    pub fn Cudd_CProjection(dd: *mut DdManager, R: *mut DdNode, Y: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_addHamming(
        dd: *mut DdManager,
        xVars: *mut *mut DdNode,
        yVars: *mut *mut DdNode,
        nVars: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_MinHammingDist(
        dd: *mut DdManager,
        f: *mut DdNode,
        minterm: *mut c_int,
        upperBound: c_int,
    ) -> c_int;
    pub fn Cudd_bddClosestCube(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        distance: *mut c_int,
    ) -> *mut DdNode;
    pub fn Cudd_addRead(
        fp: *mut FILE,
        dd: *mut DdManager,
        E: *mut *mut DdNode,
        x: *mut *mut *mut DdNode,
        y: *mut *mut *mut DdNode,
        xn: *mut *mut *mut DdNode,
        yn_: *mut *mut *mut DdNode,
        nx: *mut c_int,
        ny: *mut c_int,
        m: *mut c_int,
        n: *mut c_int,
        bx: c_int,
        sx: c_int,
        by: c_int,
        sy: c_int,
    ) -> c_int;
    pub fn Cudd_bddRead(
        fp: *mut FILE,
        dd: *mut DdManager,
        E: *mut *mut DdNode,
        x: *mut *mut *mut DdNode,
        y: *mut *mut *mut DdNode,
        nx: *mut c_int,
        ny: *mut c_int,
        m: *mut c_int,
        n: *mut c_int,
        bx: c_int,
        sx: c_int,
        by: c_int,
        sy: c_int,
    ) -> c_int;
    pub fn Cudd_Ref(n: *mut DdNode) -> c_void;
    pub fn Cudd_RecursiveDeref(table: *mut DdManager, n: *mut DdNode) -> c_void;
    pub fn Cudd_IterDerefBdd(table: *mut DdManager, n: *mut DdNode) -> c_void;
    pub fn Cudd_DelayedDerefBdd(table: *mut DdManager, n: *mut DdNode) -> c_void;
    pub fn Cudd_RecursiveDerefZdd(table: *mut DdManager, n: *mut DdNode) -> c_void;
    pub fn Cudd_Deref(node: *mut DdNode) -> c_void;
    pub fn Cudd_CheckZeroRef(manager: *mut DdManager) -> c_int;
    pub fn Cudd_ReduceHeap(
        table: *mut DdManager,
        heuristic: Cudd_ReorderingType,
        minsize: c_int,
    ) -> c_int;
    pub fn Cudd_ShuffleHeap(table: *mut DdManager, permutation: *mut c_int) -> c_int;
    pub fn Cudd_Eval(dd: *mut DdManager, f: *mut DdNode, inputs: *mut c_int) -> *mut DdNode;
    pub fn Cudd_ShortestPath(
        manager: *mut DdManager,
        f: *mut DdNode,
        weight: *mut c_int,
        support: *mut c_int,
        length: *mut c_int,
    ) -> *mut DdNode;
    pub fn Cudd_LargestCube(
        manager: *mut DdManager,
        f: *mut DdNode,
        length: *mut c_int,
    ) -> *mut DdNode;
    pub fn Cudd_ShortestLength(
        manager: *mut DdManager,
        f: *mut DdNode,
        weight: *mut c_int,
    ) -> c_int;
    pub fn Cudd_Decreasing(dd: *mut DdManager, f: *mut DdNode, i: c_int) -> *mut DdNode;
    pub fn Cudd_Increasing(dd: *mut DdManager, f: *mut DdNode, i: c_int) -> *mut DdNode;
    pub fn Cudd_EquivDC(
        dd: *mut DdManager,
        F: *mut DdNode,
        G: *mut DdNode,
        D: *mut DdNode,
    ) -> c_int;
    pub fn Cudd_bddLeqUnless(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        D: *mut DdNode,
    ) -> c_int;
    pub fn Cudd_EqualSupNorm(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        tolerance: CUDD_VALUE_TYPE,
        pr: c_int,
    ) -> c_int;
    pub fn Cudd_bddMakePrime(dd: *mut DdManager, cube: *mut DdNode, f: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_bddMaximallyExpand(
        dd: *mut DdManager,
        lb: *mut DdNode,
        ub: *mut DdNode,
        f: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_bddLargestPrimeUnate(
        dd: *mut DdManager,
        f: *mut DdNode,
        phaseBdd: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_CofMinterm(dd: *mut DdManager, node: *mut DdNode) -> *mut c_double;
    pub fn Cudd_SolveEqn(
        bdd: *mut DdManager,
        F: *mut DdNode,
        Y: *mut DdNode,
        G: *mut *mut DdNode,
        yIndex: *mut *mut c_int,
        n: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_VerifySol(
        bdd: *mut DdManager,
        F: *mut DdNode,
        G: *mut *mut DdNode,
        yIndex: *mut c_int,
        n: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_SplitSet(
        manager: *mut DdManager,
        S: *mut DdNode,
        xVars: *mut *mut DdNode,
        n: c_int,
        m: c_double,
    ) -> *mut DdNode;
    pub fn Cudd_SubsetHeavyBranch(
        dd: *mut DdManager,
        f: *mut DdNode,
        numVars: c_int,
        threshold: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_SupersetHeavyBranch(
        dd: *mut DdManager,
        f: *mut DdNode,
        numVars: c_int,
        threshold: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_SubsetShortPaths(
        dd: *mut DdManager,
        f: *mut DdNode,
        numVars: c_int,
        threshold: c_int,
        hardlimit: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_SupersetShortPaths(
        dd: *mut DdManager,
        f: *mut DdNode,
        numVars: c_int,
        threshold: c_int,
        hardlimit: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_SymmProfile(table: *mut DdManager, lower: c_int, upper: c_int) -> c_void;
    pub fn Cudd_Prime(p: c_uint) -> c_uint;
    pub fn Cudd_Reserve(manager: *mut DdManager, amount: c_int) -> c_int;
    pub fn Cudd_PrintMinterm(manager: *mut DdManager, node: *mut DdNode) -> c_int;
    pub fn Cudd_bddPrintCover(dd: *mut DdManager, l: *mut DdNode, u: *mut DdNode) -> c_int;
    pub fn Cudd_PrintDebug(dd: *mut DdManager, f: *mut DdNode, n: c_int, pr: c_int) -> c_int;
    pub fn Cudd_DagSize(node: *mut DdNode) -> c_int;
    pub fn Cudd_EstimateCofactor(
        dd: *mut DdManager,
        node: *mut DdNode,
        i: c_int,
        phase: c_int,
    ) -> c_int;
    pub fn Cudd_EstimateCofactorSimple(node: *mut DdNode, i: c_int) -> c_int;
    pub fn Cudd_SharingSize(nodeArray: *mut *mut DdNode, n: c_int) -> c_int;
    pub fn Cudd_CountMinterm(manager: *mut DdManager, node: *mut DdNode, nvars: c_int) -> c_double;
    //pub fn Cudd_EpdCountMinterm(manager: *mut DdManager, node: *mut DdNode, nvars: c_int, epd: *mut c_EpDouble) -> c_int;
    pub fn Cudd_CountPath(node: *mut DdNode) -> c_double;
    pub fn Cudd_CountPathsToNonZero(node: *mut DdNode) -> c_double;
    pub fn Cudd_SupportIndices(
        dd: *mut DdManager,
        f: *mut DdNode,
        indices: *mut *mut c_int,
    ) -> c_int;
    pub fn Cudd_Support(dd: *mut DdManager, f: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_SupportIndex(dd: *mut DdManager, f: *mut DdNode) -> *mut c_int;
    pub fn Cudd_SupportSize(dd: *mut DdManager, f: *mut DdNode) -> c_int;
    pub fn Cudd_VectorSupportIndices(
        dd: *mut DdManager,
        F: *mut *mut DdNode,
        n: c_int,
        indices: *mut *mut c_int,
    ) -> c_int;
    pub fn Cudd_VectorSupport(dd: *mut DdManager, F: *mut *mut DdNode, n: c_int) -> *mut DdNode;
    pub fn Cudd_VectorSupportIndex(dd: *mut DdManager, F: *mut *mut DdNode, n: c_int)
        -> *mut c_int;
    pub fn Cudd_VectorSupportSize(dd: *mut DdManager, F: *mut *mut DdNode, n: c_int) -> c_int;
    pub fn Cudd_ClassifySupport(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        common: *mut *mut DdNode,
        onlyF: *mut *mut DdNode,
        onlyG: *mut *mut DdNode,
    ) -> c_int;
    pub fn Cudd_CountLeaves(node: *mut DdNode) -> c_int;
    pub fn Cudd_bddPickOneCube(
        ddm: *mut DdManager,
        node: *mut DdNode,
        string: *mut c_char,
    ) -> c_int;
    pub fn Cudd_bddPickOneMinterm(
        dd: *mut DdManager,
        f: *mut DdNode,
        vars: *mut *mut DdNode,
        n: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_bddPickArbitraryMinterms(
        dd: *mut DdManager,
        f: *mut DdNode,
        vars: *mut *mut DdNode,
        n: c_int,
        k: c_int,
    ) -> *mut *mut DdNode;
    pub fn Cudd_SubsetWithMaskVars(
        dd: *mut DdManager,
        f: *mut DdNode,
        vars: *mut *mut DdNode,
        nvars: c_int,
        maskVars: *mut *mut DdNode,
        mvars: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_FirstCube(
        dd: *mut DdManager,
        f: *mut DdNode,
        cube: *mut *mut c_int,
        value: *mut CUDD_VALUE_TYPE,
    ) -> *mut DdGen;
    pub fn Cudd_NextCube(
        gen: *mut DdGen,
        cube: *mut *mut c_int,
        value: *mut CUDD_VALUE_TYPE,
    ) -> c_int;
    pub fn Cudd_FirstPrime(
        dd: *mut DdManager,
        l: *mut DdNode,
        u: *mut DdNode,
        cube: *mut *mut c_int,
    ) -> *mut DdGen;
    pub fn Cudd_NextPrime(gen: *mut DdGen, cube: *mut *mut c_int) -> c_int;
    pub fn Cudd_bddComputeCube(
        dd: *mut DdManager,
        vars: *mut *mut DdNode,
        phase: *mut c_int,
        n: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_addComputeCube(
        dd: *mut DdManager,
        vars: *mut *mut DdNode,
        phase: *mut c_int,
        n: c_int,
    ) -> *mut DdNode;
    pub fn Cudd_CubeArrayToBdd(dd: *mut DdManager, array: *mut c_int) -> *mut DdNode;
    pub fn Cudd_BddToCubeArray(dd: *mut DdManager, cube: *mut DdNode, array: *mut c_int) -> c_int;
    pub fn Cudd_FirstNode(dd: *mut DdManager, f: *mut DdNode, node: *mut *mut DdNode)
        -> *mut DdGen;
    pub fn Cudd_NextNode(gen: *mut DdGen, node: *mut *mut DdNode) -> c_int;
    pub fn Cudd_GenFree(gen: *mut DdGen) -> c_int;
    pub fn Cudd_IsGenEmpty(gen: *mut DdGen) -> c_int;
    pub fn Cudd_IndicesToCube(dd: *mut DdManager, array: *mut c_int, n: c_int) -> *mut DdNode;
    pub fn Cudd_PrintVersion(fp: *mut FILE) -> c_void;
    pub fn Cudd_AverageDistance(dd: *mut DdManager) -> c_double;
    //No match: extern long Cudd_Random (void);
    pub fn Cudd_Srandom(seed: c_long) -> c_void;
    pub fn Cudd_Density(dd: *mut DdManager, f: *mut DdNode, nvars: c_int) -> c_double;
    pub fn Cudd_OutOfMem(size: c_long) -> c_void;
    pub fn Cudd_zddCount(zdd: *mut DdManager, P: *mut DdNode) -> c_int;
    pub fn Cudd_zddCountDouble(zdd: *mut DdManager, P: *mut DdNode) -> c_double;
    pub fn Cudd_zddProduct(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddUnateProduct(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddWeakDiv(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddDivide(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddWeakDivF(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddDivideF(dd: *mut DdManager, f: *mut DdNode, g: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddComplement(dd: *mut DdManager, node: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_MakeZddTreeNode(
        dd: *mut DdManager,
        low: c_uint,
        size: c_uint,
        ttype: c_uint,
    ) -> *mut MtrNode;
    pub fn Cudd_zddIsop(
        dd: *mut DdManager,
        L: *mut DdNode,
        U: *mut DdNode,
        zdd_I: *mut *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_bddIsop(dd: *mut DdManager, L: *mut DdNode, U: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_MakeBddFromZddCover(dd: *mut DdManager, node: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddDagSize(p_node: *mut DdNode) -> c_int;
    pub fn Cudd_zddCountMinterm(zdd: *mut DdManager, node: *mut DdNode, path: c_int) -> c_double;
    pub fn Cudd_zddPrintSubtable(table: *mut DdManager) -> c_void;
    pub fn Cudd_zddPortFromBdd(dd: *mut DdManager, B: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddPortToBdd(dd: *mut DdManager, f: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddReduceHeap(
        table: *mut DdManager,
        heuristic: Cudd_ReorderingType,
        minsize: c_int,
    ) -> c_int;
    pub fn Cudd_zddShuffleHeap(table: *mut DdManager, permutation: *mut c_int) -> c_int;
    pub fn Cudd_zddIte(
        dd: *mut DdManager,
        f: *mut DdNode,
        g: *mut DdNode,
        h: *mut DdNode,
    ) -> *mut DdNode;
    pub fn Cudd_zddUnion(dd: *mut DdManager, P: *mut DdNode, Q: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddIntersect(dd: *mut DdManager, P: *mut DdNode, Q: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddDiff(dd: *mut DdManager, P: *mut DdNode, Q: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddDiffConst(zdd: *mut DdManager, P: *mut DdNode, Q: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddSubset1(dd: *mut DdManager, P: *mut DdNode, var: c_int) -> *mut DdNode;
    pub fn Cudd_zddSubset0(dd: *mut DdManager, P: *mut DdNode, var: c_int) -> *mut DdNode;
    pub fn Cudd_zddChange(dd: *mut DdManager, P: *mut DdNode, var: c_int) -> *mut DdNode;
    pub fn Cudd_zddSymmProfile(table: *mut DdManager, lower: c_int, upper: c_int) -> c_void;
    pub fn Cudd_zddPrintMinterm(zdd: *mut DdManager, node: *mut DdNode) -> c_int;
    pub fn Cudd_zddPrintCover(zdd: *mut DdManager, node: *mut DdNode) -> c_int;
    pub fn Cudd_zddPrintDebug(zdd: *mut DdManager, f: *mut DdNode, n: c_int, pr: c_int) -> c_int;
    pub fn Cudd_zddFirstPath(
        zdd: *mut DdManager,
        f: *mut DdNode,
        path: *mut *mut c_int,
    ) -> *mut DdGen;
    pub fn Cudd_zddNextPath(gen: *mut DdGen, path: *mut *mut c_int) -> c_int;
    pub fn Cudd_zddCoverPathToString(
        zdd: *mut DdManager,
        path: *mut c_int,
        str: *mut c_char,
    ) -> *mut c_char;
    pub fn Cudd_zddSupport(dd: *mut DdManager, f: *mut DdNode) -> *mut DdNode;
    pub fn Cudd_zddDumpDot(
        dd: *mut DdManager,
        n: c_int,
        f: *mut *mut DdNode,
        inames: *const *const c_char,
        onames: *const *const c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Cudd_bddSetPiVar(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddSetPsVar(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddSetNsVar(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddIsPiVar(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddIsPsVar(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddIsNsVar(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddSetPairIndex(dd: *mut DdManager, index: c_int, pairIndex: c_int) -> c_int;
    pub fn Cudd_bddReadPairIndex(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddSetVarToBeGrouped(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddSetVarHardGroup(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddResetVarToBeGrouped(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddIsVarToBeGrouped(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddSetVarToBeUngrouped(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddIsVarToBeUngrouped(dd: *mut DdManager, index: c_int) -> c_int;
    pub fn Cudd_bddIsVarHardGroup(dd: *mut DdManager, index: c_int) -> c_int;
}
