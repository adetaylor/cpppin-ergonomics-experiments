use crate::cpppin::{AsCppRef, CppPin, CppRef};

/// Generated from:
/// ```cpp
/// class OtherCppClass {
/// public:
///   OtherCppClass();
///   int other_primitive_method();
/// };
///
/// class SomeCppClass {
/// public:
///   SomeCppClass();
///   int some_primitive_method();
///   OtherCppClass& get_by_reference();
///   OtherCppClass get_by_value();
///   OtherCppClass field_by_value;
///   int primitive_field;
/// };
/// ```

pub struct SomeCppClass {
    // In practice would have size and alignment to match C++ so we can
    // keep this on the stack
}

impl SomeCppClass {
    pub fn new() -> CppPin<Self> {
        unimplemented!()
    }

    pub fn FIELD_primitive_field(self: impl AsCppRef<CppTarget = Self>) -> CppRef<i32> {
        unimplemented!()
    }

    pub fn FIELD_field_by_value(self: impl AsCppRef<CppTarget = Self>) -> CppRef<OtherCppClass> {
        unimplemented!()
    }

    pub fn some_primitive_method(self: impl AsCppRef<CppTarget = Self>) -> i32 {
        unimplemented!()
    }

    pub fn get_by_value(self: impl AsCppRef<CppTarget = Self>) -> CppPin<OtherCppClass> {
        unimplemented!()
    }

    pub fn get_by_reference(self: impl AsCppRef<CppTarget = Self>) -> CppRef<OtherCppClass> {
        unimplemented!()
    }
}

pub struct OtherCppClass {
    // In practice would have size and alignment to match C++ so we can
    // keep this on the stack
}

impl OtherCppClass {
    pub fn other_primitive_method(self: impl AsCppRef<CppTarget = Self>) -> i32 {
        unimplemented!()
    }
}
