use super::super::interface::{IDecorator, IParentTask, ITaskProxy, IBehaviorTree};
use super::super::consts::TaskStatus;

pub struct ReturnSuccess{
    pub execution_status:TaskStatus,
}

impl ReturnSuccess{
    pub fn new() -> Self{
        Self{
            execution_status:TaskStatus::Inactive,
        }
    }
}

impl IParentTask for ReturnSuccess{
    fn on_awake(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.execution_status = TaskStatus::Inactive;
    }

    fn can_execute(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->bool {
        self.execution_status == TaskStatus::Running || self.execution_status == TaskStatus::Inactive
    }

    fn can_run_parallel_children(&self)->bool {
        false
    }

    fn  on_child_executed1(&mut self, child_status:TaskStatus, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.execution_status = child_status;
    }

    fn on_end(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.execution_status = TaskStatus::Inactive;
    }

    fn current_child_index(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->u32 {0}

    fn decorate(&mut self, status:TaskStatus, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->TaskStatus {
        TaskStatus::Success
    }
}

impl IDecorator for ReturnSuccess{

}