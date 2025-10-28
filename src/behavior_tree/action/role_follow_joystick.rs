use super::super::interface::{IAction, ITaskProxy, IBehaviorTree};
use super::super::consts::TaskStatus;

pub struct RoleFollowJoystick{

}

impl RoleFollowJoystick{
    pub fn new() -> Self{
        Self{}
    }
}


impl IAction for RoleFollowJoystick{
    fn on_update(&mut self, task_proxy:&mut dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->TaskStatus {
        TaskStatus::Running
    }
}