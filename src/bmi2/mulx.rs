use int::Int;
use alg;

pub trait MULX {
    fn mulx(self, Self, &mut Self) -> Self;
}

impl<T: Int + alg::bmi2::MULX> MULX for T {
    default fn mulx(self, y: Self, hi: &mut Self) -> Self {
        alg::bmi2::mulx(self, y, hi)
    }
}

/// Unsigned multiply without affecting flags.
///
/// Unsigned multiplication of `x` with `y` storing the high half of the result
/// in `p` and returning the low half of the result.
///
/// # Intrinsics (when available BMI2)
///
/// [`MULX`](http://www.felixcloutier.com/x86/MULX.html): Unsigned Multiply
/// Without Affecting Flags (supports 32/64 bit registers).
///
/// # Examples
///
/// ```
/// use bitintr::bmi2::mulx;
///
/// { // 8-bit
///   let a: u8 = 128;
///   let b: u8 = 128;
///   let mut hi: u8 = 0;
///   let lo: u8 = mulx(a, b, &mut hi);
///   // result = 16384 = 0b0100_0000_0000_0000u16
///   //                    ^~hi~~~~~ ^~lo~~~~~
///   assert_eq!(lo, 0b0000_0000);
///   assert_eq!(hi, 0b0100_0000);
/// }
/// { // 16-bit
///   let a: u16 = 65_500;
///   let b: u16 = 65_500;
///   let mut hi: u16 = 0;
///   let lo: u16 = mulx(a, b, &mut hi);
///   // result = 4290250000 = 0b0b1111_1111_1011_1000_0000_0101_0001_0000u32
///   //                           ^~hi~~~~~~~~~~~~~~~ ^~lo~~~~~~~~~~~~~~~
///   assert_eq!(lo, 0b0000_0101_0001_0000);
///   assert_eq!(hi, 0b1111_1111_1011_1000);
/// }
/// { // 32-bit
///   let a: u32 = 4_294_967_200;
///   let b: u32 = 2;
///   let mut hi: u32 = 0;
///   let lo: u32 = mulx(a, b, &mut hi);
///   // result = 8589934400
///   //        = 0b0b0001_1111_1111_1111_1111_1111_1111_0100_0000u64
///   //              ^~hi ^~lo~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///   assert_eq!(lo, 0b1111_1111_1111_1111_1111_1111_0100_0000u32);
///   assert_eq!(hi, 0b0001u32);
/// }
/// { // 64-bit
///   let a: u64 = 9_223_372_036_854_775_800;
///   let b: u64 = 100;
///   let mut hi: u64 = 0;
///   let lo: u64 = mulx(a, b, &mut hi);
///   // result = 922337203685477580000
///   //        = 0b0b00110001_11111111_11111111_11111111_11111111_11111111_11111111_11111100_11100000u128
///   //         ^~hi~~~~~~~~~ ^~lo~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///   assert_eq!(lo, 0b11111111_11111111_11111111_11111111_11111111_11111111_11111100_11100000u64);
///   assert_eq!(hi, 0b00110001u64);
/// }
/// ```
pub fn mulx<T: MULX + alg::bmi2::MULX>(x: T, y: T, hi: &mut T) -> T {
    MULX::mulx(x, y, hi)
}
