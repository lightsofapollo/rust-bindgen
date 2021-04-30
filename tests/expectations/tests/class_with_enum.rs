#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub _address: u8,
}
pub const A_B_B1: A_B = 0;
pub const A_B_B2: A_B = 1;
#[bindgen_original_name("A::B")]
pub type A_B = ::std::os::raw::c_uint;
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(
        ::std::mem::size_of::<A>(),
        1usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        1usize,
        concat!("Alignment of ", stringify!(A))
    );
}
