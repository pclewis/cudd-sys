use libc::{c_int, c_void};
use std::os::raw::{c_char, c_double};
use EpDouble;

extern "C" {
    pub fn EpdAlloc() -> *mut EpDouble;
    pub fn EpdCmp(key1: *const c_void, key2: *const c_void) -> c_int;
    pub fn EpdFree(epd: *mut EpDouble) -> c_void;
    pub fn EpdGetString(epd: *const EpDouble, str: *mut c_char) -> c_void;
    pub fn EpdConvert(value: c_double, epd: *mut EpDouble) -> c_void;
    pub fn EpdMultiply(epd1: *mut EpDouble, value: c_double) -> c_void;
    pub fn EpdMultiply2(epd1: *mut EpDouble, epd2: *const EpDouble) -> c_void;
    pub fn EpdMultiply2Decimal(epd1: *mut EpDouble, epd2: *const EpDouble) -> c_void;
    pub fn EpdMultiply3(
        epd1: *const EpDouble,
        epd2: *const EpDouble,
        epd3: *mut EpDouble,
    ) -> c_void;
    pub fn EpdMultiply3Decimal(
        epd1: *const EpDouble,
        epd2: *const EpDouble,
        epd3: *mut EpDouble,
    ) -> c_void;
    pub fn EpdDivide(epd1: *mut EpDouble, value: c_double) -> c_void;
    pub fn EpdDivide2(epd1: *mut EpDouble, epd2: *const EpDouble) -> c_void;
    pub fn EpdDivide3(epd1: *const EpDouble, epd2: *const EpDouble, epd3: *mut EpDouble) -> c_void;
    pub fn EpdAdd(epd1: *mut EpDouble, value: c_double) -> c_void;
    pub fn EpdAdd2(epd1: *mut EpDouble, epd2: *const EpDouble) -> c_void;
    pub fn EpdAdd3(epd1: *const EpDouble, epd2: *const EpDouble, epd3: *mut EpDouble) -> c_void;
    pub fn EpdSubtract(epd1: *mut EpDouble, value: c_double) -> c_void;
    pub fn EpdSubtract2(epd1: *mut EpDouble, epd2: *const EpDouble) -> c_void;
    pub fn EpdSubtract3(
        epd1: *const EpDouble,
        epd2: *const EpDouble,
        epd3: *mut EpDouble,
    ) -> c_void;
    pub fn EpdPow2(n: c_int, epd: *mut EpDouble) -> c_void;
    pub fn EpdPow2Decimal(n: c_int, epd: *mut EpDouble) -> c_void;
    pub fn EpdNormalize(epd: *mut EpDouble) -> c_void;
    pub fn EpdNormalizeDecimal(epd: *mut EpDouble) -> c_void;
    pub fn EpdGetValueAndDecimalExponent(
        epd: *const EpDouble,
        value: *mut c_double,
        exponent: *mut c_int,
    ) -> c_void;
    pub fn EpdGetExponent(value: c_double) -> c_int;
    pub fn EpdGetExponentDecimal(value: c_double) -> c_int;
    pub fn EpdMakeInf(epd: *mut EpDouble, sign: c_int) -> c_void;
    pub fn EpdMakeZero(epd: *mut EpDouble, sign: c_int) -> c_void;
    pub fn EpdMakeNan(epd: *mut EpDouble) -> c_void;
    pub fn EpdCopy(from: *const EpDouble, to: *mut EpDouble) -> c_void;
    pub fn EpdIsInf(epd: *const EpDouble) -> c_int;
    pub fn EpdIsZero(epd: *const EpDouble) -> c_int;
    pub fn EpdIsNan(epd: *const EpDouble) -> c_int;
    pub fn EpdIsNanOrInf(epd: *const EpDouble) -> c_int;
    pub fn IsInfDouble(value: c_double) -> c_int;
    pub fn IsNanDouble(value: c_double) -> c_int;
    pub fn IsNanOrInfDouble(value: c_double) -> c_int;
}
