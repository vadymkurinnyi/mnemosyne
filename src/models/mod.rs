pub mod project;
pub mod task;
pub mod user;
pub use project::*;
pub use task::*;
pub use user::*;

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
                            let field_value = std::string::ToString::to_string(&self.$field);
                            update.push_str(&format!("SET \"{}\" = '{}',", stringify!($field), field_value));
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
