use std::{collections::HashMap, rc::{Rc,Weak}};
use super::consts::{TaskStatus, AbortType};


pub trait IUnit {
	fn id(&self) -> u64;
}

pub trait IClock{
	fn timestamp_in_mill(&self)->u64;
}

pub struct TaskRuntimeData{

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

pub trait IRebuildSyncDataCollector{

	fn stack(&mut self, behavior_tree: &dyn IBehaviorTree, data: &StackRuntimeData);

	//	需要同步的action的回调
	fn action(&mut self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&TaskProxy, datas:&Vec<Vec<u8>>);

	//	并发任务相关的执行栈恢复同步数据
	fn parallel(&mut self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&TaskProxy, child_stack_runtime_datas:Vec<&StackRuntimeData>);
}


pub trait IBehaviorTree{
	fn id(&self)->u64;

	fn enable(&mut self, parser:&dyn IParser)->Result<(), Box<dyn std::error::Error>>;
	fn disable(&mut self)->Result<(), Box<dyn std::error::Error>>;
	fn update(&mut self);
	fn is_runnning(&self)->bool;

	fn unit(&self)->Rc<Box<dyn IUnit>>;
	fn rebuild_sync(&self, collector:&dyn IRebuildSyncDataCollector);
	fn clock(&self)->Rc<Box<dyn IClock>>;
}

pub trait IRuntimeEventHandle {
	fn post_initialize(&self, behavior_tree:&dyn IBehaviorTree, now_timestamp_in_milli:u64);
	//	树结束
	fn post_on_complete(&self, behavior_tree:&dyn IBehaviorTree, now_timestamp_in_milli:u64);

	//	同步需要
	fn new_stack(&self, behavior_tree:&dyn IBehaviorTree, data:&StackRuntimeData);
	fn remove_stack(&self, behavior_tree:&dyn IBehaviorTree, data:&StackRuntimeData, now_timestamp_in_milli:u64);

	//	以下3个回调可以用于追踪树的执行
	fn pre_on_start(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&TaskProxy);
	fn post_on_update(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&TaskProxy, now_timestamp_in_milli:u64, status:TaskStatus); //	任何的任务每帧调用的结果
	fn post_on_end(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&TaskProxy, now_timestamp_in_milli:u64);

	//	需要同步的action的回调，同步需要
	fn action_post_on_start(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&TaskProxy, datas:Vec<Vec<u8>>);
	fn action_post_on_update(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&TaskProxy, now_timestamp_in_milli:u64, status:TaskStatus, datas:Vec<Vec<u8>>); //	任何的任务每帧调用的结果
	fn action_post_on_end(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&TaskProxy, now_timestamp_in_milli:u64, datas:Vec<Vec<u8>>);

	//	需要同步的并发任务进入调用，同步需要
	fn parallel_pre_on_start(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&TaskProxy);
	fn parallel_post_on_end(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&TaskProxy, now_timestamp_in_milli:u64);

