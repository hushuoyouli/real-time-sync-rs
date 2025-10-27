use std::{collections::HashMap, rc::{Rc,Weak}};
use super::consts::{TaskStatus, AbortType};
use super::interface::{IUnit, IClock, ITaskProxy,IBehaviorTree, 
	SyncDataCollector, RunningStack, TaskRuntimeData, 
	IRuntimeEventHandle, IParser,TaskAddData, IRebuildSyncDataCollector, IAction, 
	IConditional, RealTaskType, IParentTask,IDecorator};


pub struct EmptyAction;
impl IAction for EmptyAction {
	fn on_update(&mut self, task_proxy:&dyn ITaskProxy, behavior_tree:&dyn IBehaviorTree)->TaskStatus{
		TaskStatus::Success
	}
}


pub struct TaskProxy{
	corresponding_type:String,
	id:i32,
	name:String,
	disabled:bool,
	unit:Rc<Box<dyn IUnit>>,

	//	IComposite专用
	abort_type:AbortType,
	children:Vec<Rc<Box<dyn ITaskProxy>>>,
	real_task:RealTaskType,
	sync_data_collector:Option<Rc<Box<SyncDataCollector>>>,
	parent:Option<Weak<Box<dyn ITaskProxy>>>,
	owner:Option<Weak<Box<dyn IBehaviorTree>>>,
}

impl TaskProxy{
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
			owner:None,
		}
	}
}

#[allow(unused_variables)]
impl ITaskProxy for TaskProxy {
	fn set_owner(&mut self, owner:Option<Weak<Box<dyn IBehaviorTree>>>){
		self.owner = owner;
	}

	fn owner(&self)->Option<Weak<Box<dyn IBehaviorTree>>>{
		self.owner.clone()
	}

	fn set_parent(&mut self, parent:Option<Weak<Box<dyn ITaskProxy>>>){
		self.parent = parent;
	}

