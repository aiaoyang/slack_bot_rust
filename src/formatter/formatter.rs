macro_rules! define_trait {
    ($($name:tt => $fn:item),* $(,)*) => {
        $(
            pub trait $name {
                $fn
            }
        )*
    };
}

macro_rules! define_impl {
    ($trait_name:tt, $func_name:ident, $prefix:expr, $suffix:expr) => {
        impl $trait_name for Vec<String> {
            define_func! {$func_name, $prefix, $suffix}
        }
    };
}

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

define_trait! {
    Bold  => fn bold(&mut self);,
    Code  => fn code(&mut self);,
    Enter => fn enter(&mut self);,
    Line  => fn line(&mut self);,
    Link  => fn link(&mut self);,
    Tab   => fn tab(&mut self);,
    ToString => fn to_string(&self) -> String;,
}

define_impl!(Bold, bold, "#", "");
define_impl!(Code, code, "`", "`");
define_impl!(Enter, enter, "", "\n");
define_impl!(Line, line, "\n>", "");
define_impl!(Link, link, "<", ">");
define_impl!(Tab, tab, "", "\t");

macro_rules! test_func {
    ($source:expr, $expect:expr, $func_name:ident) => {
        let mut src: Vec<String> = vec![$source];
        src.$func_name();
        if let Some(v) = src.get(0) {
            assert_eq!($expect, v.clone());
        };
    };
}

#[test]
fn test() {
    test_func!("result".to_string(), "\n>result".to_string(), line);
    test_func!("result".to_string(), "#result".to_string(), bold);
    test_func!("result".to_string(), "`result`".to_string(), code);
    test_func!("result".to_string(), "<result>".to_string(), link);
    test_func!("result".to_string(), "result\n".to_string(), enter);
    test_func!("result".to_string(), "result\t".to_string(), tab);
}
