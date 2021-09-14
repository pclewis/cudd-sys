use libc::{c_char, c_int, FILE};
use {DdManager, DdNode};

/// Version of DDDMP format.
pub const DDDMP_VERSION: &str = "DDDMP-2.0";

/// Returned by functions when failed.
pub const DDDMP_FAILURE: c_int = 0;

/// Returned by functions when succeeded.
pub const DDDMP_SUCCESS: c_int = 1;

/// Text formatting mode.
pub const DDDMP_MODE_TEXT: c_int = 'A' as c_int;

/// Binary formatting mode.
pub const DDDMP_MODE_BINARY: c_int = 'B' as c_int;

/// Default formatting mode.
pub const DDDMP_MODE_DEFAULT: c_int = 'D' as c_int;

#[repr(C)]
pub enum Dddmp_DecompCnfStoreType {
    DDDMP_CNF_MODE_NODE,
    DDDMP_CNF_MODE_MAXTERM,
    DDDMP_CNF_MODE_BEST,
}

#[repr(C)]
pub enum Dddmp_DecompCnfLoadType {
    DDDMP_CNF_MODE_NO_CONJ,
    DDDMP_CNF_MODE_NO_QUANT,
    DDDMP_CNF_MODE_CONJ_QUANT,
}

#[repr(C)]
pub enum Dddmp_DecompType {
    DDDMP_BDD,
    DDDMP_ADD,
    DDDMP_CNF,
    DDDMP_NONE,
}

#[repr(C)]
pub enum Dddmp_VarInfoType {
    DDDMP_VARIDS,
    DDDMP_VARPERMIDS,
    DDDMP_VARAUXIDS,
    DDDMP_VARNAMES,
    DDDMP_VARDEFAULT,
}

#[repr(C)]
pub enum Dddmp_VarMatchType {
    DDDMP_VAR_MATCHIDS,
    DDDMP_VAR_MATCHPERMIDS,
    DDDMP_VAR_MATCHAUXIDS,
    DDDMP_VAR_MATCHNAMES,
    DDDMP_VAR_COMPOSEIDS,
}

#[repr(C)]
pub enum Dddmp_RootMatchType {
    DDDMP_ROOT_MATCHNAMES,
    DDDMP_ROOT_MATCHLIST,
}

