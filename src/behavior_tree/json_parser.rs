use std::rc::{Rc, Weak};
use serde_json::from_str;
use std::collections::HashMap;

use super::interface::{IParser, TaskAddData,ITaskProxy, IAction, IConditional, IComposite, IDecorator, RealTaskType};
use super::runtime::TaskProxy;

pub struct JsonParser{
    action_fn: HashMap<String, fn(variables:HashMap<String, serde_json::Value>,id_2_task:HashMap<u32, Weak<Box<dyn ITaskProxy>>>) -> Box<dyn IAction>>,
    conditional_fn: HashMap<String, fn(variables:HashMap<String, serde_json::Value>,id_2_task:HashMap<u32, Weak<Box<dyn ITaskProxy>>>) -> Box<dyn IConditional>>,
    composite_fn: HashMap<String, fn(variables:HashMap<String, serde_json::Value>,id_2_task:HashMap<u32, Weak<Box<dyn ITaskProxy>>>) -> Box<dyn IComposite>>,
    decorator_fn: HashMap<String, fn(variables:HashMap<String, serde_json::Value>,id_2_task:HashMap<u32, Weak<Box<dyn ITaskProxy>>>) -> Box<dyn IDecorator>>,
}

impl JsonParser{
    pub fn new() -> Self{
        Self{
            action_fn: HashMap::new(),
            conditional_fn: HashMap::new(),
            composite_fn: HashMap::new(),
            decorator_fn: HashMap::new(),
        }
    }

    pub fn register_action_fn(&mut self, name:&str, action_generate_fn:fn(variables:HashMap<String, serde_json::Value>,id_2_task:HashMap<u32, Weak<Box<dyn ITaskProxy>>>) -> Box<dyn IAction>){
        self.action_fn.insert(name.to_string(), action_generate_fn);
    }

    pub fn register_conditional_fn(&mut self, name:&str, conditional_generate_fn:fn(variables:HashMap<String, serde_json::Value>,id_2_task:HashMap<u32, Weak<Box<dyn ITaskProxy>>>) -> Box<dyn IConditional>){
        self.conditional_fn.insert(name.to_string(), conditional_generate_fn);
    }

    pub fn register_composite_fn(&mut self, name:&str, composite_generate_fn:fn(variables:HashMap<String, serde_json::Value>,id_2_task:HashMap<u32, Weak<Box<dyn ITaskProxy>>>) -> Box<dyn IComposite>){
        self.composite_fn.insert(name.to_string(), composite_generate_fn);
    }

    pub fn register_decorator_fn(&mut self, name:&str, decorator_generate_fn:fn(variables:HashMap<String, serde_json::Value>,id_2_task:HashMap<u32, Weak<Box<dyn ITaskProxy>>>) -> Box<dyn IDecorator>){
        self.decorator_fn.insert(name.to_string(), decorator_generate_fn);
    }


    fn generate_real_task(&self, corresponding_type:&str, variables:HashMap<String, serde_json::Value>,id_2_task:HashMap<u32, Weak<Box<dyn ITaskProxy>>>) -> Result<RealTaskType, Box<dyn std::error::Error>>{
        if self.action_fn.contains_key(corresponding_type){
            return Ok(RealTaskType::Action(self.action_fn.get(corresponding_type).unwrap()(variables, id_2_task)));
        }
        if self.conditional_fn.contains_key(corresponding_type){
            return Ok(RealTaskType::Conditional(self.conditional_fn.get(corresponding_type).unwrap()(variables, id_2_task)));
        }
        if self.composite_fn.contains_key(corresponding_type){
            return Ok(RealTaskType::Composite(self.composite_fn.get(corresponding_type).unwrap()(variables, id_2_task)));
        }
        if self.decorator_fn.contains_key(corresponding_type){
            return Ok(RealTaskType::Decorator(self.decorator_fn.get(corresponding_type).unwrap()(variables, id_2_task)));
        }

        Err(format!("generate_real_task not implemented for corresponding_type: {}", corresponding_type).into())
    }

    fn generate_task_proxy(&self, task_json:&serde_json::Value, variables:HashMap<String, serde_json::Value>,id_2_task:HashMap<u32, Weak<Box<dyn ITaskProxy>>>) -> Result<Rc<Box<dyn ITaskProxy>>, Box<dyn std::error::Error>>{
        let corresponding_type = task_json["Type"].as_str().unwrap();

        Err("generate_task_proxy not implemented".into())
    }


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