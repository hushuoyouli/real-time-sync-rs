use std::{collections::HashMap, rc::{Rc, Weak}};
use crate::behavior_tree;

use super::consts::{TaskStatus, AbortType};
use super::interface::{IUnit,TaskRuntimeData,IClock,IRuntimeEventHandle,TaskType,
	IParser,IBehaviorTree,IRebuildSyncDataCollector,SyncDataCollector};


pub trait IAction {
	fn on_awake(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_start(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_update(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_end(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_complete(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}

	//	默认不需要同步
	fn is_sync_to_client(&self)->bool{
		false
	}
	fn rebuild_sync_datas(&self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
}

pub struct EmptyAction;
impl IAction for EmptyAction {}

pub trait IConditional{
	fn on_awake(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_start(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_update(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_end(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
    fn on_complete(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree){}
}


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
	fn can_execute(&mut self, task_proxy:&TaskProxy, behavior_tree:&BehaviorTree)->bool;
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
}

pub trait IDecorator:IParentTask{
}

pub enum RealTaskType{
	Action(Box<dyn IAction>),
	Conditional(Box<dyn IConditional>),
	Composite(Box<dyn IComposite>),
	Decorator(Box<dyn IDecorator>),
}


pub struct TaskProxy{
	corresponding_type:String,
	id:u32,
	disabled:bool,
	unit:Rc<Box<dyn IUnit>>,

	//	IComposite专用
	abort_type:AbortType,
	children:Vec<Rc<Box<TaskProxy>>>,
	real_task:RealTaskType,
	behavior_tree:Weak<Box<BehaviorTree>>,
}

impl TaskProxy {
	pub fn new(corresponding_type:&str, unit:&Rc<Box<dyn IUnit>>,real_task:RealTaskType, behavior_tree:Weak<Box<BehaviorTree>>) -> Self{
		Self{
			corresponding_type: corresponding_type.to_string(),
			id:0,
			disabled: false,
			unit: unit.clone(),
			abort_type: AbortType::None,
			children: Vec::new(),
			real_task:real_task,
			behavior_tree: behavior_tree,
		}
	}


	pub fn corresponding_type(&self)->String{
		self.corresponding_type.clone()
	}

	pub fn id(&self)->u32{
		self.id
	}

	pub fn set_id(&mut self, id:u32){
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


	pub fn on_awake(&mut self){
		let mut behavior_tree = self.behavior_tree.upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Action(action) => action.on_awake(self, behavior_tree),
			RealTaskType::Composite(composite) => composite.on_awake(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_awake(self, behavior_tree),
			RealTaskType::Conditional(conditional) => conditional.on_awake(self, behavior_tree),
		}

		self.real_task = real_task;
	}

    pub fn on_start(&mut self){

	}

    pub fn on_end(&mut self){

	}

    pub fn on_complete(&mut self){

	}

	//提供给Action与Conditional使用
	pub fn on_update(&mut self)->TaskStatus{
		TaskStatus::Inactive
	}

	//is_sync_to_client,rebuild_sync_datas,set_sync_data_collector,sync_data_collector这几个接口是提供给action用于同步的
	pub fn is_sync_to_client(&self)->bool{
		false
	}
	
	pub fn rebuild_sync_datas(&self){

	}
	
	pub fn set_sync_data_collector(&mut self, collector:SyncDataCollector){

	}
	
	pub fn sync_data_collector(&self)->Option<SyncDataCollector>{
		None
	}

	//	IParentTask接口
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
	pub fn  on_child_executed1(&mut self, child_status:TaskStatus){

	}

	pub fn  on_child_started0(&mut self){

	}
	//	CanRunParallelChildren	为true的时候调用
	pub fn  on_child_executed2(&mut self,index:u32, child_status:TaskStatus){

	}

	pub fn 	on_child_started1(&mut self,index:u32){

	}

	pub fn current_child_index(&self)->u32{
		0
	}

	pub fn can_execute(&mut self)->bool{
		false
	}
	
	pub fn decorate(&mut self, status:TaskStatus)->TaskStatus{
		TaskStatus::Inactive
	}

	/*
		TODO：这个部分还需要继续了解
		OverrideStatus
	*/
	pub fn override_status0(&mut self)->TaskStatus{
		TaskStatus::Inactive
	}
	pub fn override_status1(&mut self, status:TaskStatus)->TaskStatus{
		TaskStatus::Inactive
	}

	pub fn on_conditional_abort(&mut self, index:u32){

	}

	pub fn on_cancel_conditional_abort(&mut self, index:u32){

	}

	pub fn children(&self)->Vec<Rc<Box<TaskProxy>>>{
		self.children.clone()
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

pub struct BehaviorTree{
    id: u64,

    task_list: Vec<Rc<Box<TaskProxy>>>,
    parent_index:Vec<u32>,

    children_index :Vec<Vec<u32>>,
	relative_child_index:Vec<u32>,

    active_stack :Vec<Rc<Box<RunningStack>>>,
	non_instant_task_status:Vec<TaskStatus>,
	conditional_reevaluate:Vec<Rc<Box<ConditionalReevaluate>>>,
	conditional_reevaluate_map:HashMap<u32, Rc<Box<ConditionalReevaluate>>>,

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

		for task in self.task_list.iter_mut(){
			let task = Rc::get_mut(task).unwrap();
			if !task.disabled(){
				task.on_awake();
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
