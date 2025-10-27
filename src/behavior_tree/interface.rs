use  std::rc::{Rc, Weak};
use super::consts::{TaskStatus, AbortType};

pub trait IUnit {
	fn id(&self) -> u64;
}

pub trait IClock{
	fn timestamp_in_mill(&self)->u64;
}

pub struct StackRuntimeData{
	pub stack_id:usize,
	pub start_time:u64,
}

impl StackRuntimeData{
	pub fn new(stack_id:usize, start_time:u64) -> Self{
		Self{
			stack_id,
			start_time,
		}
	}
}

pub struct TaskRuntimeData{

}


pub struct RunningStack{
    pub stack_id:usize,
    pub stack:Vec<u32>,
}

impl  RunningStack {
	pub fn new(stack_id:usize,stack_capacity:usize) -> Self{
		Self{
			stack_id,
			stack: Vec::with_capacity(stack_capacity),
		}
	}
}

pub struct TaskAddData{
	pub parent:Option<Weak<Box<dyn ITaskProxy>>>,
	pub parent_index:i32,
	pub depth:u32,
	pub composite_parent_index:u32,
	pub unit:Weak<Box<dyn IUnit>>,
	pub error_task:i32,
	pub error_task_name:String,
	pub owner:Weak<Box<dyn IBehaviorTree>>,
}

impl TaskAddData{
	pub fn new(unit:&Weak<Box<dyn IUnit>>,owner:&Weak<Box<dyn IBehaviorTree>>,) -> Self{
		Self{
			parent:None,
			parent_index:-1,
			depth:0,
			composite_parent_index:0,
			unit:unit.clone(),
			error_task:-1,
			error_task_name:"".to_string(),
			owner:owner.clone(),
		}
	}
}

pub trait IParser{
	fn deserialize(&self, config:&Vec<u8>, unit:&Weak<Box<dyn IUnit>>, task_add_data:&TaskAddData) -> Result<Rc<Box<dyn ITaskProxy>>, Box<dyn std::error::Error>>;
}


pub trait IBehaviorTree{
	fn set_self_weak_ref(&mut self, self_weak_ref:Option<Weak<Box<dyn IBehaviorTree>>>);
	
	fn id(&self)->u64;

	fn enable(&mut self)->Result<(), Box<dyn std::error::Error>>;
	fn disable(&mut self)->Result<(), Box<dyn std::error::Error>>;
	fn update(&mut self);
	fn is_runnning(&self)->bool;

	fn unit(&self)->Weak<Box<dyn IUnit>>;
	fn rebuild_sync(&self, collector:&dyn IRebuildSyncDataCollector);
	fn clock(&self)->Weak<Box<dyn IClock>>;
}


pub trait IRebuildSyncDataCollector{

	fn stack(&mut self, behavior_tree: &dyn IBehaviorTree, data: &StackRuntimeData);

	//	需要同步的action的回调
	fn action(&mut self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITaskProxy, datas:&Vec<Vec<u8>>);

	//	并发任务相关的执行栈恢复同步数据
	fn parallel(&mut self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITaskProxy, child_stack_runtime_datas:Vec<&StackRuntimeData>);
}

pub struct SyncDataCollector {
	datas:Vec<Vec<u8>>,
}

