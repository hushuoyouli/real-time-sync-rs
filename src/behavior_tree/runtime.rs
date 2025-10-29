use std::{collections::HashMap, rc::{Rc,Weak}, cell::RefCell};
use crate::behavior_tree;

use super::consts::{TaskStatus, AbortType};
use super::interface::{IClock, ITaskProxy,IBehaviorTree, 
	SyncDataCollector, RunningStack, TaskRuntimeData, 
	IRuntimeEventHandle, IParser,TaskAddData, IRebuildSyncDataCollector, IAction, 
	IConditional, RealTaskType, IParentTask,IDecorator,StackRuntimeData};


pub struct EmptyAction;
impl IAction for EmptyAction {
	fn on_update(&mut self, task_proxy:&mut dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->TaskStatus{
		TaskStatus::Success
	}
}


pub struct TaskProxy{
	corresponding_type:String,
	id:i32,
	name:String,
	disabled:bool,

	//	IComposite专用
	abort_type:AbortType,
	children:Vec<Rc<RefCell<Box<dyn ITaskProxy>>>>,
	real_task:RealTaskType,
	sync_data_collector:Option<Rc<RefCell<Box<SyncDataCollector>>>>,
	instant:bool,
}

impl TaskProxy{
	pub fn new(corresponding_type:&str, name:&str,real_task:RealTaskType) -> Self{
		Self{
			corresponding_type: corresponding_type.to_string(),
			id:0,
			name:name.to_string(),
			disabled: false,
			abort_type: AbortType::None,
			children: Vec::new(),
			real_task:real_task,
			sync_data_collector: None,
			instant:true,
		}
	}
}

#[allow(unused_variables)]
impl ITaskProxy for TaskProxy {
	fn set_instant(&mut self, instant:bool){
		self.instant = instant;
	}

	fn instant(&self)->bool{
		self.instant
	}

	fn initialize_variables(&mut self)->Result<(), Box<dyn std::error::Error>>{
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		let result =
		match &mut real_task {
			RealTaskType::Action(action) => action.initialize_variables(self),
			RealTaskType::Composite(composite) => composite.initialize_variables(self),
			RealTaskType::Decorator(decorator) => decorator.initialize_variables(self),
			RealTaskType::Conditional(conditional) => conditional.initialize_variables(self),
		};
		self.real_task = real_task;
		result
	}	

	fn corresponding_type(&self)->String{
		self.corresponding_type.clone()
	}

	fn name(&self)->String{
		self.name.clone()
	}

	fn id(&self)->i32{
		self.id
	}

	fn set_id(&mut self, id:i32){
		self.id = id;
	}

	//是否无效
	fn disabled(&self)->bool{
		self.disabled
	}

	fn set_disabled(&mut self, disabled:bool){
		self.disabled = disabled;
	}

	fn on_awake(&mut self, behavior_tree:&dyn IBehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Action(action) => action.on_awake(self, behavior_tree),
			RealTaskType::Composite(composite) => composite.on_awake(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_awake(self, behavior_tree),
			RealTaskType::Conditional(conditional) => conditional.on_awake(self, behavior_tree),
		}

		self.real_task = real_task;
	}

    fn on_start(&mut self, behavior_tree:&dyn IBehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Action(action) => action.on_start(self, behavior_tree),
			RealTaskType::Composite(composite) => composite.on_start(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_start(self, behavior_tree),
			RealTaskType::Conditional(conditional) => conditional.on_start(self, behavior_tree),
		}

		self.real_task = real_task;
	}

    fn on_end(&mut self, behavior_tree:&dyn IBehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Action(action) => action.on_end(self, behavior_tree),
			RealTaskType::Composite(composite) => composite.on_end(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_end(self, behavior_tree),
			RealTaskType::Conditional(conditional) => conditional.on_end(self, behavior_tree),
		}

		self.real_task = real_task;
	}

