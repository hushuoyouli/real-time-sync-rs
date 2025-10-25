use  std::rc::Rc;
use super::consts::{TaskStatus, AbortType};

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
    fn corresponding_type(&self)->String;
	fn set_corresponding_type(&mut self, corresponding_type:String);
	//	所属的树
	fn owner(&self)->Rc<Box<dyn IBehaviorTree>>;
	fn set_owner(&mut self, owner:Rc<Box<dyn IBehaviorTree>>);
	//	父节点
	fn parent(&self)->Rc<Box<dyn IParentTask>>;
	fn set_parent(&mut self, parent:Rc<Box<dyn IParentTask>>);
	//	ID
	fn id(&self)->u32;
	fn set_id(&mut self, id:u32);
	//	名字
	fn name(&self)->String;
	fn set_name(&mut self, name:String);

	//是否是Instant
    fn is_instant(&self)->bool;
    fn set_is_instant(&mut self, is_instant:bool);

	//是否无效
    fn disabled(&self)->bool;
    fn set_disabled(&mut self, disabled:bool);

	//树的宿主
	fn unit(&self)->Rc<Box<dyn IUnit>>;
    fn set_unit(&mut self, unit:Rc<Box<dyn IUnit>>);

	fn on_awake(&mut self);
    fn on_start(&mut self);
    fn on_update(&mut self)->TaskStatus{
		TaskStatus::Inactive
	}

    fn on_end(&mut self);
    fn on_complete(&mut self);
}

pub struct SyncDataCollector {
    datas:Vec<Vec<u8>>,
}

impl SyncDataCollector{
    pub fn new() -> Self{
        Self{
            datas: Vec::new(),
        }
    }
    pub fn add_data(&mut self, data:Vec<u8>){
        self.datas.push(data);
    }
    pub fn get_and_clear(&mut self)->Vec<Vec<u8>>{
        let datas = self.datas.clone();
        self.datas.clear();
        datas
    }
}

pub trait IAction:ITask {
    fn is_sync_to_client(&self)->bool;
	fn rebuild_sync_datas(&self);
	fn set_sync_data_collector(&mut self, collector:SyncDataCollector);
	fn sync_data_collector(&self)->Option<SyncDataCollector>;
}

pub trait IConditional:ITask {

}

pub trait IParentTask :ITask{
	fn can_run_parallel_children(&self)->bool{
		false
	}
	/*
		跟是否可以并发有关的
		OnChildExecuted
		OnChildStarted
		OverrideStatus
	*/
	//	CanRunParallelChildren	为false的时候调用
	fn  on_child_executed1(&mut self, child_status:TaskStatus);
	fn  on_child_started0(&mut self);
	//	CanRunParallelChildren	为true的时候调用
	fn  on_child_executed2(&mut self,index:u32, child_status:TaskStatus);
	fn 	on_child_started1(&mut self,index:u32);

	fn current_child_index(&self)->u32;
	fn can_execute(&mut self)->bool;
	fn decorate(&mut self, status:TaskStatus)->TaskStatus;

	/*
		TODO：这个部分还需要继续了解
		OverrideStatus
	*/
	fn override_status0(&mut self)->TaskStatus;
	fn override_status1(&mut self, status:TaskStatus)->TaskStatus;

	fn on_conditional_abort(&mut self, index:u32);
	fn on_cancel_conditional_abort(&mut self, index:u32); //当Abort取消的时候，会调用这个接口

	fn children(&self)->Vec<Rc<Box<dyn ITask>>>;
	fn add_child(&mut self, task:Rc<Box<dyn ITask>>);
}

pub trait IComposite:IParentTask{
	fn abort_type(&self)->AbortType;
	fn set_abort_type(&mut self, abort_type:AbortType);
}

pub trait IDecorator:IParentTask{

}

pub trait IRebuildSyncDataCollector{

    fn stack(&mut self, behavior_tree: &dyn IBehaviorTree, data: &StackRuntimeData);

	//	需要同步的action的回调
	fn action(&mut self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITask, datas:&Vec<Vec<u8>>);

	//	并发任务相关的执行栈恢复同步数据
	fn parallel(&mut self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITask, child_stack_runtime_datas:Vec<&StackRuntimeData>);
}


pub trait IBehaviorTree{
    fn id(&self)->u64;

	fn enable(&mut self)->Result<(), Box<dyn std::error::Error>>;
	fn disable(&mut self)->Result<(), Box<dyn std::error::Error>>;
	fn update(&mut self);
	fn is_runnning(&self)->bool;

	fn unit(&self)->Rc<Box<dyn IUnit>>;
	fn rebuild_sync(&self, collector:&dyn IRebuildSyncDataCollector);
	fn clock(&self)->Rc<Box<dyn IClock>>;
}

pub trait IRuntimeEventHandle {
    fn post_initialize(&self, behavior_tree:&dyn IBehaviorTree, now_timestamp_in_milli:i64);
	//	树结束
	fn post_on_complete(&self, behavior_tree:&dyn IBehaviorTree, now_timestamp_in_milli:i64);

	//	同步需要
	fn new_stack(&self, behavior_tree:&dyn IBehaviorTree, data:&StackRuntimeData);
	fn remove_stack(&self, behavior_tree:&dyn IBehaviorTree, data:&StackRuntimeData, now_timestamp_in_milli:i64);

	//	以下3个回调可以用于追踪树的执行
	fn pre_on_start(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITask);
	fn post_on_update(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITask, now_timestamp_in_milli:i64, status:TaskStatus); //	任何的任务每帧调用的结果
	fn post_on_end(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITask, now_timestamp_in_milli:i64);

	//	需要同步的action的回调，同步需要
	fn action_post_on_start(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITask, datas:Vec<Vec<u8>>);
	fn action_post_on_update(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITask, now_timestamp_in_milli:i64, status:TaskStatus, datas:Vec<Vec<u8>>); //	任何的任务每帧调用的结果
	fn action_post_on_end(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITask, now_timestamp_in_milli:i64, datas:Vec<Vec<u8>>);

	//	需要同步的并发任务进入调用，同步需要
	fn parallel_pre_on_start(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITask);
	fn parallel_post_on_end(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITask, now_timestamp_in_milli:i64);

	//	并发任务相关的执行栈的增加/减少，调用顺序是NewStack/ParallelAddChildStack/ParallelRemoveChildStack/RemoveStack
	fn parallel_add_child_stack(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITask, child_stack_runtime_data:&StackRuntimeData);
	fn parallel_remove_child_stack(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITask, child_stack_runtime_data:&StackRuntimeData, now_timestamp_in_milli:i64);
    
}
