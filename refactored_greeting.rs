struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn new(name: &'a str) -> Person<'a> {
        Person { name: name }
    }
    fn greet(&self, your_name: &str) -> String {
        format!("Hello {}, my name is {}", your_name, self.name)
    }
}
