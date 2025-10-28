use std::rc::{Rc, Weak};
use serde_json::from_str;
use std::collections::HashMap;
use std::cell::RefCell;

use super::interface::{IParser, TaskAddData,ITaskProxy, IAction, IConditional, IComposite, IDecorator, RealTaskType};
use super::runtime::TaskProxy;
use super::consts::AbortType;
use super::composite::sequence::Sequence;
use super::composite::selector::Selector;
use super::composite::parallel::Parallel;

pub struct JsonParser{
    action_fn: HashMap<String, fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<RefCell<Box<HashMap<i32, Weak<RefCell<Box<dyn ITaskProxy>>>>>>>) -> Box<dyn IAction>>,
    conditional_fn: HashMap<String, fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<RefCell<Box<HashMap<i32, Weak<RefCell<Box<dyn ITaskProxy>>>>>>>) -> Box<dyn IConditional>>,
    composite_fn: HashMap<String, fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<RefCell<Box<HashMap<i32, Weak<RefCell<Box<dyn ITaskProxy>>>>>>>) -> Box<dyn IComposite>>,
    decorator_fn: HashMap<String, fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<RefCell<Box<HashMap<i32, Weak<RefCell<Box<dyn ITaskProxy>>>>>>>) -> Box<dyn IDecorator>>,
}

impl JsonParser{
    pub fn new() -> Rc<RefCell<Box<dyn IParser>>>{
        let mut parser = Self{
            action_fn: HashMap::new(),
            conditional_fn: HashMap::new(),
            composite_fn: HashMap::new(),
            decorator_fn: HashMap::new(),
        };

        //  注册默认节点
        parser.register_composite_fn("BehaviorDesigner.Runtime.Tasks.Sequence", |variables, id_2_task| -> Box<dyn IComposite> {Box::new(Sequence::new())});
        parser.register_composite_fn("BehaviorDesigner.Runtime.Tasks.Selector", |variables, id_2_task| -> Box<dyn IComposite> {Box::new(Selector::new())});
        parser.register_composite_fn("BehaviorDesigner.Runtime.Tasks.Parallel", |variables, id_2_task| -> Box<dyn IComposite> {Box::new(Parallel::new())});

        Rc::new(RefCell::new(Box::new(parser)))
    }

