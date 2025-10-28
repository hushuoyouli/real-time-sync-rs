use super::super::interface::{IDecorator, IParentTask, ITaskProxy, IBehaviorTree};
use super::super::consts::TaskStatus;

pub struct UntilFailure{
    pub execution_status:TaskStatus,
}

impl UntilFailure{
    pub fn new() -> Self{
        Self{
            execution_status:TaskStatus::Inactive,
        }
    }
}

impl IParentTask for UntilFailure{
    fn on_awake(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.execution_status = TaskStatus::Inactive;
    }

    fn current_child_index(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->u32 {
        0
    }

    fn can_execute(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->bool {
        self.execution_status == TaskStatus::Success || self.execution_status == TaskStatus::Inactive
    }

    fn on_end(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.execution_status = TaskStatus::Inactive;
    }

    fn  on_child_executed1(&mut self, child_status:TaskStatus, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.execution_status = child_status;
    }
}

impl IDecorator for UntilFailure{}