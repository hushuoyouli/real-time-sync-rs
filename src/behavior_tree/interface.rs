use  std::rc::Rc;

pub trait IUnit {
    fn id(&self) -> u64;
}

pub trait IClock{

}

pub struct TaskRuntimeData{

}

pub struct StackRuntimeData{

}

pub trait ITask{

}

pub trait IRebuildSyncDataCollector{
    fn stack(&mut self, behavior_tree:&impl IBehaviorTree, data:&StackRuntimeData);

	//	需要同步的action的回调
	fn action(&mut self, behavior_tree:&impl IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&impl ITask, datas:&Vec<Vec<u8>>);

	//	并发任务相关的执行栈恢复同步数据
	fn parallel(&mut self, behavior_tree:&impl IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&impl ITask, child_stack_runtime_datas:Vec<&StackRuntimeData>);
}


pub trait IBehaviorTree{
    fn id()->u64;

	fn enable(&mut self)->Result<(), Box<dyn std::error::Error>>;
	fn disable(&mut self)->Result<(), Box<dyn std::error::Error>>;
	fn update(&mut self);
	fn is_runnning(&self)->bool;

	fn unit(&self)->Rc<Box<dyn IUnit>>;
	fn rebuild_sync(&self, collector:&impl IRebuildSyncDataCollector);
	fn clock()->Rc<Box<dyn IClock>>;
}

pub trait IRuntimeEventHandle {
    
}