impl SyncDataCollector{
	pub fn new() -> Rc<Box<Self>>{
		Rc::new(Box::new(Self{
			datas: Vec::new(),
		}))
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


pub trait ITaskProxy{
	fn set_instant(&mut self, instant:bool);
	fn instant(&self)->bool;
	
	fn initialize_variables(&mut self)->Result<(), Box<dyn std::error::Error>>;
	fn set_owner(&mut self, owner:Option<Weak<Box<dyn IBehaviorTree>>>);
	fn owner(&self)->Option<Weak<Box<dyn IBehaviorTree>>>;
	fn set_parent(&mut self, parent:Option<Weak<Box<dyn ITaskProxy>>>);
	fn parent(&self)->Option<Weak<Box<dyn ITaskProxy>>>;
	fn corresponding_type(&self)->String;

	fn name(&self)->String;

	fn id(&self)->i32;
	fn set_id(&mut self, id:i32);

	//是否无效
	fn disabled(&self)->bool;
	fn set_disabled(&mut self, disabled:bool);

	fn unit(&self)->Weak<Box<dyn IUnit>>;
	fn on_awake(&mut self);
    fn on_start(&mut self);
    fn on_end(&mut self);
    fn on_complete(&mut self);

	//提供给Action与Conditional使用
	fn on_update(&mut self)->TaskStatus;
	fn is_sync_to_client(&self)->bool;
	
	fn rebuild_sync_datas(&self);
	
	fn set_sync_data_collector(&mut self, collector:Option<Rc<Box<SyncDataCollector>>>);
	fn sync_data_collector(&self)->Option<Rc<Box<SyncDataCollector>>>;

	//	IParentTask接口
	fn can_run_parallel_children(&self)->bool;
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

	fn can_execute(&self)->bool;
	
	fn decorate(&mut self, status:TaskStatus)->TaskStatus;

	/*
		TODO：这个部分还需要继续了解
		OverrideStatus
	*/
	fn override_status1(&mut self, status:TaskStatus)->TaskStatus;
	fn on_conditional_abort(&mut self, index:u32);

	fn on_cancel_conditional_abort(&mut self, index:u32);

	fn children(&self)->&Vec<Rc<Box<dyn ITaskProxy>>>;

	fn children_mut(&mut self)->&mut Vec<Rc<Box<dyn ITaskProxy>>>;
	
	fn add_child(&mut self, task:&Rc<Box<dyn ITaskProxy>>);
	fn abort_type(&self)->AbortType;
	
	fn set_abort_type(&mut self, abort_type:AbortType);
	//是否是action
	fn is_implements_iaction(&self)-> bool;

	//是否是composite
	fn is_implements_icomposite(&self)-> bool;
	//是否是decorator
	fn is_implements_idecorator(&self)-> bool;

	//是否是conditional
	fn is_implements_iconditional(&self)-> bool;

	//是否是parent task
	fn is_implements_iparenttask(&self)-> bool;
}

pub trait IRuntimeEventHandle {
	fn post_initialize(&self, behavior_tree:&dyn IBehaviorTree, now_timestamp_in_milli:u64);
	//	树结束
	fn post_on_complete(&self, behavior_tree:&dyn IBehaviorTree, now_timestamp_in_milli:u64);

	//	同步需要
	fn new_stack(&self, behavior_tree:&dyn IBehaviorTree, data:&StackRuntimeData);
	fn remove_stack(&self, behavior_tree:&dyn IBehaviorTree, data:&StackRuntimeData, now_timestamp_in_milli:u64);

	//	以下3个回调可以用于追踪树的执行
	fn pre_on_start(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITaskProxy);
	fn post_on_update(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITaskProxy, now_timestamp_in_milli:u64, status:TaskStatus); //	任何的任务每帧调用的结果
	fn post_on_end(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITaskProxy, now_timestamp_in_milli:u64);

	//	需要同步的action的回调，同步需要
	fn action_post_on_start(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITaskProxy, datas:Vec<Vec<u8>>);
	fn action_post_on_update(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITaskProxy, now_timestamp_in_milli:u64, status:TaskStatus, datas:Vec<Vec<u8>>); //	任何的任务每帧调用的结果
	fn action_post_on_end(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITaskProxy, now_timestamp_in_milli:u64, datas:Vec<Vec<u8>>);

	//	需要同步的并发任务进入调用，同步需要
	fn parallel_pre_on_start(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITaskProxy);
	fn parallel_post_on_end(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITaskProxy, now_timestamp_in_milli:u64);

	//	并发任务相关的执行栈的增加/减少，调用顺序是NewStack/ParallelAddChildStack/ParallelRemoveChildStack/RemoveStack
	fn parallel_add_child_stack(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITaskProxy, child_stack_runtime_data:&StackRuntimeData);
	fn parallel_remove_child_stack(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&dyn ITaskProxy, child_stack_runtime_data:&StackRuntimeData, now_timestamp_in_milli:u64);
	
}


#[allow(unused_variables)]
pub trait IAction {
	fn initialize_variables(&mut self)->Result<(), Box<dyn std::error::Error>>{Ok(())}
	fn on_awake(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
    fn on_start(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
    fn on_update(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->TaskStatus;
    fn on_end(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
    fn on_complete(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}

	//	默认不需要同步
	fn is_sync_to_client(&self)->bool{
		false
	}

	fn rebuild_sync_datas(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
}

#[allow(unused_variables)]
pub trait IConditional{
	fn initialize_variables(&mut self)->Result<(), Box<dyn std::error::Error>>{Ok(())}
	fn on_awake(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
    fn on_start(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
    fn on_update(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->TaskStatus;
    fn on_end(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
    fn on_complete(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
}


#[allow(unused_variables)]
pub trait  IParentTask {
	fn initialize_variables(&mut self)->Result<(), Box<dyn std::error::Error>>{Ok(())}
	fn on_awake(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
    fn on_start(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}   
    fn on_end(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
    fn on_complete(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}

	fn can_run_parallel_children(&self)->bool{ false }
	/*
		跟是否可以并发有关的
		OnChildExecuted
		OnChildStarted
		OverrideStatus
	*/
	//	CanRunParallelChildren	为false的时候调用
	fn  on_child_executed1(&mut self, child_status:TaskStatus, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
	fn  on_child_started0(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
	//	CanRunParallelChildren	为true的时候调用
	fn  on_child_executed2(&mut self,index:u32, child_status:TaskStatus, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
	fn 	on_child_started1(&mut self,index:u32, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}

	fn current_child_index(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->u32;
	fn can_execute(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->bool;
	fn decorate(&mut self, status:TaskStatus, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->TaskStatus{status}

	/*
		TODO：这个部分还需要继续了解
		OverrideStatus
	*/
	fn override_status1(&mut self, status:TaskStatus, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->TaskStatus{status}

	fn on_conditional_abort(&mut self, index:u32,task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){}
	fn on_cancel_conditional_abort(&mut self, index:u32,task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){} //当Abort取消的时候，会调用这个接口
}

pub trait IComposite:IParentTask{
	fn is_composite(&self)->bool{
		true
	}
}

pub trait IDecorator:IParentTask{
	fn is_decorator(&self)->bool{
		true
	}
}

pub enum RealTaskType{
	Action(Box<dyn IAction>),
	Conditional(Box<dyn IConditional>),
	Composite(Box<dyn IComposite>),
	Decorator(Box<dyn IDecorator>),
}






