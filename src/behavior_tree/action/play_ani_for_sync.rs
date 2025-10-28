use super::super::interface::{IAction, ITaskProxy, IBehaviorTree};
use super::super::consts::TaskStatus;

pub struct PlayAniForSync{

}

impl PlayAniForSync{
    pub fn new() -> Self{
        Self{}
    }
}


impl IAction for PlayAniForSync{
    fn on_update(&mut self, task_proxy:&mut dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->TaskStatus {
        TaskStatus::Running
    }
}