use std::{collections::HashMap, rc::Rc};
use super::consts::{TaskStatus, AbortType};
use super::interface::{IAction, IConditional, IComposite, IDecorator,IUnit,TaskRuntimeData,IClock,IRuntimeEventHandle};

enum TaskType{
    Action(Rc<Box<dyn IAction>>),
    Conditional(Rc<Box<dyn IConditional>>),
    Composite(Rc<Box<dyn IComposite>>),
    Decorator(Rc<Box<dyn IDecorator>>),
}

pub struct ConditionalReevaluate{

}


pub struct StackRuntimeData{

}

struct RunningStack{
    stackID:u32,
    stack:Vec<u32>,
    stackRuntimeData:Rc<Box<StackRuntimeData>>,
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
	root_task:Option<TaskType>,
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
	pub fn new(id: u64, config:&Vec<u8>,	unit:Rc<Box<dyn IUnit>>,  clock:Rc<Box<dyn IClock>>, runtime_event_handle:Rc<Box<dyn IRuntimeEventHandle>>) -> Rc<Box<Self>>{
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
}