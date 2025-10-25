use std::{collections::HashMap, rc::Rc};
use super::consts::{TaskStatus, AbortType};
use super::interface::{ITask,IAction, IConditional, IComposite, IDecorator,
	IUnit,TaskRuntimeData,IClock,IRuntimeEventHandle,TaskType,
	IParser,IBehaviorTree,IRebuildSyncDataCollector,SyncDataCollector};


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

    task_list: Vec<TaskType>,
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
	root_task:Option<Rc<Box<TaskType>>>,
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
			match task {
				TaskType::Action(action) => {
					let action = Rc::get_mut(action).unwrap();
					if action.is_sync_to_client(){
						action.set_sync_data_collector(SyncDataCollector::new());
					};
					if !action.disabled() {
						action.on_awake();
					}
				},
				TaskType::Conditional(conditional) => {
					let conditional = Rc::get_mut(conditional).unwrap();
					if !conditional.disabled() {
						conditional.on_awake();
					}
				},
				TaskType::Composite(composite) => {
					let composite = Rc::get_mut(composite).unwrap();
					if !composite.disabled() {
						composite.on_awake();
					}
				},
				TaskType::Decorator(decorator) => {
					let decorator = Rc::get_mut(decorator).unwrap();
					if !decorator.disabled() {
						decorator.on_awake();
					}
				},
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
