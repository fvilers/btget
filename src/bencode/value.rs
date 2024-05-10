#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Integer(isize),
    ByteString(&'a [u8]),
    List(Vec<Value<'a>>),
}
