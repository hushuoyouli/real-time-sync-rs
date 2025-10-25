use std::rc::Rc;

enum TaskType{
    Action,
    Conditional,
    Composite,
    Decorator,
}


pub struct BehaviorTree{
    id: u64,
    taskList: Vec<Rc<Box<TaskType>>>,

}

impl BehaviorTree{

    
}