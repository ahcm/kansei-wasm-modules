#[no_mangle]
pub extern "C" fn fastx_version() -> i32
{
    1
}

#[no_mangle]
pub extern "C" fn fastx_add_i64(a: i64, b: i64) -> i64
{
    a + b
}

#[no_mangle]
pub extern "C" fn fastx_mul_f64(a: f64, b: f64) -> f64
{
    a * b
}

#[no_mangle]
pub extern "C" fn fastx_pow_f64(a: f64, b: f64) -> f64
{
    a.powf(b)
}
