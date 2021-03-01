#[macro_export]
macro_rules! define_trait {
    ($($name:tt => $fn:item),* $(,)*) => {
        $(
            pub trait $name {
                $fn
            }
        )*
    };
}

#[macro_export]
macro_rules! define_impl {
    ($trait_name:tt, $func_name:ident, $prefix:expr, $suffix:expr) => {
        impl $trait_name for Vec<String> {
            define_func! {$func_name, $prefix, $suffix}
        }
    };
}

#[macro_export]
macro_rules! define_func {
    ($name:ident, $prefix:expr, $suffix:expr) => {
        fn $name(&mut self) {
            *self = self
                .into_iter()
                .map(|content| $prefix.to_string() + content + $suffix)
                .collect::<Vec<String>>();
        }
    };
}