	//	并发任务相关的执行栈的增加/减少，调用顺序是NewStack/ParallelAddChildStack/ParallelRemoveChildStack/RemoveStack
	fn parallel_add_child_stack(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&TaskProxy, child_stack_runtime_data:&StackRuntimeData);
	fn parallel_remove_child_stack(&self, behavior_tree:&dyn IBehaviorTree, task_runtime_data:&TaskRuntimeData, stack_runtime_data:&StackRuntimeData, task:&TaskProxy, child_stack_runtime_data:&StackRuntimeData, now_timestamp_in_milli:u64);
	
}

#[allow(unused_variables)]
pub trait IAction {
	fn on_awake(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_start(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_update(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree)->TaskStatus;
    fn on_end(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_complete(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}

	//	默认不需要同步
	fn is_sync_to_client(&self)->bool{
		false
	}
	fn rebuild_sync_datas(&self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
}

pub struct EmptyAction;
impl IAction for EmptyAction {
	fn on_update(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree)->TaskStatus{
		TaskStatus::Success
	}
}

#[allow(unused_variables)]
pub trait IConditional{
	fn on_awake(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_start(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_update(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree)->TaskStatus;
    fn on_end(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_complete(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
}


#[allow(unused_variables)]
pub trait  IParentTask {
	fn on_awake(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_start(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}   
    fn on_end(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_complete(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}

	fn can_run_parallel_children(&self)->bool{ false }
	/*
		跟是否可以并发有关的
		OnChildExecuted
		OnChildStarted
		OverrideStatus
	*/
	//	CanRunParallelChildren	为false的时候调用
	fn  on_child_executed1(&mut self, child_status:TaskStatus, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
	fn  on_child_started0(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
	//	CanRunParallelChildren	为true的时候调用
	fn  on_child_executed2(&mut self,index:u32, child_status:TaskStatus, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
	fn 	on_child_started1(&mut self,index:u32, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}

	fn current_child_index(&self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree)->u32;
	fn can_execute(&self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree)->bool;
	fn decorate(&mut self, status:TaskStatus, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree)->TaskStatus{status}

	/*
		TODO：这个部分还需要继续了解
		OverrideStatus
	*/
	fn override_status1(&mut self, status:TaskStatus, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree)->TaskStatus{status}

	fn on_conditional_abort(&mut self, index:u32,task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
	fn on_cancel_conditional_abort(&mut self, index:u32,task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){} //当Abort取消的时候，会调用这个接口
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


pub struct TaskProxy{
	corresponding_type:String,
	id:i32,
	name:String,
	disabled:bool,
	unit:Rc<Box<dyn IUnit>>,

	//	IComposite专用
	abort_type:AbortType,
	children:Vec<Rc<Box<TaskProxy>>>,
	real_task:RealTaskType,
	sync_data_collector:Option<Rc<Box<SyncDataCollector>>>,
	parent:Option<Weak<Box<TaskProxy>>>,
}

#[allow(unused_variables)]
impl TaskProxy {
	pub fn new(corresponding_type:&str, name:&str,unit:&Rc<Box<dyn IUnit>>,real_task:RealTaskType) -> Self{
		Self{
			corresponding_type: corresponding_type.to_string(),
			id:0,
			name:name.to_string(),
			disabled: false,
			unit: unit.clone(),
			abort_type: AbortType::None,
			children: Vec::new(),
			real_task:real_task,
			sync_data_collector: None,
			parent: None,
		}
	}

	pub fn set_parent(&mut self, parent:Option<Weak<Box<TaskProxy>>>){
		self.parent = parent;
	}

	pub fn parent(&self)->Option<Weak<Box<TaskProxy>>>{
		self.parent.clone()
	}

	pub fn corresponding_type(&self)->String{
		self.corresponding_type.clone()
	}

	pub fn name(&self)->String{
		self.name.clone()
	}

	pub fn id(&self)->i32{
		self.id
	}

	pub fn set_id(&mut self, id:i32){
		self.id = id;
	}

	//是否无效
	pub fn disabled(&self)->bool{
		self.disabled
	}

	pub fn set_disabled(&mut self, disabled:bool){
		self.disabled = disabled;
	}

	pub fn unit(&self)->Rc<Box<dyn IUnit>>{
		self.unit.clone()
	}


	pub fn on_awake(&mut self,behavior_tree:&BehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Action(action) => action.on_awake(self, behavior_tree),
			RealTaskType::Composite(composite) => composite.on_awake(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_awake(self, behavior_tree),
			RealTaskType::Conditional(conditional) => conditional.on_awake(self, behavior_tree),
		}

		self.real_task = real_task;
	}

    pub fn on_start(&mut self,behavior_tree:&BehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Action(action) => action.on_start(self, behavior_tree),
			RealTaskType::Composite(composite) => composite.on_start(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_start(self, behavior_tree),
			RealTaskType::Conditional(conditional) => conditional.on_start(self, behavior_tree),
		}

		self.real_task = real_task;
	}

    pub fn on_end(&mut self,behavior_tree:&BehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Action(action) => action.on_end(self, behavior_tree),
			RealTaskType::Composite(composite) => composite.on_end(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_end(self, behavior_tree),
			RealTaskType::Conditional(conditional) => conditional.on_end(self, behavior_tree),
		}

		self.real_task = real_task;
	}

    pub fn on_complete(&mut self,behavior_tree:&BehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Action(action) => action.on_complete(self, behavior_tree),
			RealTaskType::Composite(composite) => composite.on_complete(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_complete(self, behavior_tree),
			RealTaskType::Conditional(conditional) => conditional.on_complete(self, behavior_tree),
		}

		self.real_task = real_task;
	}

	//提供给Action与Conditional使用
	pub fn on_update(&mut self,behavior_tree:&BehaviorTree)->TaskStatus{
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		let status = match &mut real_task {
			RealTaskType::Action(action) => action.on_update(self, behavior_tree),
			RealTaskType::Conditional(conditional) => conditional.on_update(self, behavior_tree),
			_ => {
				panic!("error");
				TaskStatus::Inactive
			},
		};

		self.real_task = real_task;
		status
	}

	//is_sync_to_client,rebuild_sync_datas,set_sync_data_collector,sync_data_collector这几个接口是提供给action用于同步的
	pub fn is_sync_to_client(&self)->bool{
		match &self.real_task {
			RealTaskType::Action(action) => action.is_sync_to_client(),
			_ => {
					panic!("error");
					false
				},
		}
	}
	
	pub fn rebuild_sync_datas(&self,behavior_tree:&BehaviorTree){
		match &self.real_task {
			RealTaskType::Action(action) => action.rebuild_sync_datas(self, behavior_tree),
			_ => {panic!("error");},
		}
	}
	
	pub fn set_sync_data_collector(&mut self, collector:Rc<Box<SyncDataCollector>>){
		match &self.real_task {
			RealTaskType::Action(_) => self.sync_data_collector = Some(collector),
			_ => {panic!("error");},
		}
	}
	
	pub fn sync_data_collector(&self)->Option<Rc<Box<SyncDataCollector>>>{
		self.sync_data_collector.clone()
	}

	//	IParentTask接口
	fn can_run_parallel_children(&self)->bool{
		match &self.real_task {
			RealTaskType::Composite(composite) => composite.can_run_parallel_children(),
			RealTaskType::Decorator(decorator) => false,
			_ =>{panic!("error"); false},
		}
	}
	/*
		跟是否可以并发有关的
		OnChildExecuted
		OnChildStarted
		OverrideStatus
	*/
	//	CanRunParallelChildren	为false的时候调用
	pub fn  on_child_executed1(&mut self, child_status:TaskStatus,behavior_tree:&BehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_child_executed1(child_status,self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_child_executed1(child_status,self, behavior_tree),
			_ => {panic!("error");},
		}

		self.real_task = real_task;
	}

	pub fn  on_child_started0(&mut self,behavior_tree:&BehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_child_started0(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_child_started0(self, behavior_tree),
			_ => {panic!("error");},
		}

		self.real_task = real_task;
	}
	//	CanRunParallelChildren	为true的时候调用
	#[allow(unused_variables)]
	pub fn  on_child_executed2(&mut self,index:u32, child_status:TaskStatus,behavior_tree:&BehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_child_executed2(index, child_status,self, behavior_tree),
			//RealTaskType::Decorator(decorator) => decorator.on_child_executed2(index, child_status,self, behavior_tree),
			_ => {panic!("error");},
		}

		self.real_task = real_task;
	}

	pub fn 	on_child_started1(&mut self,index:u32,behavior_tree:&BehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_child_started1(index,self, behavior_tree),
			//RealTaskType::Decorator(decorator) => decorator.on_child_executed2(index, child_status,self, behavior_tree),
			_ => {panic!("error");},
		}

		self.real_task = real_task;
	}

	pub fn current_child_index(&self, behavior_tree:&BehaviorTree)->u32{
		let result = match &self.real_task {
			RealTaskType::Composite(composite) => composite.current_child_index(self, behavior_tree),
			//RealTaskType::Decorator(decorator) => decorator.on_child_executed2(index, child_status,self, behavior_tree),
			_ => {panic!("error");  },
		};
		
		result
	}

	pub fn can_execute(&self, behavior_tree:&BehaviorTree)->bool{
		match &self.real_task {
			RealTaskType::Composite(composite) => composite.can_execute(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.can_execute(self, behavior_tree),
			_ => {panic!("error");},
		}
	}
	
	pub fn decorate(&mut self, status:TaskStatus, behavior_tree:&BehaviorTree)->TaskStatus{
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		let result = match &mut real_task {
			RealTaskType::Composite(composite) => composite.decorate(status,self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.decorate(status,self, behavior_tree),
			_ => {panic!("error");},
		};

		self.real_task = real_task;
		result
	}

	/*
		TODO：这个部分还需要继续了解
		OverrideStatus
	*/
	#[allow(unused_variables)]
	pub fn override_status1(&mut self, status:TaskStatus, behavior_tree:&BehaviorTree)->TaskStatus{
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		let result = match &mut real_task {
			RealTaskType::Composite(composite) => composite.override_status1(status,self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.override_status1(status,self, behavior_tree),
			_ => {panic!("error");},
		};

		self.real_task = real_task;
		result
	}

	pub fn on_conditional_abort(&mut self, index:u32,behavior_tree:&BehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		let result = match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_conditional_abort(index,self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_conditional_abort(index,self, behavior_tree),
			_ => {panic!("error");},
		};

		self.real_task = real_task;
		result
	}

	pub fn on_cancel_conditional_abort(&mut self, index:u32,behavior_tree:&BehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		let result = match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_cancel_conditional_abort(index,self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_cancel_conditional_abort(index,self, behavior_tree),
			_ => {panic!("error");},
		};

		self.real_task = real_task;
		result
	}

	pub fn children(&self)->&Vec<Rc<Box<TaskProxy>>>{
		&self.children
	}

	pub fn children_mut(&mut self)->&mut Vec<Rc<Box<TaskProxy>>>{
		&mut self.children
	}
	
	pub fn add_child(&mut self, task:&Rc<Box<TaskProxy>>){
		self.children.push(task.clone());
	}

	pub fn abort_type(&self)->AbortType{
		self.abort_type
	}
	
	pub fn set_abort_type(&mut self, abort_type:AbortType){
		self.abort_type = abort_type;
	}

	//是否是action
	pub fn is_implements_iaction(&self)-> bool{
		match self.real_task {
			RealTaskType::Action(_) => true,
			_ => false,
		}
	}

	//是否是composite
	pub fn is_implements_icomposite(&self)-> bool{
		match self.real_task {
			RealTaskType::Composite(_) => true,
			_ => false,
		}
	}
	
	//是否是decorator
	pub fn is_implements_idecorator(&self)-> bool{
		match self.real_task {
			RealTaskType::Decorator(_) => true,
			_ => false,
		}
	}

	//是否是conditional
	pub fn is_implements_iconditional(&self)-> bool{
		match self.real_task {
			RealTaskType::Conditional(_) => true,
			_ => false,
		}
	}

	//是否是parent task
	pub fn is_implements_iparenttask(&self)-> bool{
		match self.real_task {
			RealTaskType::Composite(_) => true,
			RealTaskType::Decorator(_) => true,
			_ => false,
		}
	}
}

pub struct ConditionalReevaluate{

}


pub struct StackRuntimeData{

}

struct RunningStack{
    stack_id:u32,
    stack:Vec<u32>,
    stack_runtime_data:Rc<Box<StackRuntimeData>>,
}

pub struct TaskAddData{
	pub parent:Option<Rc<Box<TaskProxy>>>,
	pub parent_index:i32,
	pub depth:u32,
	pub composite_parent_index:u32,
	pub unit:Rc<Box<dyn IUnit>>,
	pub error_task:i32,
	pub error_task_name:String,
}

impl TaskAddData{
	pub fn new(unit:&Rc<Box<dyn IUnit>>) -> Self{
		Self{
			parent:None,
			parent_index:-1,
			depth:0,
			composite_parent_index:0,
			unit:unit.clone(),
			error_task:-1,
			error_task_name:"".to_string(),
		}
	}
}

pub trait IParser{
	fn deserialize(&self, config:&Vec<u8>, task_add_data:&TaskAddData) -> Result<Rc<Box<TaskProxy>>, Box<dyn std::error::Error>>;
}

struct EntryRoot{
	execution_status:TaskStatus,
}

impl  EntryRoot {
	pub fn new() -> Box<dyn IDecorator>{
		Box::new(Self{
			execution_status:TaskStatus::Inactive,
		})
	}
}

impl IParentTask for EntryRoot {
	fn on_awake(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree) {
		self.execution_status = TaskStatus::Inactive;
	}	

	fn can_execute(&self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree)->bool {
		if let TaskStatus::Failure = self.execution_status {
			true
		}else{
			false
		}
	}

	fn current_child_index(&self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree)->u32{
		0
	}

	fn on_end(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){
		self.execution_status = TaskStatus::Inactive;
	}

	fn  on_child_executed1(&mut self, child_status:TaskStatus, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree) {
		self.execution_status = child_status;
	}
}

impl IDecorator for EntryRoot {
}

pub struct BehaviorTree{
    id: u64,

    task_list: Vec<Rc<Box<TaskProxy>>>,
    parent_index:Vec<i32>,

    children_index :Vec<Vec<i32>>,
	relative_child_index:Vec<i32>,

    active_stack :Vec<Rc<Box<RunningStack>>>,
	non_instant_task_status:Vec<TaskStatus>,
	conditional_reevaluate:Vec<Rc<Box<ConditionalReevaluate>>>,
	conditional_reevaluate_map:HashMap<u32, Rc<Box<ConditionalReevaluate>>>,

	parent_composite_index:Vec<i32>,
	child_conditional_index:Vec<Vec<i32>>,

    is_running:bool,
	initialize_first_stack_and_first_task:bool, //	是否需要初始化第一个执行栈和第一个任务
	execution_status:TaskStatus,
	config:Vec<u8>,
	unit:Rc<Box<dyn IUnit>>,
	root_task:Option<Rc<Box<TaskProxy>>>,
	clock:Rc<Box<dyn IClock>>,                            
	stack_id:u32,
    stack_id_to_stack_data:HashMap<u32, Rc<Box<RunningStack>>>,

	task_datas:HashMap<u32, Rc<Box<TaskRuntimeData>>>,

	stack_id_to_parallel_task_id:HashMap<u32, u32>,
	parallel_task_id_to_stack_ids:HashMap<u32, Vec<u32>>,

	runtime_event_handle:Rc<Box<dyn IRuntimeEventHandle>>,
	initialize_for_base_flag:bool
}
#[allow(unused_variables)]
impl BehaviorTree{
	pub fn new(id: u64, config:&Vec<u8>,	unit:Rc<Box<dyn IUnit>>,  clock:Rc<Box<dyn IClock>>, 
		runtime_event_handle:Rc<Box<dyn IRuntimeEventHandle>>) -> Rc<Box<Self>>{
		Rc::new(Box::new(Self{
			id,
			task_list: Vec::new(),
			parent_index: Vec::new(),
			children_index: Vec::new(),
			relative_child_index: Vec::new(),
			active_stack: Vec::new(),
			non_instant_task_status: Vec::new(),
			conditional_reevaluate: Vec::new(),
			conditional_reevaluate_map: HashMap::new(),
			parent_composite_index:Vec::new(),
			child_conditional_index:Vec::new(),
			is_running: false,
			initialize_first_stack_and_first_task: false,
			execution_status: TaskStatus::Inactive,
			config: config.clone(),
			unit:unit,
			root_task:None,
			clock:clock,
			stack_id: 0,
			stack_id_to_stack_data: HashMap::new(),
			task_datas: HashMap::new(),
			stack_id_to_parallel_task_id: HashMap::new(),
			parallel_task_id_to_stack_ids: HashMap::new(),
			runtime_event_handle: runtime_event_handle,
			initialize_for_base_flag: false,
		}))
	}

	fn initialize_for_base(&mut self, parser:&dyn IParser) ->Result<(), Box<dyn std::error::Error>>{
		self.task_list.clear();
		self.parent_index.clear();
		self.children_index.clear();
		self.relative_child_index.clear();
		self.parent_composite_index.clear();
		self.child_conditional_index.clear();
		self.root_task = None;
		let taskAddData: TaskAddData = TaskAddData::new(&self.unit);

		let root_task = parser.deserialize(&self.config, &taskAddData)?;
		let entry_root = EntryRoot::new();
		let mut root_proxy = TaskProxy::new("EntryRoot", "EntryRoot", &self.unit, RealTaskType::Decorator(entry_root));
		root_proxy.add_child(&root_task);
		
		self.root_task = Some(Rc::new(Box::new(root_proxy)));
		self.task_list.push(self.root_task.clone().unwrap());
		self.parent_index.push(-1);
		self.parent_composite_index.push(-1);
		self.child_conditional_index.push(Vec::with_capacity(10));
		self.relative_child_index.push(-1);
		let mut parent_composite_index = -1;

		Rc::get_mut(self.root_task.as_mut().unwrap()).unwrap().set_id(0);

		if self.root_task.as_ref().unwrap().is_implements_iparenttask(){
			if self.root_task.as_ref().unwrap().is_implements_icomposite(){
				parent_composite_index = self.root_task.as_ref().unwrap().id();
			}

			let mut parent_task = self.root_task.as_mut().unwrap().clone();
			let mut children = Rc::get_mut(&mut parent_task).unwrap().children_mut().clone();


			for child in children.iter_mut(){
				self.parse_child_task(child, &parent_task, parent_composite_index)?;
			}
		}
		
		Ok(())
	}

	fn parse_child_task(&mut self, child_task:&mut Rc<Box<TaskProxy>>, parent_task:&Rc<Box<TaskProxy>>, mut parent_composite_index: i32)->Result<(), Box<dyn std::error::Error>>{
		let index = self.task_list.len() as i32;
		let parent_index = parent_task.id();

		self.children_index[parent_index as usize].push(index);
		self.relative_child_index.push(self.children_index[parent_index as usize].len() as i32 - 1);
		self.task_list.push(child_task.clone());
		self.parent_index.push(parent_task.id());
		self.parent_composite_index.push(parent_composite_index);
		self.child_conditional_index.push(Vec::with_capacity(10));
		self.children_index.push(Vec::with_capacity(10));

		Rc::get_mut(child_task).unwrap().set_id(index);
		Rc::get_mut(child_task).unwrap().set_parent(Some(Rc::downgrade(parent_task)));

		if child_task.is_implements_iparenttask(){
			if child_task.is_implements_icomposite(){
				parent_composite_index = child_task.id();
			}

			let mut children = Rc::get_mut(child_task).unwrap().children_mut().clone();
			for child in children.iter_mut(){
				self.parse_child_task(child, child_task, parent_composite_index)?;
			}
		}else{
			if child_task.is_implements_iconditional(){
				if parent_composite_index != -1{
					self.child_conditional_index[parent_composite_index as usize].push(child_task.id());
				}
			}
		}
		Ok(())
	}

	fn initialize(&mut self, parser:&dyn IParser)->Result<(), Box<dyn std::error::Error>>{
		Ok(())
	}
}

impl IBehaviorTree for BehaviorTree{
	fn id(&self)->u64{
		self.id
	}

	fn enable(&mut self, parser:&dyn IParser)->Result<(), Box<dyn std::error::Error>>{
		if self.is_running{
			return Err("BehaviorTree is already running".into());
		}

		self.initialize(parser)?;

		for task in self.task_list.iter_mut(){
			let action = Rc::get_mut(task).unwrap();
			if action.is_implements_iaction(){
				if action.is_sync_to_client(){
					action.set_sync_data_collector(SyncDataCollector::new());
				};
			}
		}

		let mut task_list = self.task_list.clone();
		for task in task_list.iter_mut(){
			let task = Rc::get_mut( task).unwrap();
			if !task.disabled(){
				task.on_awake(self);
			}
		}

		self.execution_status = TaskStatus::Inactive;
		self.is_running = true;
		
		let now_timestamp_in_milli = self.clock.timestamp_in_mill();
		self.runtime_event_handle.post_initialize(self, now_timestamp_in_milli);
		self.initialize_first_stack_and_first_task = true;

		Ok(())
	}

	fn disable(&mut self)->Result<(), Box<dyn std::error::Error>>{
		Ok(())
	}

	fn update(&mut self){

	}

	fn is_runnning(&self)->bool{
		self.is_running
	}

	fn unit(&self)->Rc<Box<dyn IUnit>>{
		self.unit.clone()
	}

	fn rebuild_sync(&self, collector:&dyn IRebuildSyncDataCollector){

	}

	fn clock(&self)->Rc<Box<dyn IClock>>{
		self.clock.clone()
	}
}
