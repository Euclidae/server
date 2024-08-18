use std::convert::TryFrom;
use super::method::Method;

 pub struct Request {
        pub path: String,
        pub query_string: Option<String>,
        pub method: Method,
}

impl Request{
    fn from_byte_array(buf: &[u8]) -> Result<Self,String>{
        let string = String::from("asd");
        string.encrypt();
        buf.encrypt();
        unimplemented!();
    }
}

impl TryFrom<&[u8]> for Request{
    type Error = String;
    fn try_from(buf: &[u8])-> Result<Self,Self::Error>{
        unimplemented!();
    }
}

trait Encrypt{
    fn encrypt(&self)->Self;
}

impl Encrypt for String{
    fn encrypt(&self)->Self {
        unimplemented!();
    }
}

impl Encrypt for &[u8]{
    fn encrypt(&self)->Self {
        unimplemented!();
    }
}