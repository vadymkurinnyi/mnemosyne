pub mod project;
pub mod task;
pub use project::*;
pub use task::*;

pub use crate::*;

#[macro_export]
macro_rules! make_qotes {
    ($field:ident, $typ:ty) => {
        match <$typ>::default() {
            _ if std::mem::size_of::<$typ>() == std::mem::size_of::<bool>() => {
                format!("{}", self.$field)
            }
            _ => format!("'{}'", self.$field),
        }
        quote!();
    };
}

#[macro_export]
macro_rules! generate_update {
    ($struct:ident { $( $field:ident : $typ:ty),* }) => {
        impl $struct {
            pub fn get_update(&self, old: Self) -> Option<String> {
                let mut update = String::new();
                $(
                        if self.$field != old.$field {
                            let field_value = match <$typ>::default() {
                                _ if std::mem::size_of::<$typ>() == std::mem::size_of::<bool>() => format!("{}", self.$field),
                                _ => format!("'{}'", self.$field)
                            };

//                            let type_name = std::any::type_name::<i32>();
//                            if std::any::type_name::<i32>() == std::any::type_name::<$typ>() {
//                                println!("x is an i32");
//                            } else {
//                                println!("x is not an i32");
//                            }

                            update.push_str(&format!("SET \"{}\" = {},", stringify!($field), field_value));
                        }
                )*
                if update.is_empty(){
                    None
                }else{
                    update.pop();
                    Some(update)
                }
            }
            pub fn get_force_update(&self) -> Option<String> {
                let mut update = String::new();
                $(
                            let field_value = match <$typ>::default() {
                                _ if std::mem::size_of::<$typ>() == std::mem::size_of::<bool>() => format!("{}", self.$field),
                                _ => format!("'{}'", self.$field)
                            };

                            update.push_str(&format!("SET \"{}\" = {},", stringify!($field), field_value));
                )*
                if update.is_empty(){
                    None
                }else{
                    update.pop();
                    Some(update)
                }
            }
        }
    };
}

//macro_rules! create_function {
//    // This macro takes an argument of designator `ident` and
//    // creates a function named `$func_name`.
//    // The `ident` designator is used for variable/function names.
//    ($func_name:ident) => {
//        fn $func_name() {
//            // The `stringify!` macro converts an `ident` into a string.
//            println!("You called {:?}()",
//                     stringify!($func_name));
//        }
//    };
//}