	fn parent(&self)->Option<Weak<Box<dyn ITaskProxy>>>{
		self.parent.clone()
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

	fn unit(&self)->Rc<Box<dyn IUnit>>{
		self.unit.clone()
	}


	fn on_awake(&mut self){
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Action(action) => action.on_awake(self, behavior_tree),
			RealTaskType::Composite(composite) => composite.on_awake(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_awake(self, behavior_tree),
			RealTaskType::Conditional(conditional) => conditional.on_awake(self, behavior_tree),
		}

		self.real_task = real_task;
	}

    fn on_start(&mut self){
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Action(action) => action.on_start(self, behavior_tree),
			RealTaskType::Composite(composite) => composite.on_start(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_start(self, behavior_tree),
			RealTaskType::Conditional(conditional) => conditional.on_start(self, behavior_tree),
		}

		self.real_task = real_task;
	}

    fn on_end(&mut self){
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Action(action) => action.on_end(self, behavior_tree),
			RealTaskType::Composite(composite) => composite.on_end(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_end(self, behavior_tree),
			RealTaskType::Conditional(conditional) => conditional.on_end(self, behavior_tree),
		}

		self.real_task = real_task;
	}

    fn on_complete(&mut self){		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

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
	fn on_update(&mut self)->TaskStatus{
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

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
	
	fn rebuild_sync_datas(&self){
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

		match &self.real_task {
			RealTaskType::Action(action) => action.rebuild_sync_datas(self, behavior_tree),
			_ => {panic!("error");},
		}
	}
	
	fn set_sync_data_collector(&mut self, collector:Option<Rc<Box<SyncDataCollector>>>){
		match &self.real_task {
			RealTaskType::Action(_) => self.sync_data_collector = collector,
			_ => {panic!("error");},
		}
	}
	
	fn sync_data_collector(&self)->Option<Rc<Box<SyncDataCollector>>>{
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
	fn  on_child_executed1(&mut self, child_status:TaskStatus){
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_child_executed1(child_status,self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_child_executed1(child_status,self, behavior_tree),
			_ => {panic!("error");},
		}

		self.real_task = real_task;
	}

	fn  on_child_started0(&mut self){
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

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
	fn  on_child_executed2(&mut self,index:u32, child_status:TaskStatus){
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_child_executed2(index, child_status,self, behavior_tree),
			//RealTaskType::Decorator(decorator) => decorator.on_child_executed2(index, child_status,self, behavior_tree),
			_ => {panic!("error");},
		}

		self.real_task = real_task;
	}

	fn 	on_child_started1(&mut self,index:u32){
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_child_started1(index,self, behavior_tree),
			//RealTaskType::Decorator(decorator) => decorator.on_child_executed2(index, child_status,self, behavior_tree),
			_ => {panic!("error");},
		}

		self.real_task = real_task;
	}

	fn current_child_index(&self)->u32{
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

		let result = match &self.real_task {
			RealTaskType::Composite(composite) => composite.current_child_index(self, behavior_tree),
			//RealTaskType::Decorator(decorator) => decorator.on_child_executed2(index, child_status,self, behavior_tree),
			_ => {panic!("error");  },
		};
		
		result
	}

	fn can_execute(&self)->bool{
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

		match &self.real_task {
			RealTaskType::Composite(composite) => composite.can_execute(self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.can_execute(self, behavior_tree),
			_ => {panic!("error");},
		}
	}
	
	fn decorate(&mut self, status:TaskStatus)->TaskStatus{
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

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
	fn override_status1(&mut self, status:TaskStatus)->TaskStatus{
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		let result = match &mut real_task {
			RealTaskType::Composite(composite) => composite.override_status1(status,self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.override_status1(status,self, behavior_tree),
			_ => {panic!("error");},
		};

		self.real_task = real_task;
		result
	}

	fn on_conditional_abort(&mut self, index:u32){
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree = behavior_tree.as_ref();

		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		let result = match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_conditional_abort(index,self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_conditional_abort(index,self, behavior_tree),
			_ => {panic!("error");},
		};

		self.real_task = real_task;
		result
	}

	fn on_cancel_conditional_abort(&mut self, index:u32){
		let mut behavior_tree = self.owner.as_ref().unwrap().upgrade().unwrap();
		let behavior_tree = Rc::get_mut(&mut behavior_tree).unwrap();
		let behavior_tree: &dyn IBehaviorTree = behavior_tree.as_ref();

		let mut real_task =std::mem::replace(&mut self.real_task, RealTaskType::Action(Box::new(EmptyAction)));
		let result = match &mut real_task {
			RealTaskType::Composite(composite) => composite.on_cancel_conditional_abort(index,self, behavior_tree),
			RealTaskType::Decorator(decorator) => decorator.on_cancel_conditional_abort(index,self, behavior_tree),
			_ => {panic!("error");},
		};

		self.real_task = real_task;
		result
	}

	fn children(&self)->&Vec<Rc<Box<dyn ITaskProxy>>>{
		&self.children
	}

	fn children_mut(&mut self)->&mut Vec<Rc<Box<dyn ITaskProxy>>>{
		&mut self.children
	}
	
	fn add_child(&mut self, task:&Rc<Box<dyn ITaskProxy>>){
		self.children.push(task.clone());
	}

	fn abort_type(&self)->AbortType{
		self.abort_type
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
}

pub struct ConditionalReevaluate{

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

    task_list: Vec<Rc<Box<dyn ITaskProxy>>>,
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
	root_task:Option<Rc<Box<dyn ITaskProxy>>>,
	clock:Rc<Box<dyn IClock>>,                            
	stack_id:u32,
    stack_id_to_stack_data:HashMap<u32, Rc<Box<RunningStack>>>,

	task_datas:HashMap<u32, Rc<Box<TaskRuntimeData>>>,

	stack_id_to_parallel_task_id:HashMap<u32, u32>,
	parallel_task_id_to_stack_ids:HashMap<u32, Vec<u32>>,

	runtime_event_handle:Rc<Box<dyn IRuntimeEventHandle>>,
	initialize_for_base_flag:bool,
    
	
	parser:Rc<Box<dyn IParser>>,
	self_weak_ref:Option<Weak<Box<dyn IBehaviorTree>>>,
}


#[allow(unused_variables)]
impl BehaviorTree{
	pub fn new(id: u64, config:&Vec<u8>,	unit:Rc<Box<dyn IUnit>>,  clock:Rc<Box<dyn IClock>>, 
		runtime_event_handle:Rc<Box<dyn IRuntimeEventHandle>>,parser:Rc<Box<dyn IParser>>) -> Rc<Box<dyn IBehaviorTree>>{
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
			parser:parser,
			self_weak_ref:None,
		};

		let mut behavior_tree:Rc<Box<dyn IBehaviorTree>> = Rc::new(Box::new(behavior_tree));
		let self_weak_ref = Some(Rc::downgrade(&behavior_tree));
		Rc::get_mut(&mut behavior_tree).unwrap().set_self_weak_ref(self_weak_ref);
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
		let taskAddData: TaskAddData = TaskAddData::new(&self.unit);

		let root_task = self.parser.deserialize(&self.config, &taskAddData)?;
		let entry_root = EntryRoot::new();
		let mut root_proxy = TaskProxy::new("EntryRoot", "EntryRoot", &self.unit, RealTaskType::Decorator(entry_root));
		root_proxy.set_owner(self.self_weak_ref.clone());
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

	fn parse_child_task(&mut self, child_task:&mut Rc<Box<dyn ITaskProxy>>, parent_task:&Rc<Box<dyn ITaskProxy>>, mut parent_composite_index: i32)->Result<(), Box<dyn std::error::Error>>{
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
		Rc::get_mut(child_task).unwrap().set_owner(self.self_weak_ref.clone());

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

	fn initialize(&mut self)->Result<(), Box<dyn std::error::Error>>{
		Ok(())
	}
}

impl IBehaviorTree for BehaviorTree{
	fn set_self_weak_ref(&mut self, self_weak_ref:Option<Weak<Box<dyn IBehaviorTree>>>){
		self.self_weak_ref = self_weak_ref;
	}

	fn id(&self)->u64{
		self.id
	}

	fn enable(&mut self)->Result<(), Box<dyn std::error::Error>>{
		if self.is_running{
			return Err("BehaviorTree is already running".into());
		}

		self.initialize()?;

		for task in self.task_list.iter_mut(){
			let action = Rc::get_mut(task).unwrap();
			if action.is_implements_iaction(){
				if action.is_sync_to_client(){
					action.set_sync_data_collector(Some(SyncDataCollector::new()));
				};
			}
		}

		let mut task_list = self.task_list.clone();
		for task in task_list.iter_mut(){
			let task = Rc::get_mut( task).unwrap();
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