    fn on_complete(&mut self, behavior_tree:&dyn IBehaviorTree){
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
	fn on_update(&mut self, behavior_tree:&dyn IBehaviorTree)->TaskStatus{		
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
	fn is_sync_to_client(&self)->bool{
		match &self.real_task {
			RealTaskType::Action(action) => action.is_sync_to_client(),
			_ => {
					panic!("error");
					false
				},
		}
	}
	
	fn rebuild_sync_datas(&self, behavior_tree:&dyn IBehaviorTree){
		match &self.real_task {
			RealTaskType::Action(action) => action.rebuild_sync_datas(self, behavior_tree),
			_ => {panic!("error");},
		}
	}
	
	fn set_sync_data_collector(&mut self, collector:Option<Rc<RefCell<Box<SyncDataCollector>>>>){
		match &self.real_task {
			RealTaskType::Action(_) => self.sync_data_collector = collector,
			_ => {panic!("error");},
		}
	}
	
	fn sync_data_collector(&self)->Option<Rc<RefCell<Box<SyncDataCollector>>>>{
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
	fn  on_child_executed1(&mut self, child_status:TaskStatus, behavior_tree:&dyn IBehaviorTree){

		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_child_executed1(child_status,self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_child_executed1(child_status,self, behavior_tree),
			_ => {panic!("error");},
		}

		self.real_task = real_task;
	}

	fn  on_child_started0(&mut self, behavior_tree:&dyn IBehaviorTree){
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
	fn  on_child_executed2(&mut self,index:u32, child_status:TaskStatus, behavior_tree:&dyn IBehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_child_executed2(index, child_status,self, behavior_tree),
			//RealTaskType::Decorator(decorator) => decorator.on_child_executed2(index, child_status,self, behavior_tree),
			_ => {panic!("error");},
		}

		self.real_task = real_task;
	}

	fn 	on_child_started1(&mut self,index:u32, behavior_tree:&dyn IBehaviorTree){


		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_child_started1(index,self, behavior_tree),
			//RealTaskType::Decorator(decorator) => decorator.on_child_executed2(index, child_status,self, behavior_tree),
			_ => {panic!("error");},
		}

		self.real_task = real_task;
	}

	fn current_child_index(&self, behavior_tree:&dyn IBehaviorTree)->u32{
		let result = match &self.real_task {
			RealTaskType::Composite(composite) => composite.current_child_index(self, behavior_tree),
			//RealTaskType::Decorator(decorator) => decorator.on_child_executed2(index, child_status,self, behavior_tree),
			_ => {panic!("error");  },
		};
		
		result
	}

	fn can_execute(&self, behavior_tree:&dyn IBehaviorTree)->bool{
		match &self.real_task {
			RealTaskType::Composite(composite) => composite.can_execute(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.can_execute(self, behavior_tree),
			_ => {panic!("error");},
		}
	}
	
	fn decorate(&mut self, status:TaskStatus, behavior_tree:&dyn IBehaviorTree)->TaskStatus{
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
	fn override_status1(&mut self, status:TaskStatus, behavior_tree:&dyn IBehaviorTree)->TaskStatus{
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		let result = match &mut real_task {
			RealTaskType::Composite(composite) => composite.override_status1(status,self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.override_status1(status,self, behavior_tree),
			_ => {panic!("error");},
		};

		self.real_task = real_task;
		result
	}

	fn on_conditional_abort(&mut self, index:u32, behavior_tree:&dyn IBehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		let result = match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_conditional_abort(index,self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_conditional_abort(index,self, behavior_tree),
			_ => {panic!("error");},
		};

		self.real_task = real_task;
		result
	}

	fn on_cancel_conditional_abort(&mut self, behavior_tree:&dyn IBehaviorTree){
		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		let result = match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_cancel_conditional_abort(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_cancel_conditional_abort(self, behavior_tree),
			_ => {panic!("error");},
		};

		self.real_task = real_task;
		result
	}

	fn children(&self)->&Vec<Rc<RefCell<Box<dyn ITaskProxy>>>>{
		&self.children
	}

	fn children_mut(&mut self)->&mut Vec<Rc<RefCell<Box<dyn ITaskProxy>>>>{
		&mut self.children
	}
	
	fn add_child(&mut self, task:&Rc<RefCell<Box<dyn ITaskProxy>>>){
		self.children.push(task.clone());
	}

	fn abort_type(&self)->AbortType{
		self.abort_type.clone()
	}
	
	fn set_abort_type(&mut self, abort_type:AbortType){
		self.abort_type = abort_type;
	}

	//是否是action
	fn is_implements_iaction(&self)-> bool{
		match self.real_task {
			RealTaskType::Action(_) => true,
			_ => false,
		}
	}

	//是否是composite
	fn is_implements_icomposite(&self)-> bool{
		match self.real_task {
			RealTaskType::Composite(_) => true,
			_ => false,
		}
	}
	
	//是否是decorator
	fn is_implements_idecorator(&self)-> bool{
		match self.real_task {
			RealTaskType::Decorator(_) => true,
			_ => false,
		}
	}

	//是否是conditional
	fn is_implements_iconditional(&self)-> bool{
		match self.real_task {
			RealTaskType::Conditional(_) => true,
			_ => false,
		}
	}

	//是否是parent task
	fn is_implements_iparenttask(&self)-> bool{
		match self.real_task {
			RealTaskType::Composite(_) => true,
			RealTaskType::Decorator(_) => true,
			_ => false,
		}
	}

	fn send_sync_data(&mut self, data:Vec<u8>){
		if let Some(collector) = self.sync_data_collector.as_mut(){
			collector.borrow_mut().add_data(data.clone());
		}
	}
}

pub struct ConditionalReevaluate{
	pub index:i32,
	pub task_status:TaskStatus,
	pub composite_index:i32,
}

impl ConditionalReevaluate{
	pub fn new(index:i32, task_status:TaskStatus, composite_index:i32) -> Self{
		Self{
			index,
			task_status,
			composite_index,
		}
	}

	pub fn initialize(&mut self, index:i32, task_status:TaskStatus, composite_index:i32){
		self.index = index;
		self.task_status = task_status;
		self.composite_index = composite_index;
	}
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
	fn on_awake(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
		self.execution_status = TaskStatus::Inactive;
	}	

	fn can_execute(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->bool {
		if let TaskStatus::Failure = self.execution_status {
			true
		}else{
			false
		}
	}

	fn current_child_index(&self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->u32{
		0
	}

	fn on_end(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree){
		self.execution_status = TaskStatus::Inactive;
	}

	fn  on_child_executed1(&mut self, child_status:TaskStatus, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree) {
		self.execution_status = child_status;
	}
}

impl IDecorator for EntryRoot {
}

pub struct BehaviorTree{
    id: u64,

    task_list: Vec<Weak<RefCell<Box<dyn ITaskProxy>>>>,
    parent_index:Vec<i32>,

    children_index :Vec<Vec<i32>>,
	relative_child_index:Vec<i32>,

    active_stack :Vec<Rc<RefCell<Box<RunningStack>>>>,
	non_instant_task_status:Vec<TaskStatus>,
	conditional_reevaluate:Vec<Rc<RefCell<Box<ConditionalReevaluate>>>>,
	conditional_reevaluate_map:HashMap<i32, Rc<RefCell<Box<ConditionalReevaluate>>>>,

	parent_composite_index:Vec<i32>,
	child_conditional_index:Vec<Vec<i32>>,

    is_running:bool,
	initialize_first_stack_and_first_task:bool, //	是否需要初始化第一个执行栈和第一个任务
	execution_status:TaskStatus,
	config:Vec<u8>,
	root_task:Option<Rc<RefCell<Box<dyn ITaskProxy>>>>,
	clock:Weak<RefCell<Box<dyn IClock>>>,                            
	stack_id:usize,
    stack_id_to_stack_data:HashMap<usize, Rc<RefCell<Box<StackRuntimeData>>>>,

	task_datas:HashMap<i32, Rc<RefCell<Box<TaskRuntimeData>>>>,

	stack_id_to_parallel_task_id:HashMap<u32, u32>,
	parallel_task_id_to_stack_ids:HashMap<i32, Vec<u32>>,

	runtime_event_handle:Box<dyn IRuntimeEventHandle>,
	initialize_for_base_flag:bool,
    
	parser:Weak<RefCell<Box<dyn IParser>>>,
	task_execute_id:u32,
	unit_id:u64,
}


#[allow(unused_variables)]
impl BehaviorTree{
	pub fn new(id: u64, config:&Vec<u8>,	unit_id:u64,  clock:&Weak<RefCell<Box<dyn IClock>>>, 
		runtime_event_handle:Box<dyn IRuntimeEventHandle>,parser:Weak<RefCell<Box<dyn IParser>>>) -> Rc<RefCell<Box<dyn IBehaviorTree>>>{
		let behavior_tree = Self{
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
			unit_id:unit_id,
			root_task:None,
			clock:clock.clone(),
			stack_id: 0,
			stack_id_to_stack_data: HashMap::new(),
			task_datas: HashMap::new(),
			stack_id_to_parallel_task_id: HashMap::new(),
			parallel_task_id_to_stack_ids: HashMap::new(),
			runtime_event_handle: runtime_event_handle,
			initialize_for_base_flag: false,
			parser:parser,
			task_execute_id:1,
		};

		let behavior_tree:Rc<RefCell<Box<dyn IBehaviorTree>>> = Rc::new(RefCell::new(Box::new(behavior_tree)));
		behavior_tree
	}



	fn initialize_for_base(&mut self) ->Result<(), Box<dyn std::error::Error>>{
		self.task_list.clear();
		self.parent_index.clear();
		self.children_index.clear();
		self.relative_child_index.clear();
		self.parent_composite_index.clear();
		self.child_conditional_index.clear();
		self.root_task = None;
		let mut task_add_data: TaskAddData = TaskAddData::new();

		let parser =self.parser.upgrade().unwrap();
		let parser = parser.borrow();


		let root_task = parser.deserialize(&self.config, &mut task_add_data)?;
		let entry_root = EntryRoot::new();
		let mut root_proxy = TaskProxy::new("EntryRoot", "EntryRoot", RealTaskType::Decorator(entry_root));
		root_proxy.add_child(&root_task);
				
		self.root_task = Some(Rc::new(RefCell::new(Box::new(root_proxy))));
		self.task_list.push(Rc::downgrade(&self.root_task.clone().unwrap()));
		self.parent_index.push(-1);
		self.parent_composite_index.push(-1);
		self.child_conditional_index.push(Vec::with_capacity(10));
		self.relative_child_index.push(-1);
		let mut parent_composite_index = -1;

		self.root_task.as_mut().unwrap().borrow_mut().set_id(0);
		//Rc::get_mut(self.root_task.as_mut().unwrap()).unwrap().set_id(0);

		if self.root_task.as_mut().unwrap().borrow_mut().is_implements_iparenttask(){
			if self.root_task.as_mut().unwrap().borrow_mut().is_implements_icomposite(){
				parent_composite_index = self.root_task.as_mut().unwrap().borrow_mut().id();
			}

			let mut parent_task = self.root_task.as_mut().unwrap().clone();
			let mut children = parent_task.borrow_mut().children_mut().clone();


			for child in children.iter_mut(){
				self.parse_child_task(child, &parent_task, parent_composite_index)?;
			}
		}
		
		Ok(())
	}

	fn parse_child_task(&mut self, child_task:&mut Rc<RefCell<Box<dyn ITaskProxy>>>, parent_task:&Rc<RefCell<Box<dyn ITaskProxy>>>, mut parent_composite_index: i32)->Result<(), Box<dyn std::error::Error>>{
		let index = self.task_list.len() as i32;
		let parent_index = parent_task.borrow().id();

		self.children_index[parent_index as usize].push(index);
		self.relative_child_index.push(self.children_index[parent_index as usize].len() as i32 - 1);
		self.task_list.push(Rc::downgrade(child_task));
		self.parent_index.push(parent_task.borrow().id());
		self.parent_composite_index.push(parent_composite_index);
		self.child_conditional_index.push(Vec::with_capacity(10));
		self.children_index.push(Vec::with_capacity(10));

		child_task.borrow_mut().set_id(index);
		

		if child_task.borrow().is_implements_iparenttask(){
			if child_task.borrow().is_implements_icomposite(){
				parent_composite_index = child_task.borrow().id();
			}

			let mut children = child_task.borrow_mut().children_mut().clone();
			for child in children.iter_mut(){
				self.parse_child_task(child, child_task, parent_composite_index)?;
			}
		}else{
			if child_task.borrow().is_implements_iconditional(){
				if parent_composite_index != -1{
					self.child_conditional_index[parent_composite_index as usize].push(child_task.borrow().id());
				}
			}
		}
		Ok(())
	}

	fn initialize(&mut self)->Result<(), Box<dyn std::error::Error>>{
		if !self.initialize_for_base_flag{
			self.initialize_for_base()?;
			self.initialize_for_base_flag = true;
		}

		self.stack_id = 1;
		self.active_stack.clear();
		self.conditional_reevaluate.clear();
		self.conditional_reevaluate_map.clear();
		Ok(())
	}

	fn next_stack_id(&mut self) -> usize{
		let stack_id = self.stack_id;
		self.stack_id += 1;
		stack_id
	}

	fn add_stack(&mut self) -> usize{
		let stack_id = self.next_stack_id();
		let stack_index = self.active_stack.len();
		let stack = Rc::new(RefCell::new(Box::new(RunningStack::new(stack_id,10))));
		self.active_stack.push(stack);
		self.non_instant_task_status.push(TaskStatus::Inactive);

		let timestamp_in_mill = self.clock.upgrade().as_ref().unwrap().borrow().timestamp_in_mill();
		let stack_data = StackRuntimeData::new(stack_id, timestamp_in_mill);
		self.runtime_event_handle.new_stack(self, &stack_data);
		self.stack_id_to_stack_data.insert(stack_id, Rc::new(RefCell::new(Box::new(stack_data))));
		return stack_index;
	}

	fn is_parent_task(&self, possible_parent:i32, possible_child:i32)->bool{
		let mut  parent_index;
		let mut  child_index = possible_child;

		while child_index != -1 {
			parent_index = self.parent_index[child_index as usize];
			if parent_index == possible_parent {
				return true;
			}

			child_index = parent_index;
		}

		false
	}

	fn next_task_execute_id(&mut self) -> u32{
		let task_execute_id = self.task_execute_id;
		self.task_execute_id += 1;
		task_execute_id
	}

	fn push_task(&mut self, stack_index:usize, task_index:u32, stack:&mut RunningStack, stack_data: &StackRuntimeData){
		if !self.is_running || stack_index >= self.active_stack.len() {
			return
		}
	
		if stack.len() == 0 || stack.peak() != task_index {
			stack.push(task_index);

			self.non_instant_task_status[stack_index] = TaskStatus::Running;
			
			let task = &mut self.task_list.get(task_index as usize).unwrap();
	        let task = task.clone().upgrade().unwrap();
			let mut task = task.borrow_mut();
			let task = task.as_mut();

			let now_timestamp= self.clock.upgrade().as_ref().unwrap().borrow().timestamp_in_mill();
			let task_execute_id= self.next_task_execute_id();
	
			let task_runtime_data= TaskRuntimeData::new(task.id(), now_timestamp, task_execute_id, stack_data.stack_id);
			self.task_datas.insert(task.id(), Rc::new(RefCell::new(Box::new(task_runtime_data))));
	
			//	TODO:这里需要截获初始化的数据？
			self.runtime_event_handle.pre_on_start(self, &self.task_datas.get(&task.id()).unwrap().borrow(), &stack_data, task);

			//self.runtimeEventHandle.PreOnStart(p, taskRuntimeData, stackData, task)
			if task.is_implements_iparenttask() {
				if task.can_run_parallel_children() {
					self.runtime_event_handle.parallel_pre_on_start(self, &self.task_datas.get(&task.id()).unwrap().borrow(), &stack_data, task);
				}
			}
	
			//	先清理数据
			if task.is_implements_iaction() {
				if task.is_sync_to_client() {
					let sync_data_collector = task.sync_data_collector().unwrap();
					sync_data_collector.borrow_mut().get_and_clear();
				}
			}

			task.on_start(self);
			
			if task.is_implements_iaction() {
				//action := task.(iface.IAction)
				if task.is_sync_to_client() {
					let sync_data_collector = task.sync_data_collector().unwrap();
					let datas = sync_data_collector.borrow_mut().get_and_clear();
					self.runtime_event_handle.action_post_on_start(self, &self.task_datas.get(&task.id()).unwrap().borrow(), &stack_data, task, datas);
				}
			}
	
			if task.is_implements_iparenttask() {
				//	可以并发的父节点有特殊处理
				
				if task.can_run_parallel_children() {
					self.parallel_task_id_to_stack_ids.insert(task.id(), Vec::new());
				}

				 
				if task.is_implements_icomposite() {
					match task.abort_type() {
						AbortType::None => (),
						_ => {
							let mut conditional_reevaluates = self.conditional_reevaluate.clone();
						    for conditional_reevaluate in  conditional_reevaluates.iter_mut(){
								let mut conditional_reevaluate = conditional_reevaluate.borrow_mut();
								if self.is_parent_task(task.id(), conditional_reevaluate.index) {
									conditional_reevaluate.composite_index = task.id();
								}
							}
							match task.abort_type() {
								AbortType::LowerPriority => {
									let child_conditional_indexes = self.child_conditional_index[task.id() as usize].clone();
									for child_conditional_index in child_conditional_indexes.into_iter(){
										let child_conditional_indexes = child_conditional_index;
										if let Some(conditional_reevaluate) = self.conditional_reevaluate_map.get_mut(&child_conditional_indexes){
											//let conditional_reevaluate = Rc::get_mut(&mut conditional_reevaluate).unwrap();
											conditional_reevaluate.borrow_mut().composite_index = -1;
										}
									}
									()
								},
								_ => (),
							};
							()
						},
					}
				}
				
			}
		}
	}

	fn pop_task<'a>(&mut self, task_index:i32, stack_index:usize,mut status:TaskStatus, 
				pop_children:bool, task:&mut dyn ITaskProxy, stack:&mut RunningStack, 
				stack_data: &StackRuntimeData, mut parent_task:Option<&'a mut dyn ITaskProxy>, mut composite_task:Option<&'a mut dyn ITaskProxy>)->TaskStatus{
		if self.is_running{
			return status;
		}

		if stack_index >= self.active_stack.len(){
			return status;
		}

		if self.active_stack[stack_index].borrow().len() == 0||self.active_stack[stack_index].borrow().peak()!= task_index as u32{
			return status;
		}

		self.active_stack[stack_index].borrow_mut().pop();
		self.non_instant_task_status[stack_index] = TaskStatus::Inactive;

		if task.is_implements_iaction(){
			if task.is_sync_to_client(){
				task.sync_data_collector().unwrap().borrow_mut().get_and_clear();
			}
		}

		task.on_end(self);

		
		if let Some(parent_task_ref) = &mut parent_task{
			let parent_index = self.parent_index[task_index as usize];
			if task.is_implements_iconditional(){
				let composite_parent_index = self.parent_composite_index[task_index as usize];
				if composite_parent_index != -1{
					let composite_task = if composite_parent_index == parent_index{
						parent_task_ref
					}else{
						match &mut composite_task{
							Some(composite_task_ref) => composite_task_ref,
							None => parent_task_ref,
						}
					};

					if composite_task.abort_type() != AbortType::None{
						let mut composite = -1;
						if composite_task.abort_type() != AbortType::LowerPriority{
							composite = composite_parent_index;
						}

						match self.conditional_reevaluate_map.get(&task_index){
							Some(conditional_reevaluate) => {
								conditional_reevaluate.borrow_mut().initialize(task_index, status.clone(), composite);
								()
							},
							None => {
								let conditional_reevaluate = Rc::new(RefCell::new(Box::new(ConditionalReevaluate::new(task_index, status.clone(), composite))));
								self.conditional_reevaluate_map.insert(task_index, conditional_reevaluate.clone());
								self.conditional_reevaluate.push(conditional_reevaluate.clone());
								()
							},
						}
					}

				}
			}

			if !parent_task.as_ref().unwrap().can_run_parallel_children(){
				parent_task.as_mut().unwrap().on_child_executed1(status.clone(), self);
				status = parent_task.as_mut().unwrap().decorate(status, self);
			}else{
				parent_task.as_mut().unwrap().on_child_executed2(self.relative_child_index[task_index as usize] as u32, status.clone(), self);
			}
		}

		if task.is_implements_iparenttask(){
			if task.is_implements_icomposite(){
				if task.abort_type() != AbortType::Self_|| task.abort_type() != AbortType::None{
					self.remove_child_conditional_reevaluate(task_index);
				}else if task.abort_type() != AbortType::LowerPriority|| task.abort_type() != AbortType::Both{
					if self.parent_composite_index[task_index as usize] == -1{
						self.remove_child_conditional_reevaluate(task_index);
					}else{
						let mut conditional_reevaluates = self.conditional_reevaluate.clone();
						for conditional_reevaluate in conditional_reevaluates{
							if self.is_parent_task(task_index, conditional_reevaluate.borrow().index){
								conditional_reevaluate.borrow_mut().composite_index = self.parent_composite_index[task_index as usize];
							}
						}
					}
				}
			}
		}



		status
	}

	fn reevaluate_conditional_tasks(&mut self){
		let mut update_condition_indexes:Vec<Rc<RefCell<Box<ConditionalReevaluate>>>> = Vec::with_capacity(10);
		//updateConditionIndexes := util.NewList[*ConditionalReevaluate](10)
		let  mut conditional_reevaluatees = self.conditional_reevaluate.clone();
		let len = conditional_reevaluatees.len();
		for i in (0..len).rev(){
			let conditional_reevaluate: &mut Rc<RefCell<Box<ConditionalReevaluate>>> = &mut conditional_reevaluatees[i];
			if conditional_reevaluate.borrow().composite_index != -1{
				let condition_index = conditional_reevaluate.borrow().index;
				let condition_status = conditional_reevaluate.borrow().task_status.clone();

				let condition_task =&mut self.task_list[condition_index as usize];
				let condition_task = condition_task.upgrade().unwrap();
				let mut condition_task = condition_task.borrow_mut();
				//let condition_task = condition_task.borrow().as_ref();

				if condition_task.on_update(self) != condition_status {
					let composite_index = conditional_reevaluate.borrow().composite_index;
					for j in (0..self.active_stack.len()).rev(){
						if self.active_stack[j].as_ref().borrow().len() > 0{
							let task_index = self.active_stack[j].as_ref().borrow().peak();
							let mut task_index = task_index as i32;
							if !self.is_parent_task(composite_index, task_index){
								continue;
							}

							let stack_count = self.active_stack.len();
							while task_index != -1 && task_index != composite_index && self.active_stack.len() == stack_count {
								let status = TaskStatus::Failure;
								let task = &mut self.task_list[task_index as usize].upgrade().unwrap();
								let mut task = task.borrow_mut();
								let task = task.as_mut();
								let stack = self.active_stack[j].clone();
								let mut stack = stack.borrow_mut();
								let stack_data = self.stack_id_to_stack_data.get(&stack.stack_id).unwrap().clone();
								let stack_data = stack_data.borrow();
								let stack_data = stack_data.as_ref();
								self.pop_task(task_index, j, status, false,  task, stack.as_mut(), stack_data);
								task_index = self.parent_index[task_index as usize];
							}

							for j in (i..self.conditional_reevaluate.len()).rev(){
								let j_conditional_reval = self.conditional_reevaluate[j].clone();
								if self.is_parent_task(composite_index, j_conditional_reval.borrow().index) {
									let j_index = j_conditional_reval.borrow().index;
									self.conditional_reevaluate_map.remove(&j_index);
									self.conditional_reevaluate.remove(j);
								}
							}

							//	原先abort过的要设置为原位
							for j in (0..update_condition_indexes.len()).rev(){
								let conditional_reevaluate = &update_condition_indexes[j];
								if self.is_parent_task(composite_index, conditional_reevaluate.borrow().index) {
									let mut task_index = self.parent_index[conditional_reevaluate.borrow().index as usize];
									while task_index != -1 && task_index != conditional_reevaluate.borrow().composite_index {
										let task = &mut self.task_list[task_index as usize];
										let task = task.upgrade().unwrap();
										let mut task = task.borrow_mut();

										task.on_cancel_conditional_abort(self);
										task_index = self.parent_index[task_index as usize];
									}
								}

								update_condition_indexes.remove(j as usize);
							}

							update_condition_indexes.push(conditional_reevaluate.clone());
							//是否需要把当前的conditionalReevaluate也删除掉？需要
							self.conditional_reevaluate_map.remove(&condition_index);
							self.conditional_reevaluate.remove(i);

							let mut conditional_parent_indexes :Vec<i32> = Vec::with_capacity(10);
							let mut parent_index = condition_index;
							while parent_index != composite_index {
								parent_index = self.parent_index[parent_index as usize];
								conditional_parent_indexes.push(parent_index);
							}

							for j in (0..conditional_parent_indexes.len()).rev(){
								let parent_task = &mut self.task_list[conditional_parent_indexes[j] as usize];
								let mut parent_task = parent_task.upgrade().unwrap();

								if j == 0 {
									parent_task.borrow_mut().on_conditional_abort(self.relative_child_index[condition_index as usize] as u32, self);
								}else{
									parent_task.borrow_mut().on_conditional_abort(self.relative_child_index[conditional_parent_indexes[j - 1] as usize] as u32, self);
								}
							}
						}
					}
				}
			}
		}
	}

	fn remove_stack(&mut self, stack_index:usize, stack:&mut RunningStack,stack_data: &StackRuntimeData) {
		if stack_index < self.active_stack.len() {
			let now_timestamp = self.clock.upgrade().as_ref().unwrap().borrow().timestamp_in_mill();
			if self.stack_id_to_parallel_task_id.contains_key(&(stack_data.stack_id as u32)) {
				let parallel_task_id = *self.stack_id_to_parallel_task_id.get(&(stack_data.stack_id as u32)).unwrap();
				let task_runtime_data = self.task_datas.get(&(parallel_task_id as i32)).unwrap().clone();
				let task_runtime_data = task_runtime_data.borrow();
				let task_runtime_data = task_runtime_data.as_ref();

				let parent_stack_data = self.stack_id_to_stack_data.get(&task_runtime_data.active_stack_id).unwrap();
				let parent_stack_data = parent_stack_data.borrow();
				let task = self.task_list[task_runtime_data.task_id as usize].clone().upgrade().unwrap();
				self.runtime_event_handle.parallel_remove_child_stack(self, task_runtime_data, parent_stack_data.as_ref(), task.borrow().as_ref(), &stack_data, now_timestamp);
				
				self.stack_id_to_parallel_task_id.remove(&(stack_data.stack_id as u32));
                let old_parallel_task_id_to_stack_ids = self.parallel_task_id_to_stack_ids.get_mut(&(parallel_task_id as i32)).unwrap().clone();
				self.parallel_task_id_to_stack_ids.get_mut(&(parallel_task_id as i32)).unwrap().clear();
				for stack_id in old_parallel_task_id_to_stack_ids.iter(){
					if (*stack_id as usize) != stack_data.stack_id {
						self.parallel_task_id_to_stack_ids.get_mut(&(parallel_task_id as i32)).unwrap().push(*stack_id);
					}
				}
			}

			self.runtime_event_handle.remove_stack(self, stack_data, now_timestamp);
			self.stack_id_to_stack_data.remove(&(stack_data.stack_id as usize));
			
			self.active_stack.remove(stack_index);
			self.non_instant_task_status.remove(stack_index);
		}
	}

	fn remove_child_conditional_reevaluate(&mut self, composite_index:i32){
		for i in (0..self.conditional_reevaluate.len()).rev(){
			let conditional_reevaluate = self.conditional_reevaluate[i].clone();
			if self.is_parent_task(composite_index, conditional_reevaluate.borrow().composite_index){
				let conditional_index = conditional_reevaluate.borrow().index;
				self.conditional_reevaluate_map.remove(&conditional_index);
				self.conditional_reevaluate.remove(i);
			}
		}
	}

	fn run_task(&mut self, task_index:u32, stack_index:usize, previous_status:TaskStatus, stack:&mut RunningStack, stack_data: &StackRuntimeData, task:&mut dyn ITaskProxy,task_runtime_data:&TaskRuntimeData) -> TaskStatus{
		if task_index as usize >= self.task_list.len(){
			return previous_status;
		}

		if task.disabled(){
			let parent_index = self.parent_index[task_index as usize];
			if parent_index != -1{
				let parent_task = self.task_list[parent_index as usize].upgrade().unwrap();
				let mut parent_task = parent_task.borrow_mut();
			
				if !parent_task.can_run_parallel_children(){
					parent_task.on_child_executed1(TaskStatus::Inactive, self);
				}else{
					parent_task.on_child_executed2(self.relative_child_index[task_index as usize] as u32, TaskStatus::Inactive, self);
				}
			}

			let mut status = TaskStatus::Success;
			if self.active_stack[stack_index].borrow().len() == 0{
				if stack_index == 0{
					self.remove_stack(stack_index, stack,stack_data);
					let _ =  self.disable();
					self.execution_status = status;
					status = TaskStatus::Inactive;
				}else{
					self.remove_stack(stack_index, stack,stack_data);
				}
			}

			return status;
		}

		let mut status: TaskStatus = previous_status;
		if task.instant() && (self.non_instant_task_status[stack_index] == TaskStatus::Success || self.non_instant_task_status[stack_index] == TaskStatus::Failure){
			status = self.non_instant_task_status[stack_index].clone();
			status = self.pop_task(task_index as i32, stack_index, status, true, task, stack, stack_data);
			return status;
		}

		self.push_task(stack_index, task_index, stack, stack_data);
		if task.is_implements_iparenttask(){
			status = self.run_parent_task(task_index, stack_index, status, task, stack, stack_data, task_runtime_data);
			status = task.override_status1(status, self);
		}else{
			if task.is_implements_iaction(){
				if task.is_sync_to_client(){
					if task.is_sync_to_client(){
						task.sync_data_collector().unwrap().borrow_mut().get_and_clear();
					}
				}
			}

			status = task.on_update(self);
		}

		let now_timestamp = self.clock.upgrade().as_ref().unwrap().borrow().timestamp_in_mill();
		self.runtime_event_handle.post_on_update(self, task_runtime_data, stack_data, task,now_timestamp, status.clone());

		if task.is_implements_iaction(){
			if task.is_sync_to_client(){
				let datas = task.sync_data_collector().unwrap().borrow_mut().get_and_clear();
				self.runtime_event_handle.action_post_on_update(self, task_runtime_data, stack_data, task,now_timestamp, status.clone(), datas);
			}
		}

		if status != TaskStatus::Running{
			if task.instant(){
				status = self.pop_task(task_index as i32, stack_index, status, true, task, stack, stack_data);
			}else{
				self.non_instant_task_status[stack_index] = status.clone();
				status = TaskStatus::Running;
			}
		}

		return status;
	}

	fn run_parent_task(&mut self, task_index:u32, stack_index:usize, mut status:TaskStatus, task:&mut dyn ITaskProxy, stack:&mut RunningStack, stack_data: &StackRuntimeData,task_runtime_data:&TaskRuntimeData) -> TaskStatus{
		if !task.can_run_parallel_children() || task.override_status1(TaskStatus::Running, self) != TaskStatus::Running{
			let mut child_status = TaskStatus::Inactive;
			let parent_stack = stack_index;
			let children_indexs = self.children_index[task_index as usize].clone();

			while task.can_execute(self) &&(child_status != TaskStatus::Running||task.can_run_parallel_children())&&self.is_running{
				let child_index = task.current_child_index(self);
				if task.can_run_parallel_children(){
					let child_stack_index = self.add_stack();
					let child_stack = self.active_stack[child_stack_index].clone();
					let mut child_stack = child_stack.borrow_mut();
					let child_stack = child_stack.as_mut();

					self.stack_id_to_parallel_task_id.insert(child_stack.stack_id as u32, task.id() as u32);
					self.parallel_task_id_to_stack_ids.get_mut(&(task.id() as i32)).unwrap().push(child_stack.stack_id as u32);

					let child_stack_data = self.stack_id_to_stack_data.get(&child_stack.stack_id).unwrap().clone();
					let child_stack_data = child_stack_data.borrow();
					let child_stack_data = child_stack_data.as_ref();
					self.runtime_event_handle.parallel_add_child_stack(self, task_runtime_data, stack_data, task, child_stack_data);
					task.on_child_started1(child_index, self);

					let child_task = self.task_list[children_indexs[child_index as usize] as usize].upgrade().unwrap();
					let mut child_task = child_task.borrow_mut();
					let child_task = child_task.as_mut();

					child_status = self.run_task(children_indexs[child_index as usize] as u32, child_stack_index, status,  child_stack, child_stack_data, child_task, task_runtime_data);
					status = child_status.clone();
				}else{
					task.on_child_started0(self);
					let child_task = self.task_list[children_indexs[child_index as usize] as usize].upgrade().unwrap();
					let mut child_task = child_task.borrow_mut();
					child_status = self.run_task(children_indexs[child_index as usize] as u32, stack_index, child_status,  stack, stack_data, child_task.as_mut(), task_runtime_data);
					status = child_status.clone();
				}
			}

		}

		status
	}

	/* func (p *BehaviorTree) RunTask(taskIndex, stackIndex int, previousStatus iface.TaskStatus) iface.TaskStatus { */
}

impl IBehaviorTree for BehaviorTree{
	fn id(&self)->u64{
		self.id
	}

	fn enable(&mut self)->Result<(), Box<dyn std::error::Error>>{
		if self.is_running{
			return Err("BehaviorTree is already running".into());
		}

		self.initialize()?;

		for task in self.task_list.iter_mut(){
			let action = task.upgrade().unwrap();
			let mut action =action .borrow_mut();

			//let action = Rc::get_mut(task).unwrap();
			if action.is_implements_iaction(){
				if action.is_sync_to_client(){
					action.set_sync_data_collector(Some(SyncDataCollector::new()));
				};
			}
		}

		let mut task_list = self.task_list.clone();
		for task in task_list.iter_mut(){
			let task = task.upgrade().unwrap();
			let mut task = task.borrow_mut();
			if !task.disabled(){
				task.on_awake(self);
			}
		}

		self.execution_status = TaskStatus::Inactive;
		self.is_running = true;
		
		let now_timestamp_in_milli = self.clock.upgrade().as_ref().unwrap().borrow().timestamp_in_mill();
		self.runtime_event_handle.post_initialize(self, now_timestamp_in_milli);
		self.initialize_first_stack_and_first_task = true;

		Ok(())
	}

	fn disable(&mut self)->Result<(), Box<dyn std::error::Error>>{
		Ok(())
	}

	fn update(&mut self){
		if self.is_running{
			if self.initialize_first_stack_and_first_task{
				self.add_stack();
				let stack = self.active_stack[0].clone();
				let mut stack = stack.borrow_mut();
				let stack = stack.as_mut();
				let stack_data = self.stack_id_to_stack_data.get(&stack.stack_id).unwrap().clone();
				let stack_data = stack_data.borrow();
				let stack_data = stack_data.as_ref();
				self.push_task(0,0, stack, stack_data);
				self.initialize_first_stack_and_first_task = false;
			}


			self.reevaluate_conditional_tasks();

			for j in (0..self.active_stack.len()).rev(){
				let mut status = TaskStatus::Inactive;
				let mut start_index = -1;
				let mut task_index;

				let current_stack = self.active_stack[j].clone();
				let mut stack = current_stack.borrow_mut();
				let stack = stack.as_mut();
				let stack_data = self.stack_id_to_stack_data.get(&stack.stack_id).unwrap().clone();
				let stack_data = stack_data.borrow();
				let stack_data = stack_data.as_ref();

				while status != TaskStatus::Running && j < self.active_stack.len() && self.active_stack[j].as_ref().borrow().len() > 0 && Rc::ptr_eq(&current_stack, &self.active_stack[j]) {
					task_index = self.active_stack[j].as_ref().borrow().peak();
					if !self.is_running{
						break;
					}

					if self.active_stack[j].as_ref().borrow().len() > 0 && start_index == (self.active_stack[j].as_ref().borrow().peak() as i32){
						break;
					}

					let task = self.task_list[task_index as usize].upgrade().unwrap();
					let mut task = task.borrow_mut();
					let task = task.as_mut();

					start_index = task_index as i32;
					let task_runtime_data = self.task_datas.get(&task.id()).unwrap().clone();
					let task_runtime_data = task_runtime_data.borrow();
					let task_runtime_data = task_runtime_data.as_ref();

					status = self.run_task(task_index, j, status, stack, stack_data, task, task_runtime_data);
				}
			}
		}
	}

	fn is_runnning(&self)->bool{
		self.is_running
	}

	fn unit_id(&self)-> u64{
		self.unit_id
	}

	fn rebuild_sync(&self, collector:&dyn IRebuildSyncDataCollector){

	}

	fn clock(&self)->Weak<RefCell<Box<dyn IClock>>>{
		self.clock.clone()
	}
}