extern "C" {
    pub fn Dddmp_Text2Bin(filein: *mut c_char, fileout: *mut c_char) -> c_int;
    pub fn Dddmp_Bin2Text(filein: *mut c_char, fileout: *mut c_char) -> c_int;
    pub fn Dddmp_cuddBddDisplayBinary(fileIn: *mut c_char, fileOut: *mut c_char) -> c_int;
    pub fn Dddmp_cuddBddLoad(
        ddMgr: *mut DdManager,
        varMatchMode: Dddmp_VarMatchType,
        varmatchnames: *mut *mut c_char,
        varmatchauxids: *mut c_int,
        varcomposeids: *mut c_int,
        mode: c_int,
        file: *mut c_char,
        fp: *mut FILE,
    ) -> *mut DdNode;
    pub fn Dddmp_cuddBddArrayLoad(
        ddMgr: *mut DdManager,
        rootMatchMode: Dddmp_RootMatchType,
        rootmatchnames: *mut *mut c_char,
        varMatchMode: Dddmp_VarMatchType,
        varmatchnames: *mut *mut c_char,
        varmatchauxids: *mut c_int,
        varcomposeids: *mut c_int,
        mode: c_int,
        file: *mut c_char,
        fp: *mut FILE,
        pproots: *mut *mut *mut DdNode,
    ) -> c_int;
    pub fn Dddmp_cuddAddLoad(
        ddMgr: *mut DdManager,
        varMatchMode: Dddmp_VarMatchType,
        varmatchnames: *mut *mut c_char,
        varmatchauxids: *mut c_int,
        varcomposeids: *mut c_int,
        mode: c_int,
        file: *mut c_char,
        fp: *mut FILE,
    ) -> *mut DdNode;
    pub fn Dddmp_cuddAddArrayLoad(
        ddMgr: *mut DdManager,
        rootMatchMode: Dddmp_RootMatchType,
        rootmatchnames: *mut *mut c_char,
        varMatchMode: Dddmp_VarMatchType,
        varmatchnames: *mut *mut c_char,
        varmatchauxids: *mut c_int,
        varcomposeids: *mut c_int,
        mode: c_int,
        file: *mut c_char,
        fp: *mut FILE,
        pproots: *mut *mut *mut DdNode,
    ) -> c_int;
    pub fn Dddmp_cuddHeaderLoad(
        ddType: *mut Dddmp_DecompType,
        nVars: *mut c_int,
        nsuppvars: *mut c_int,
        suppVarNames: *mut *mut *mut c_char,
        orderedVarNames: *mut *mut *mut c_char,
        varIds: *mut *mut c_int,
        composeIds: *mut *mut c_int,
        auxIds: *mut *mut c_int,
        nRoots: *mut c_int,
        file: *mut c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Dddmp_cuddBddLoadCnf(
        ddMgr: *mut DdManager,
        varmatchmode: Dddmp_VarMatchType,
        varmatchnames: *mut *mut c_char,
        varmatchauxids: *mut c_int,
        varcomposeids: *mut c_int,
        mode: c_int,
        file: *mut c_char,
        fp: *mut FILE,
        rootsPtrPtr: *mut *mut *mut DdNode,
        nRoots: *mut c_int,
    ) -> c_int;
    pub fn Dddmp_cuddBddArrayLoadCnf(
        ddMgr: *mut DdManager,
        rootmatchmode: Dddmp_RootMatchType,
        rootmatchnames: *mut *mut c_char,
        varmatchmode: Dddmp_VarMatchType,
        varmatchnames: *mut *mut c_char,
        varmatchauxids: *mut c_int,
        varcomposeids: *mut c_int,
        mode: c_int,
        file: *mut c_char,
        fp: *mut FILE,
        rootsPtrPtr: *mut *mut *mut DdNode,
        nRoots: *mut c_int,
    ) -> c_int;
    pub fn Dddmp_cuddHeaderLoadCnf(
        nVars: *mut c_int,
        nsuppvars: *mut c_int,
        suppVarNames: *mut *mut *mut c_char,
        orderedVarNames: *mut *mut *mut c_char,
        varIds: *mut *mut c_int,
        composeIds: *mut *mut c_int,
        auxIds: *mut *mut c_int,
        nRoots: *mut c_int,
        file: *mut c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Dddmp_cuddAddStore(
        ddMgr: *mut DdManager,
        ddname: *mut c_char,
        f: *mut DdNode,
        varnames: *mut *mut c_char,
        auxids: *mut c_int,
        mode: c_int,
        varinfo: Dddmp_VarInfoType,
        fname: *mut c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Dddmp_cuddAddArrayStore(
        ddMgr: *mut DdManager,
        ddname: *mut c_char,
        nRoots: c_int,
        f: *mut *mut DdNode,
        rootnames: *mut *mut c_char,
        varnames: *mut *mut c_char,
        auxids: *mut c_int,
        mode: c_int,
        varinfo: Dddmp_VarInfoType,
        fname: *mut c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Dddmp_cuddBddStore(
        ddMgr: *mut DdManager,
        ddname: *mut c_char,
        f: *mut DdNode,
        varnames: *mut *mut c_char,
        auxids: *mut c_int,
        mode: c_int,
        varinfo: Dddmp_VarInfoType,
        fname: *mut c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Dddmp_cuddBddArrayStore(
        ddMgr: *mut DdManager,
        ddname: *mut c_char,
        nRoots: c_int,
        f: *mut *mut DdNode,
        rootnames: *mut *mut c_char,
        varnames: *mut *mut c_char,
        auxids: *mut c_int,
        mode: c_int,
        varinfo: Dddmp_VarInfoType,
        fname: *mut c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Dddmp_cuddBddStoreCnf(
        ddMgr: *mut DdManager,
        f: *mut DdNode,
        mode: Dddmp_DecompCnfStoreType,
        noHeader: c_int,
        varNames: *mut *mut c_char,
        bddIds: *mut c_int,
        bddAuxIds: *mut c_int,
        cnfIds: *mut c_int,
        idInitial: c_int,
        edgeInTh: c_int,
        pathLengthTh: c_int,
        fname: *mut c_char,
        fp: *mut FILE,
        clauseNPtr: *mut c_int,
        varNewNPtr: *mut c_int,
    ) -> c_int;
    pub fn Dddmp_cuddBddArrayStoreCnf(
        ddMgr: *mut DdManager,
        f: *mut *mut DdNode,
        rootN: c_int,
        mode: Dddmp_DecompCnfStoreType,
        noHeader: c_int,
        varNames: *mut *mut c_char,
        bddIds: *mut c_int,
        bddAuxIds: *mut c_int,
        cnfIds: *mut c_int,
        idInitial: c_int,
        edgeInTh: c_int,
        pathLengthTh: c_int,
        fname: *mut c_char,
        fp: *mut FILE,
        clauseNPtr: *mut c_int,
        varNewNPtr: *mut c_int,
    ) -> c_int;
    pub fn Dddmp_cuddBddStorePrefix(
        ddMgr: *mut DdManager,
        nRoots: c_int,
        f: *mut DdNode,
        inputNames: *mut *mut c_char,
        outputNames: *mut *mut c_char,
        modelName: *mut c_char,
        fileName: *mut c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Dddmp_cuddBddArrayStorePrefix(
        ddMgr: *mut DdManager,
        nroots: c_int,
        f: *mut *mut DdNode,
        inputNames: *mut *mut c_char,
        outputNames: *mut *mut c_char,
        modelName: *mut c_char,
        fname: *mut c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Dddmp_cuddBddStoreBlif(
        ddMgr: *mut DdManager,
        nRoots: c_int,
        f: *mut DdNode,
        inputNames: *mut *mut c_char,
        outputNames: *mut *mut c_char,
        modelName: *mut c_char,
        fileName: *mut c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Dddmp_cuddBddArrayStoreBlif(
        ddMgr: *mut DdManager,
        nroots: c_int,
        f: *mut *mut DdNode,
        inputNames: *mut *mut c_char,
        outputNames: *mut *mut c_char,
        modelName: *mut c_char,
        fname: *mut c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Dddmp_cuddBddStoreSmv(
        ddMgr: *mut DdManager,
        nRoots: c_int,
        f: *mut DdNode,
        inputNames: *mut *mut c_char,
        outputNames: *mut *mut c_char,
        modelName: *mut c_char,
        fileName: *mut c_char,
        fp: *mut FILE,
    ) -> c_int;
    pub fn Dddmp_cuddBddArrayStoreSmv(
        ddMgr: *mut DdManager,
        nroots: c_int,
        f: *mut *mut DdNode,
        inputNames: *mut *mut c_char,
        outputNames: *mut *mut c_char,
        modelName: *mut c_char,
        fname: *mut c_char,
        fp: *mut FILE,
    ) -> c_int;
}
