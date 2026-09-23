pub fn Test_QuickCheck_Gen_float32ToInt32(number: f64) -> i64 {
    // Reinterpret the float32 bit pattern as a signed 32-bit integer, like the
    // DataView-based JS implementation.
    (number as f32).to_bits() as i32 as i64
}
