use super::super::interface::{IComposite, ITaskProxy, IBehaviorTree, IParentTask};
use super::super::consts::TaskStatus;

pub struct Parallel{
    current_child_index :u32,
	execution_status :Vec<TaskStatus>,
    children_len:u32,
}

impl Parallel{
    pub fn new() -> Self{
        Self{
            current_child_index:0,
            execution_status:Vec::new(),
            children_len:0,
        }
    }
}

impl IParentTask for Parallel{
    fn initialize_variables(&mut self, task_proxy:&dyn ITaskProxy)->Result<(), Box<dyn std::error::Error>> {
        self.children_len = task_proxy.children().len() as u32;
        Ok(())
    }

    fn on_awake(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.execution_status.clear();
        self.execution_status.resize(self.children_len as usize, TaskStatus::Inactive);
        self.current_child_index = 0;
    }

    fn on_start(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {}

    fn can_run_parallel_children(&self)->bool {true}

    fn  on_child_executed2(&mut self,index:u32, child_status:TaskStatus, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.execution_status[index as usize] = child_status;
    }

    fn on_child_started1(&mut self,index:u32, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.current_child_index += 1;
        self.execution_status[index as usize] = TaskStatus::Running;
    }

    fn current_child_index(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->u32 {
        self.current_child_index as u32
    }

    fn can_execute(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->bool {
       self.current_child_index < self.children_len  
    }

    fn override_status1(&mut self, status:TaskStatus, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->TaskStatus {
        let mut child_complete = true;

        for i in 0..self.children_len as usize {
            if self.execution_status[i] == TaskStatus::Running {
                child_complete = false;
            } else if self.execution_status[i] == TaskStatus::Failure {
                return TaskStatus::Failure;
            }
        }

        if child_complete {
            return TaskStatus::Success;
        }else{
            return TaskStatus::Running;
        }
    }

    fn on_conditional_abort(&mut self, index:u32,task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {}

    fn on_cancel_conditional_abort(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {}

    fn on_end(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
        self.current_child_index = 0;
        self.execution_status.resize(self.children_len as usize, TaskStatus::Inactive);
    }

}

impl IComposite for Parallel{

}