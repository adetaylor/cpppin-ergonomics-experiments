use core::ops::Receiver;

/// Any C++ type which is "plain old data"/"trivial"
pub trait CppTrivial {}

impl CppTrivial for i32 {}

/// Anything which is, or can be converted to, a C++ reference
pub trait AsCppRef: core::ops::Receiver<Target = <Self as AsCppRef>::CppTarget> {
    type CppTarget: ?Sized;
    fn as_cpp_ref(&self) -> CppRef<Self::CppTarget>;
}

/// A prison newtype wrapper which ensures only C++-style references are
/// vended to the data inside. It can then be subject to C++'s lax aliasing
/// and mutability rules without risking UB in Rust.
pub struct CppPin<T>(T);

impl<T> CppPin<T> {
    /// Imprisons an object in a `CppPin` which guarantees no Rust references
    /// exist to the object, and then you're free to make C++ references to it,
    /// store references to it in C++ etc.
    pub fn new(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> AsCppRef for CppPin<T> {
    type CppTarget = T;
    fn as_cpp_ref(&self) -> CppRef<T> {
        unimplemented!()
    }
}

impl<T> Receiver for CppPin<T> {
    type Target = T;
}

/// A C++ reference or pointer. (Basically, a reference, except that we're
/// going to pretend that `this` is a reference not a pointer.)
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

/// A reference of some kind - whether a Rust reference or a C++ reference
pub trait Ref: core::ops::Receiver<Target = <Self as Ref>::Target> {
    type Target: ?Sized;
}

impl<T: ?Sized> Ref for CppRef<T> {
    type Target = T;
}

impl<T: ?Sized> Ref for &T {
    type Target = T;
}
