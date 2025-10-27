use std::rc::{Rc, Weak};
use serde_json::from_str;
use std::collections::HashMap;

use super::interface::{IParser, TaskAddData,ITaskProxy, IAction, IConditional, IComposite, IDecorator, RealTaskType, IUnit};
use super::runtime::TaskProxy;
use super::consts::AbortType;

pub struct JsonParser{
    action_fn: HashMap<String, fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<Box<HashMap<i32, Weak<Box<dyn ITaskProxy>>>>>) -> Box<dyn IAction>>,
    conditional_fn: HashMap<String, fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<Box<HashMap<i32, Weak<Box<dyn ITaskProxy>>>>>) -> Box<dyn IConditional>>,
    composite_fn: HashMap<String, fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<Box<HashMap<i32, Weak<Box<dyn ITaskProxy>>>>>) -> Box<dyn IComposite>>,
    decorator_fn: HashMap<String, fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<Box<HashMap<i32, Weak<Box<dyn ITaskProxy>>>>>) -> Box<dyn IDecorator>>,
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

    pub fn register_action_fn(&mut self, name:&str, action_generate_fn:fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<Box<HashMap<i32, Weak<Box<dyn ITaskProxy>>>>>) -> Box<dyn IAction>){
        self.action_fn.insert(name.to_string(), action_generate_fn);
    }

    pub fn register_conditional_fn(&mut self, name:&str, conditional_generate_fn:fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<Box<HashMap<i32, Weak<Box<dyn ITaskProxy>>>>>) -> Box<dyn IConditional>){
        self.conditional_fn.insert(name.to_string(), conditional_generate_fn);
    }

    pub fn register_composite_fn(&mut self, name:&str, composite_generate_fn:fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<Box<HashMap<i32, Weak<Box<dyn ITaskProxy>>>>>) -> Box<dyn IComposite>){
        self.composite_fn.insert(name.to_string(), composite_generate_fn);
    }

    pub fn register_decorator_fn(&mut self, name:&str, decorator_generate_fn:fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<Box<HashMap<i32, Weak<Box<dyn ITaskProxy>>>>>) -> Box<dyn IDecorator>){
        self.decorator_fn.insert(name.to_string(), decorator_generate_fn);
    }


    fn generate_real_task(&self, corresponding_type:&str, variables:HashMap<String, serde_json::Value>,id_2_task:Weak<Box<HashMap<i32, Weak<Box<dyn ITaskProxy>>>>>) -> Result<RealTaskType, Box<dyn std::error::Error>>{
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

    fn generate_task_proxy(&self, task_json:&serde_json::Value, unit:&Weak<Box<dyn IUnit>>, id_2_task:Weak<Box<HashMap<i32, Weak<Box<dyn ITaskProxy>>>>>) -> Result<Rc<Box<dyn ITaskProxy>>, Box<dyn std::error::Error>>{
        let corresponding_type = task_json["Type"].as_str().unwrap();

        let mut variables:HashMap<String, serde_json::Value> = HashMap::new();
        for (key, value) in task_json.as_object().unwrap().iter() {
            match key.as_str() {
                "Type"|"Children"|"Name"|"ID"|"Instant"|"Disabled"|"BehaviorDesigner.Runtime.Tasks.AbortType,abortType"=> {
                    ()
                },
                _ => {
                    variables.insert(key.to_string(), value.clone());
                },
            }
        }

        let real_task: RealTaskType = self.generate_real_task(corresponding_type, variables, id_2_task.clone())?;

        let name = match task_json["Name"].as_str(){
            Some(name) => name,
            None => "",
        };

        let mut task_proxy: TaskProxy =TaskProxy::new(corresponding_type, name, unit, real_task);

        for (key, value) in task_json.as_object().unwrap().iter() {
            match key.as_str() {
                "ID" => task_proxy.set_id(value.as_i64().unwrap() as i32),
                "Instant" => task_proxy.set_instant(value.as_bool().unwrap()),
                "Disabled" => task_proxy.set_disabled(value.as_bool().unwrap()),
                "BehaviorDesigner.Runtime.Tasks.AbortType,abortType" => 
                {
                    let abort_type = match value.as_str().unwrap(){
                        "None" => AbortType::None,
                        "Self" => AbortType::Self_,
                        "LowerPriority" => AbortType::LowerPriority,
                        "Both" => AbortType::Both,
                        _ => AbortType::None,
                    };
                    task_proxy.set_abort_type(abort_type);
                },
                _ => (),
            }
        }

        if task_proxy.id() == 0{
            return Err("ID is 0".into());
        }

        let task_proxy:Rc<Box<dyn ITaskProxy>> = Rc::new(Box::new(task_proxy));
        let mut id_2_task = id_2_task.upgrade().unwrap();
        Rc::get_mut(&mut id_2_task).unwrap().insert(task_proxy.id(), Rc::downgrade(&task_proxy));
        Ok(task_proxy)
    }

    fn initialize_task(&self, task_json:&serde_json::Value, unit:&Weak<Box<dyn IUnit>>, id_2_task:Weak<Box<HashMap<i32, Weak<Box<dyn ITaskProxy>>>>>) -> Result<Rc<Box<dyn ITaskProxy>>, Box<dyn std::error::Error>>{
        let real_task = self.generate_task_proxy(task_json, unit, id_2_task)?;


        Err("initialize_task not implemented".into())
    }

}

impl IParser for JsonParser{
    fn deserialize(&self, config:&Vec<u8>, unit:&Weak<Box<dyn IUnit>>,task_add_data:&TaskAddData) -> Result<Rc<Box<dyn ITaskProxy>>, Box<dyn std::error::Error>>{
        let json: serde_json::Value = from_str(std::str::from_utf8(config)?).unwrap();
        let root_task_json = json.get("RootTask").unwrap();

        Err("Not implemented".into())
    }
}