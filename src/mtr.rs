use libc::{c_int, c_uint, c_void, FILE};
use MtrNode;

/// Default flag value in `Mtr_MakeGroup`.
pub const MTR_DEFAULT: c_uint = 0;
/// See `Mtr_MakeGroup`.
pub const MTR_TERMINAL: c_uint = 1;
/// See `Mtr_MakeGroup`.
pub const MTR_SOFT: c_uint = 2;
/// See `Mtr_MakeGroup`.
pub const MTR_FIXED: c_uint = 4;
/// See `Mtr_MakeGroup`.
pub const MTR_NEWNODE: c_uint = 8;

extern "C" {
    pub fn Mtr_AllocNode() -> *mut MtrNode;
    pub fn Mtr_DeallocNode(node: *mut MtrNode) -> c_void;
    pub fn Mtr_InitTree() -> *mut MtrNode;
    pub fn Mtr_FreeTree(node: *mut MtrNode) -> c_void;
    pub fn Mtr_CopyTree(node: *const MtrNode, expansion: c_int) -> *mut MtrNode;
    pub fn Mtr_MakeFirstChild(parent: *mut MtrNode, child: *mut MtrNode) -> c_void;
    pub fn Mtr_MakeLastChild(parent: *mut MtrNode, child: *mut MtrNode) -> c_void;
    pub fn Mtr_CreateFirstChild(parent: *mut MtrNode) -> *mut MtrNode;
    pub fn Mtr_CreateLastChild(parent: *mut MtrNode) -> *mut MtrNode;
    pub fn Mtr_MakeNextSibling(first: *mut MtrNode, second: *mut MtrNode) -> c_void;
    pub fn Mtr_PrintTree(node: *const MtrNode) -> c_void;
    pub fn Mtr_InitGroupTree(lower: c_int, size: c_int) -> *mut MtrNode;
    pub fn Mtr_MakeGroup(
        root: *mut MtrNode,
        low: c_uint,
        high: c_uint,
        flags: c_uint,
    ) -> *mut MtrNode;
    pub fn Mtr_DissolveGroup(group: *mut MtrNode) -> *mut MtrNode;
    pub fn Mtr_FindGroup(root: *mut MtrNode, low: c_uint, high: c_uint) -> *mut MtrNode;
    pub fn Mtr_SwapGroups(first: *mut MtrNode, second: *mut MtrNode) -> c_int;
    pub fn Mtr_ReorderGroups(treenode: *mut MtrNode, permutation: *mut c_int) -> c_void;
    pub fn Mtr_PrintGroups(root: *const MtrNode, silent: c_int) -> c_void;
    pub fn Mtr_PrintGroupedOrder(
        root: *const MtrNode,
        invperm: *const c_int,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Mtr_ReadGroups(fp: *mut FILE, nleaves: c_int) -> *mut MtrNode;
}
