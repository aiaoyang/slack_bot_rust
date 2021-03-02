pub trait Context {
    fn to_string(&self) -> String;
    fn todo();
}
pub struct MyContext {
    s: String,
}
impl MyContext {
    pub fn from(s: String) -> Self {
        MyContext { s }
    }
}

impl Context for MyContext {
    fn to_string(&self) -> String {
        self.s.clone()
    }
    fn todo() {}
}
