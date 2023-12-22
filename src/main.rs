#![feature(receiver_trait)]
#![feature(concat_idents)]

use cpppin::{AsCppRef, CppPin};
use generated::{OtherCppClass, SomeCppClass};

#[macro_use]
mod cpppin;
mod generated;

fn main() {
    // Obtain a
    let some_cpp_class: CppPin<SomeCppClass> = SomeCppClass::new();
    let some_cpp_class_ref = some_cpp_class.as_cpp_ref();
    let some_val: i32 = some_cpp_class_ref.some_primitive_method();
    let some_other_val: i32 = some_cpp_class_ref.FIELD_primitive_field().get();

    let some_other_cpp_class_by_value: CppPin<OtherCppClass> = some_cpp_class_ref.get_by_value();
    let yet_another_val: i32 = some_other_cpp_class_by_value.other_primitive_method();
    let yet_another_val2: i32 = some_cpp_class.get_by_reference().other_primitive_method();
}
