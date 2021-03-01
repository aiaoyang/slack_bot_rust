use crate::{define_func, define_impl, define_trait};
define_trait! {
    Bold  => fn bold(&mut self)->Self;,
    Code  => fn code(&mut self)->Self;,
    Enter => fn enter(&mut self)->Self;,
    Line  => fn line(&mut self)->Self;,
    Link  => fn link(&mut self)->Self;,
    Tab   => fn tab(&mut self)->Self;,
    ToString => fn to_string(&self)->String;,
}

define_impl!(Bold, bold, "#", "");
define_impl!(Code, code, "`", "`");
define_impl!(Enter, enter, "", "\n");
define_impl!(Line, line, "\n>", "");
define_impl!(Link, link, "<", ">");
define_impl!(Tab, tab, "", "\t");

impl ToString for Vec<String> {
    fn to_string(&self) -> String {
        self.iter().fold(String::new(), |acc, v| acc + v)
    }
}

macro_rules! test_func {
    ($source:expr, $expect:expr, $func_name:ident) => {
        let mut src: String = $source;
        src.$func_name();
        assert_eq!($expect, src.clone());
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

    let v: Vec<String> = vec!["123".into(), "456".into()];
    assert_eq!("123456".to_string(), v.to_string());
}
