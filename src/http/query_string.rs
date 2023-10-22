use std::collections::HashMap;

// p=1&b=2&c&d=&e=&d=7&d=abc

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&str, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}
