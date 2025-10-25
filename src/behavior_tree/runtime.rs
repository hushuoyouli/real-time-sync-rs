use std::{collections::HashMap, rc::Rc};
use super::consts::{TaskStatus, AbortType};
use super::interface::{IAction, IConditional, IComposite, IDecorator};

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
    taskList: Vec<TaskType>,
    parentIndex:Vec<u32>,

    childrenIndex :Vec<Vec<u32>>,
	relativeChildIndex:Vec<u32>,

    activeStack :Vec<Rc<Box<RunningStack>>>,
	nonInstantTaskStatus:Vec<TaskStatus>,
	conditionalReevaluate:Vec<Rc<Box<ConditionalReevaluate>>>,
	conditionalReevaluateMap:HashMap<u32, Rc<Box<ConditionalReevaluate>>>,

    isRunning:bool,
	initializeFirstStackAndFirstTask:bool, //	是否需要初始化第一个执行栈和第一个任务
	executionStatus:TaskStatus,
	config:Vec<u8>,
	//unit                             iface.IUnit
	//rootTask                         iface.ITask
	//clock                            iface.IClock
	stackID:u32,
    stackID2StackData:HashMap<u32, Rc<Box<RunningStack>>>,

	//taskDatas map[int]*iface.TaskRuntimeData

	stackID2ParallelTaskID:HashMap<u32, u32>,
	parallelTaskID2StackIDs:HashMap<u32, Vec<u32>>,

	//runtimeEventHandle    iface.IRuntimeEventHandle
	initializeForBaseFlag:bool
}

impl BehaviorTree{
	pub fn new() -> Self{
		Self{
			id: 0,
			taskList: Vec::new(),
			parentIndex: Vec::new(),
			childrenIndex: Vec::new(),
			relativeChildIndex: Vec::new(),
			activeStack: Vec::new(),
			nonInstantTaskStatus: Vec::new(),
			conditionalReevaluate: Vec::new(),
			conditionalReevaluateMap: HashMap::new(),
			isRunning: false,
			initializeFirstStackAndFirstTask: false,
			executionStatus: TaskStatus::Inactive,
			config: Vec::new(),
			stackID: 0,
			stackID2StackData: HashMap::new(),
			stackID2ParallelTaskID: HashMap::new(),
			parallelTaskID2StackIDs: HashMap::new(),
			initializeForBaseFlag: false,
		}
	}
    
}