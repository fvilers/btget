use std::{borrow::Cow, collections::BTreeMap};

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Integer(isize),
    ByteString(&'a [u8]),
    List(Vec<Value<'a>>),
    Dictionary(BTreeMap<Cow<'a, str>, Value<'a>>),
}
