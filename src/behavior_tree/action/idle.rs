use super::super::interface::{IAction, ITaskProxy, IBehaviorTree};
use super::super::consts::TaskStatus;

pub struct Idle{

}

impl IAction for Idle{
    fn on_update(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->TaskStatus {
        TaskStatus::Running
    }
}