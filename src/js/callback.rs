use crate::error::Result;


pub struct Callback<'a> {
    pub name: String,
    pub function: Box<dyn Fn() -> Result<()> + Send + Sync + 'a>,
}


impl std::fmt::Debug for Callback<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Callback {{ name: {}, function: <function> }}", self.name)
    }
}

impl<'a> Callback<'a> {
    pub fn new(name: impl Into<String>, function: impl Fn() -> Result<()> + Send + Sync + 'a) -> Self {
        Self {
            name: name.into(),
            function: Box::new(function),
        }
    }
}

