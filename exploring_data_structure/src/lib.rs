#[cfg(test)]
mod test {

    use proc_macro2::TokenStream;
    use quote::quote;

    #[test]
    fn dump_syn_structure() {
        let source: TokenStream = quote! {
            // ======= Attributes & Visibility =======
            //! This is a doc comment
            #![allow(dead_code)]
            #[derive(Debug, Clone, Copy)]
            #[repr(C)]
            pub struct Foo<'a, T: Clone + Default>
            where
                T: core::fmt::Debug,
            {
                // ======= Fields =======
                pub id: u32,
                pub name: &'a str,
                value: Option<T>,
                #[cfg(feature = "extra")]
                extra: u8,
            }

            // ======= Enum with Variants =======
            #[derive(Debug, PartialEq)]
            enum MyEnum<T> {
                Unit,
                Tuple(u32, String),
                Struct {
                    field1: T,
                    field2: Option<u8>,
                },
            }

            // ======= Type Alias =======
            type MyAlias<'a> = &'a str;

            // ======= Const and Static =======
            const CONST_VAL: usize = 42;
            static mut GLOBAL_COUNTER: i32 = 0;

            // ======= Function =======
            fn compute<'a, T: Copy + core::fmt::Debug>(x: &'a T, y: T) -> T
            where
                T: core::ops::Add<Output = T>,
            {
                println!("compute: {:?}", x);
                y
            }

            // ======= Impl block =======
            impl<'a, T> Foo<'a, T>
            where
                T: Clone + core::fmt::Debug,
            {
                pub const VERSION: &'static str = "1.0";

                pub fn new(name: &'a str, id: u32, value: T) -> Self {
                    Self { id, name, value: Some(value) }
                }

                pub fn id(&self) -> u32 {
                    self.id
                }

                pub fn greet(&self) {
                    println!("Hello from {}", self.name);
                }
            }

            // ======= Trait =======
            pub trait Greeter {
                fn greet(&self);
                fn greet_named(&self, name: &str) -> String {
                    format!("Hello, {}", name)
                }
            }

            // ======= Trait Implementation =======
            impl<'a, T> Greeter for Foo<'a, T> {
                fn greet(&self) {
                    println!("Greetings from Foo: {}", self.name);
                }
            }

            // ======= Generic Function + Lifetimes =======
            fn generic_func<'a, 'b: 'a, T: Clone>(a: &'a T, b: &'b T) -> &'a T {
                if true { a } else { b }
            }

            // ======= Macro invocation =======
            println!("Macro call with value: {}", 123);

            // ======= Nested modules =======
            pub mod inner {
                use super::*;

                pub struct InnerStruct {
                    pub x: i32,
                    pub y: i32,
                }

                impl InnerStruct {
                    pub fn sum(&self) -> i32 {
                        self.x + self.y
                    }
                }
            }

            // ======= Const generics example =======
            struct ArrayHolder<T, const N: usize> {
                data: [T; N],
            }

            // ======= Enum with discriminants =======
            enum State {
                Idle = 0,
                Running = 1,
                Error(i32),
            }

            // ======= Match Expression & Control Flow =======
            fn handle_state(s: State) {
                match s {
                    State::Idle => println!("Idle"),
                    State::Running => println!("Running"),
                    State::Error(code) if code < 0 => println!("Negative error"),
                    _ => println!("Other"),
                }
            }

            // ======= Unsafe Block & Raw Pointers =======
            unsafe fn modify_global() {
                GLOBAL_COUNTER += 1;
            }

            // ======= Impl for generic type =======
            impl<T> core::fmt::Display for MyEnum<T> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "MyEnum")
                }
            }
        };

        let parsed: syn::File = syn::parse2(source).unwrap();
        println!("{:#?}", parsed);
    }
}
