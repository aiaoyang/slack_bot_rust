pub trait Context {
    fn to_string(&self) -> String;
}
pub struct MyContext {
    pub s: String,
}
impl MyContext {
    fn from(s: String) -> Self {
        MyContext { s }
    }
}

impl Context for MyContext {
    fn to_string(&self) -> String {
        self.s.clone()
    }
}
