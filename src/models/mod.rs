pub mod project;
pub mod task;
pub use project::*;
pub use task::*;

pub use crate::*;

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
        }
    };
}
