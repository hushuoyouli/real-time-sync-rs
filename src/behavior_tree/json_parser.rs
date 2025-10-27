use std::rc::Rc;
use super::interface::{IParser, TaskAddData,ITaskProxy};

pub struct JsonParser{

}

impl IParser for JsonParser{
    fn deserialize(&self, config:&Vec<u8>, task_add_data:&TaskAddData) -> Result<Rc<Box<dyn ITaskProxy>>, Box<dyn std::error::Error>>{
        Err("Not implemented".into())
    }
}