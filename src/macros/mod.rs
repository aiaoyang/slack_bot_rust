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
        impl $trait_name for String {
            define_func! {$func_name, $prefix, $suffix}
        }
    };
}

#[macro_export]
macro_rules! define_func {
    ($name:ident, $prefix:expr, $suffix:expr) => {
        fn $name(&mut self) -> Self {
            *self = $prefix.to_string() + &self + $suffix;
            self.to_owned()
        }
    };
}
