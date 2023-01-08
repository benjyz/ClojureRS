use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use nom::lib::std::convert::TryFrom;
use std::rc::Rc;

use std::fs::File;
use std::io::Read;

use reqwest;
use url::Url;

use crate::error_message;

/// (slurp f & opts)
///
/// * Write a file
///
#[derive(Debug, Clone)]
pub struct SpitFn {}
impl ToValue for SpitFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}

impl IFn for SpitFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        println!("spit invoked");
        // if !args.is_empty() {
            
        // } else {
        //     Value::Nil
        // }

        Value::Nil
    }
}
