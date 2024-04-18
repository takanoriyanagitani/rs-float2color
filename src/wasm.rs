use core::ptr::{addr_of, addr_of_mut};

use crate::{float2rgba_simple, FloatToRgba, Rgba};

static mut INPUT: Vec<u8> = vec![];
static mut OUTPUT: Vec<u8> = vec![];

/// Gets the offset value of the INPUT buffer.
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn i_ptr() -> *mut u8 {
    let a: *mut Vec<u8> = unsafe { addr_of_mut!(INPUT) };
    let oi: Option<&mut Vec<u8>> = unsafe { a.as_mut() };
    oi.map(|v: &mut Vec<u8>| v.as_mut_ptr())
        .unwrap_or_else(std::ptr::null_mut)
}

/// Grows the INPUT buffer.
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn i_allocate(add_bytes_count: i32) -> i32 {
    let a: *mut Vec<u8> = unsafe { addr_of_mut!(INPUT) };
    let oi: Option<&mut Vec<u8>> = unsafe { a.as_mut() };
    oi.and_then(|v: &mut Vec<u8>| {
        let u: usize = add_bytes_count as usize;
        v.try_reserve(u)
            .ok()
            .and_then(|_| v.capacity().try_into().ok())
    })
    .unwrap_or(-1)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn o_allocate(add_bytes_count: i32) -> i32 {
    let a: *mut Vec<u8> = unsafe { addr_of_mut!(OUTPUT) };
    let om: Option<&mut Vec<u8>> = unsafe { a.as_mut() };
    om.and_then(|v: &mut Vec<u8>| {
        let u: usize = add_bytes_count as usize;
        v.try_reserve(u)
            .ok()
            .and_then(|_| v.capacity().try_into().ok())
    })
    .unwrap_or(-1)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn i_zero() -> i32 {
    let a: *mut Vec<u8> = unsafe { addr_of_mut!(INPUT) };
    let oi: Option<&mut Vec<u8>> = unsafe { a.as_mut() };
    oi.and_then(|v: &mut Vec<u8>| {
        v.clear();
        v.resize(v.capacity(), 0);
        v.capacity().try_into().ok()
    })
    .unwrap_or(-1)
}

/// Gets the offset value of the OUTPUT buffer.
#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn o_ptr() -> *mut u8 {
    let a: *mut Vec<u8> = unsafe { addr_of_mut!(OUTPUT) };
    let oi: Option<&mut Vec<u8>> = unsafe { a.as_mut() };
    oi.map(|v: &mut Vec<u8>| v.as_mut_ptr())
        .unwrap_or_else(std::ptr::null_mut)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn convert_simple(f: f32) -> u32 {
    let converted: Rgba = float2rgba_simple(f);
    converted.into()
}

pub fn convert_all<C, F, I>(
    converter: &C,
    bytes2float: F,
    integer2bytes: I,
    input: &[u8],
    output: &mut Vec<u8>,
) where
    C: FloatToRgba,
    F: Fn([u8; 4]) -> f32,
    I: Fn(u32) -> [u8; 4],
{
    output.clear();
    let chunks = input.chunks_exact(4);
    let floats = chunks.flat_map(|chunk: &[u8]| chunk.try_into().ok().map(&bytes2float));
    let rgbs = floats.map(|v: f32| converter.convert(v));
    let integers = rgbs.map(u32::from);
    let bytes = integers.map(integer2bytes);
    bytes.for_each(|a: [u8; 4]| {
        let s: &[u8] = &a;
        output.extend(s);
    });
}

pub fn try_convert_all<C, F, I>(
    converter: &C,
    bytes2float: F,
    integer2bytes: I,
) -> Result<usize, &'static str>
where
    C: FloatToRgba,
    F: Fn([u8; 4]) -> f32,
    I: Fn(u32) -> [u8; 4],
{
    #[allow(unsafe_code)]
    let ci: *const Vec<u8> = unsafe { addr_of!(INPUT) };
    #[allow(unsafe_code)]
    let oi: Option<&Vec<u8>> = unsafe { ci.as_ref() };
    let i: &Vec<u8> = oi.ok_or("invalid input")?;

    #[allow(unsafe_code)]
    let mi: *mut Vec<u8> = unsafe { addr_of_mut!(OUTPUT) };
    #[allow(unsafe_code)]
    let oo: Option<&mut Vec<u8>> = unsafe { mi.as_mut() };
    let o: &mut Vec<u8> = oo.ok_or("invalid output")?;

    convert_all(converter, bytes2float, integer2bytes, i, o);

    Ok(o.len())
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn convert_all_simple_le() -> i32 {
    try_convert_all(&float2rgba_simple, f32::from_le_bytes, |u: u32| {
        u.to_be_bytes()
    })
    .ok()
    .and_then(|u: usize| u.try_into().ok())
    .unwrap_or(-1)
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn convert_all_simple_be() -> i32 {
    try_convert_all(&float2rgba_simple, f32::from_be_bytes, |u: u32| {
        u.to_be_bytes()
    })
    .ok()
    .and_then(|u: usize| u.try_into().ok())
    .unwrap_or(-1)
}
