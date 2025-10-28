use super::super::interface::{IComposite, ITaskProxy, IBehaviorTree, IParentTask};
use super::super::consts::TaskStatus;

pub struct Sequence{
    current_child_index :u32,
	execution_status :TaskStatus,
    children_len:u32,
}

impl Sequence{
    fn new() -> Self{
        Self{
            current_child_index:0,
            execution_status:TaskStatus::Inactive,
            children_len:0,
        }
    }
}

impl IParentTask for Sequence{
    fn initialize_variables(&mut self, task_proxy:&dyn ITaskProxy)->Result<(), Box<dyn std::error::Error>> {
        self.children_len = task_proxy.children().len() as u32;
        Ok(())
    }




    fn on_awake(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {        
        self.execution_status = TaskStatus::Inactive;
        self.current_child_index = 0;
    }

    fn on_start(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {}

    fn can_run_parallel_children(&self)->bool {false}

    fn  on_child_executed1(&mut self, child_status:TaskStatus, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.current_child_index += 1;
        self.execution_status = child_status;
    }

    fn  on_child_started0(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        
    }

    fn current_child_index(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->u32 {
        self.current_child_index as u32
    }

    fn can_execute(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->bool {
        self.current_child_index < self.children_len && self.execution_status != TaskStatus::Failure
    }

    fn on_conditional_abort(&mut self, index:u32,task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.current_child_index = index;
        self.execution_status = TaskStatus::Inactive
    }

    fn on_cancel_conditional_abort(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.current_child_index = 0;
        self.execution_status = TaskStatus::Inactive;
    }

    fn on_end(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.current_child_index = 0;
        self.execution_status = TaskStatus::Inactive;
    }
}

impl IComposite for Sequence{

}