    pub fn register_action_fn(&mut self, name:&str, action_generate_fn:fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<RefCell<Box<HashMap<i32, Weak<RefCell<Box<dyn ITaskProxy>>>>>>>) -> Box<dyn IAction>){
        self.action_fn.insert(name.to_string(), action_generate_fn);
    }

    pub fn register_conditional_fn(&mut self, name:&str, conditional_generate_fn:fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<RefCell<Box<HashMap<i32, Weak<RefCell<Box<dyn ITaskProxy>>>>>>>) -> Box<dyn IConditional>){
        self.conditional_fn.insert(name.to_string(), conditional_generate_fn);
    }

    pub fn register_composite_fn(&mut self, name:&str, composite_generate_fn:fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<RefCell<Box<HashMap<i32, Weak<RefCell<Box<dyn ITaskProxy>>>>>>>) -> Box<dyn IComposite>){
        self.composite_fn.insert(name.to_string(), composite_generate_fn);
    }

    pub fn register_decorator_fn(&mut self, name:&str, decorator_generate_fn:fn(variables:HashMap<String, serde_json::Value>,id_2_task:Weak<RefCell<Box<HashMap<i32, Weak<RefCell<Box<dyn ITaskProxy>>>>>>>) -> Box<dyn IDecorator>){
        self.decorator_fn.insert(name.to_string(), decorator_generate_fn);
    }


    fn generate_real_task(&self, corresponding_type:&str, variables:HashMap<String, serde_json::Value>,id_2_task:Weak<RefCell<Box<HashMap<i32, Weak<RefCell<Box<dyn ITaskProxy>>>>>>>) -> Result<RealTaskType, Box<dyn std::error::Error>>{
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

    fn generate_task_proxy(&self, task_json:&serde_json::Value, id_2_task:&Rc<RefCell<Box<HashMap<i32, Weak<RefCell<Box<dyn ITaskProxy>>>>>>>, all_tasks:&mut Vec<Weak<RefCell<Box<dyn ITaskProxy>>>>) -> Result<Rc<RefCell<Box<dyn ITaskProxy>>>, Box<dyn std::error::Error>>{
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

        let real_task: RealTaskType = self.generate_real_task(corresponding_type, variables, Rc::downgrade(id_2_task))?;

        let name = match task_json["Name"].as_str(){
            Some(name) => name,
            None => "",
        };

        let mut task_proxy: TaskProxy =TaskProxy::new(corresponding_type, name, real_task);

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

        let task_proxy:Rc<RefCell<Box<dyn ITaskProxy>>> = Rc::new(RefCell::new(Box::new(task_proxy)));

        if id_2_task.borrow().contains_key(&task_proxy.borrow().id()){
            return Err("ID already exists".into());
        }

        id_2_task.borrow_mut().insert(task_proxy.borrow().id(), Rc::downgrade(&task_proxy));
        all_tasks.push(Rc::downgrade(&task_proxy));

        match task_json["Children"].as_array(){
            Some(children) => 
            for child in children.iter(){
                let child = self.generate_task_proxy(child, id_2_task, all_tasks)?;
                task_proxy.borrow_mut().add_child(&child);
                //Rc::get_mut(&mut task_proxy).unwrap().add_child(&child);
            },
            None => (),
        };

        Ok(task_proxy)
    }

    fn initialize_task(&self, task_json:&serde_json::Value, id_2_task:&Rc<RefCell<Box<HashMap<i32, Weak<RefCell<Box<dyn ITaskProxy>>>>>>>, all_tasks:&mut Vec<Weak<RefCell<Box<dyn ITaskProxy>>>>) -> Result<Rc<RefCell<Box<dyn ITaskProxy>>>, Box<dyn std::error::Error>>{
        self.generate_task_proxy(task_json, id_2_task, all_tasks)
    }

    fn initialize_parent_task(&self, task_proxy:&mut Rc<RefCell<Box<dyn ITaskProxy>>>, task_add_data:&mut TaskAddData){
        let task_proxy_bak = task_proxy.clone();
        let task_proxy = Rc::get_mut(task_proxy).unwrap();
        
        if task_proxy.borrow().is_implements_iparenttask(){
            let old_parent = std::mem::replace(&mut task_add_data.parent, Some(Rc::downgrade(&task_proxy_bak)));

            task_proxy.borrow_mut().children_mut().iter_mut().for_each(|child|{
                self.initialize_parent_task(child, task_add_data);
            });
        
            task_add_data.parent = old_parent;
        }

    }
}

impl IParser for JsonParser{
    fn deserialize(&self, config:&Vec<u8>, task_add_data:&mut TaskAddData) -> Result<Rc<RefCell<Box<dyn ITaskProxy>>>, Box<dyn std::error::Error>>{
        let json: serde_json::Value = from_str(std::str::from_utf8(config)?).unwrap();
        let root_task_json: &serde_json::Value = json.get("RootTask").ok_or("json文件缺少RootTask的配置")?;
        let mut all_tasks:Vec<Weak<RefCell<Box<dyn ITaskProxy>>>> = Vec::new();
        let id_2_task = Rc::new(RefCell::new(Box::new(HashMap::new())));
        let mut root_task = self.initialize_task(root_task_json, &id_2_task,&mut all_tasks)?;

        let mut detached_tasks:Vec<Rc<RefCell<Box<dyn ITaskProxy>>>> = Vec::new();
        if let Some(_) = json.get("DetachedTasksConfigs"){
            let detached_tasks_configs = json.get("DetachedTasksConfigs").unwrap().as_array().unwrap();
            for detached_task_config in detached_tasks_configs.iter(){
                let detached_task = self.initialize_task(detached_task_config, &id_2_task,&mut all_tasks)?;
                detached_tasks.push(detached_task);
            }
        }

        //  初始化任务变量
        for task in all_tasks.iter(){
            let task = task.upgrade().unwrap();
            task.borrow_mut().initialize_variables()?;
        }

        self.initialize_parent_task(&mut root_task,task_add_data);

        for detached_task in detached_tasks.iter_mut(){
            self.initialize_parent_task(detached_task,task_add_data);
        }

        Ok(root_task)
    }
}