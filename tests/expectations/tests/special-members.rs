#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default)]
pub struct A {
    pub _address: u8,
}
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
extern "C" {
    #[bindgen_original_name("A")]
    #[bindgen_special_member("default_ctor")]
    #[link_name = "\u{1}_ZN1AC1Ev"]
    pub fn A_A(this: *mut A);
}
extern "C" {
    #[bindgen_arg_type_reference(arg1)]
    #[bindgen_original_name("A")]
    #[bindgen_special_member("copy_ctor")]
    #[link_name = "\u{1}_ZN1AC1ERS_"]
    pub fn A_A1(this: *mut A, arg1: *mut A);
}
extern "C" {
    #[bindgen_arg_type_reference(arg1)]
    #[bindgen_original_name("A")]
    #[bindgen_special_member("move_ctor")]
    #[link_name = "\u{1}_ZN1AC1EOS_"]
    pub fn A_A2(this: *mut A, arg1: *mut A);
}
extern "C" {
    #[bindgen_original_name("A_destructor")]
    #[bindgen_special_member("dtor")]
    #[link_name = "\u{1}_ZN1AD1Ev"]
    pub fn A_A_destructor(this: *mut A);
}
impl A {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        A_A(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[bindgen_arg_type_reference(arg1)]
    #[inline]
    pub unsafe fn new1(arg1: *mut A) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        A_A1(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
    #[bindgen_arg_type_reference(arg1)]
    #[inline]
    pub unsafe fn new2(arg1: *mut A) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        A_A2(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        A_A_destructor(self)
    }
}
