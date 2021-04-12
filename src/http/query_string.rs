use std::collections::HashMap;
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}
// ex. a=1&b=1&c&d=&e===&d=7&d=abc
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// from_str wouldn't work because it doesn't work with lifetimes
// we should always be able to convert a string to a querystring so From not TryFrom
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            // case 1: there is only a key and no "="
            let mut key = sub_str;
            let mut val = "";
            // case 2: take the value after "="
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }
            // check if the key already exists in the hashmap
            // push if it's multiple, or make it multiple if it's single
            // if it doesn't exist in data, make a new single
            data.entry(key)
                .and_modify(|existing| match existing {
                    Value::Single(prev_val) => *existing = Value::Multiple(vec![prev_val, val]),
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }
        // add the return early so that the compiler can infer the hashmap types
        QueryString { data }
    }
}
