use core::ops::Receiver;

/// Marker trait for anything which is a C++ type
pub trait CppType {}

/// Any C++ type which is "plain old data"/"trivial"
pub trait CppTrivial: CppType {}

impl CppType for i32 {}
impl CppTrivial for i32 {}

/// Anything which is, or can be converted to, a C++ reference
pub trait AsCppRef: core::ops::Receiver<Target = <Self as AsCppRef>::CppTarget> {
    type CppTarget: ?Sized + CppType;
    fn as_cpp_ref(&self) -> CppRef<Self::CppTarget>;
}

pub struct CppPin<T: ?Sized + CppType>(T);

impl<T: ?Sized + CppType> AsCppRef for CppPin<T> {
    type CppTarget = T;
    fn as_cpp_ref(&self) -> CppRef<T> {
        unimplemented!()
    }
}

impl<T: ?Sized + CppType> Receiver for CppPin<T> {
    type Target = T;
}

pub struct CppRef<T: ?Sized + CppType>(*const T);

impl<T: ?Sized + CppType> Clone for CppRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized + CppType> Copy for CppRef<T> {}

impl<T: ?Sized + CppType> Receiver for CppRef<T> {
    type Target = T;
}

impl<T: ?Sized + CppType> AsCppRef for CppRef<T> {
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
