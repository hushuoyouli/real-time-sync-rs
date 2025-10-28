use super::super::interface::{IConditional, ITaskProxy, IBehaviorTree};
use super::super::consts::TaskStatus;


pub struct NeedFollowJoystick{
    need_follow_joystick_flag:bool,
}

impl NeedFollowJoystick{
    pub fn new() -> Self{
        Self{
            need_follow_joystick_flag:true,
        }
    }
}

impl IConditional for NeedFollowJoystick{
    fn on_update(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->TaskStatus {
        if self.need_follow_joystick_flag {
            TaskStatus::Success
        }else{
            TaskStatus::Failure
        }
    }
}
