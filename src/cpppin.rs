use core::ops::Receiver;

/// Any C++ type which is "plain old data"/"trivial"
pub trait CppTrivial {}

impl CppTrivial for i32 {}

/// Anything which is, or can be converted to, a C++ reference
pub trait AsCppRef: core::ops::Receiver<Target = <Self as AsCppRef>::CppTarget> {
    type CppTarget: ?Sized;
    fn as_cpp_ref(&self) -> CppRef<Self::CppTarget>;
}

pub struct CppPin<T: ?Sized>(T);

impl<T: ?Sized> AsCppRef for CppPin<T> {
    type CppTarget = T;
    fn as_cpp_ref(&self) -> CppRef<T> {
        unimplemented!()
    }
}

impl<T: ?Sized> Receiver for CppPin<T> {
    type Target = T;
}

pub struct CppRef<T: ?Sized>(*const T);

impl<T: ?Sized> Clone for CppRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> Copy for CppRef<T> {}

impl<T: ?Sized> Receiver for CppRef<T> {
    type Target = T;
}

impl<T: ?Sized> AsCppRef for CppRef<T> {
    type CppTarget = T;
    fn as_cpp_ref(&self) -> CppRef<T> {
        self.clone()
    }
}

impl<T: CppTrivial> CppRef<T> {
    pub fn set(&mut self, value: T) {
        unimplemented!()
    }

    pub fn get(&self) -> T {
        unimplemented!()
    }
}

/// Macro to access a field of a [`CppPin`] or [`CppRef`]
macro_rules! cpp_field {
    ($type:expr, $field_name:ident) => {
        $type.concat_idents!(FIELD_,$field_name)
    };
}
