/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[derive(Copy, Debug)]
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Struct_NastyStruct<T> {
    pub mIsSome: bool,
    pub mStorage: Union_NastyStruct_union_template_hpp_unnamed_1<T>,
    pub NastyStruct_union_template_hpp_unnamed_2: Union_NastyStruct_union_template_hpp_unnamed_2<T>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Union_NastyStruct_union_template_hpp_unnamed_1<T> {
    pub mFoo: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub mDummy: __BindgenUnionField<::std::os::raw::c_ulong>,
    pub _bindgen_data_: u64,
    pub _phantom0: ::std::marker::PhantomData<T>,
}
impl <T> Union_NastyStruct_union_template_hpp_unnamed_1<T> {
    pub unsafe fn mFoo(&mut self) -> *mut *mut ::std::os::raw::c_void {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn mDummy(&mut self) -> *mut ::std::os::raw::c_ulong {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Union_NastyStruct_union_template_hpp_unnamed_2<T> {
    pub wat: __BindgenUnionField<::std::os::raw::c_short>,
    pub wut: __BindgenUnionField<*mut ::std::os::raw::c_int>,
    pub _bindgen_data_: u64,
    pub _phantom0: ::std::marker::PhantomData<T>,
}
impl <T> Union_NastyStruct_union_template_hpp_unnamed_2<T> {
    pub unsafe fn wat(&mut self) -> *mut ::std::os::raw::c_short {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn wut(&mut self) -> *mut *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Union_Whatever<T> {
    pub mTPtr: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub mInt: __BindgenUnionField<::std::os::raw::c_int>,
    pub _bindgen_data_: u64,
    pub _phantom0: ::std::marker::PhantomData<T>,
}
impl <T> Union_Whatever<T> {
    pub unsafe fn mTPtr(&mut self) -> *mut *mut ::std::os::raw::c_void {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn mInt(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
