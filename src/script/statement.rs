#[derive(Debug,Clone)]
pub enum Statement {
   Value(i64),
   Bytes(Box<[u8]>),
   Op(u8),
}

