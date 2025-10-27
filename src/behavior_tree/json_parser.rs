use std::rc::Rc;
use serde_json::from_str;

use super::interface::{IParser, TaskAddData,ITaskProxy};
use super::runtime::TaskProxy;

pub struct JsonParser{

}

impl JsonParser{
    fn initialize_task(&self, task_json:&serde_json::Value, task_add_data:&TaskAddData) -> Result<Rc<Box<dyn ITaskProxy>>, Box<dyn std::error::Error>>{
        let correspondingType = task_json["Type"].as_str().unwrap();

        for (key, value) in task_json.as_object().unwrap().iter() {
            match key.as_str() {
                "Type"|"" => {
                    let correspondingType = value.as_str().unwrap();
                    value.clone();

                },
                _ => (),
            }
        }
        


        Err("initialize_task not implemented".into())
    }

}

impl IParser for JsonParser{
    fn deserialize(&self, config:&Vec<u8>, task_add_data:&TaskAddData) -> Result<Rc<Box<dyn ITaskProxy>>, Box<dyn std::error::Error>>{
        let json: serde_json::Value = from_str(std::str::from_utf8(config)?).unwrap();
        let root_task_json = json.get("RootTask").unwrap();

        Err("Not implemented".into())
    }
}