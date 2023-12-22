#![feature(receiver_trait)]
#![feature(concat_idents)]

use cpppin::{AsCppRef, CppPin, CppRef, Ref};
use generated::{OtherCppClass, SomeCppClass};

#[macro_use]
mod cpppin;
mod generated;

/// Here's a struct which may be used by Rust or by C++.
/// If it's used by C++, it must be imprisoned in a CppPin, such that
/// there are guaranteed to be no Rust references.
struct MayBeUsedByCppOrByRust(u32);

impl MayBeUsedByCppOrByRust {
    /// This method can accept either &Self or CppRef<Self>
    fn some_method(self: impl Ref<Target = Self>) -> u32 {
        // Of course, given that `self` is an `impl Ref` it's
        // hard to do anything with it.
        // We could imagine abstractions which could do things
        // like the following...
        //   get_field_from_ref!(self, 0)
        // which would work for either kind of reference by
        // using the underlying raw pointer,
        unimplemented!()
    }

    fn some_method_only_callable_when_object_in_rust_domain(&self) {}
    fn some_method_only_callable_when_object_in_cpp_domain(self: CppRef<Self>) {}
}

fn main() {
    // Obtain a SomeCppClass somehow, by value. This is kept in a CppPin
    // to ensure we can't create Rust references to it.
    let some_cpp_class: CppPin<SomeCppClass> = SomeCppClass::new();

    // We want to call methods on it. We can call methods directly on it
    // but this will consume the CppPin<T>, so for multiple calls we need
    // to get a CppRef out of it. This is a little analogous to Pin::as_mut
    // which is equally annoying.
    // Better alternatives here would be:
    // * If the "autoref" done by the method probing algorithm didn't
    //   automatically create a & reference, but instead called some trait
    //   which could be overridden. Pin<&mut T> could create another Pin<&mut T>
    //   and CppPin<T> could create a CppRef<T>.
    // * We could implement the `Receiver` trait for `&CppPin<T>`. We can't
    //   do that because it conflicts with the implementation of `Receiver` for
    //   `&T`.
    // So for now, let's create a reference.
    let some_cpp_class_ref = some_cpp_class.as_cpp_ref();
    // Once we've got a CppRef, it's Copy so we can call multiple methods on it.
    let some_val: i32 = some_cpp_class_ref.some_primitive_method();
    let some_val2: i32 = some_cpp_class_ref.some_primitive_method();
    // We also interact with fields using method calls so that the actual
    // accesses can be done in generated C++ code.
    // Ideally we'd have a macro to make this nicer.
    let some_other_val: i32 = some_cpp_class_ref.FIELD_primitive_field().get();

    // Other methods or fields may return classes by value or by reference.
    let some_other_cpp_class_by_value: CppPin<OtherCppClass> = some_cpp_class_ref.get_by_value();
    let yet_another_val: i32 = some_other_cpp_class_by_value.other_primitive_method();
    let yet_another_val2: i32 = some_cpp_class.get_by_reference().other_primitive_method();

    // Now we'll experiment with a type which may be kept either in the Rust
    // domain (with &T, &mut T etc.) or in the C++ domain (with CppRef<T>).
    let rust_thingy = MayBeUsedByCppOrByRust(1);
    let cpp_thingy = CppPin::new(MayBeUsedByCppOrByRust(2));
    // The same method can be called on either, like this.
    // Annoyingly it seems the autoreffing doesn't work
    // so we have to explicitly take a reference:
    (&rust_thingy).some_method();
    cpp_thingy.as_cpp_ref().some_method();
}
