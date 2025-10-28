use super::super::interface::{IDecorator, IParentTask, ITaskProxy, IBehaviorTree};
use super::super::consts::TaskStatus;

pub struct UntilForever{

}

impl UntilForever{
    pub fn new() -> Self{
        Self{}
    }
}

impl IParentTask for UntilForever{
    fn current_child_index(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->u32 {0}

    fn can_execute(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->bool {
        true
    }
}