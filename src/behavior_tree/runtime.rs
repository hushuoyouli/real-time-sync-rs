use std::{collections::HashMap, rc::Rc};

enum TaskStatus{
    Inactive,
	Running,
	Success,
	Failure,
}

enum TaskType{
    Action,
    Conditional,
    Composite,
    Decorator,
}

enum AbortType{
    None,
	Self_,
	LowerPriority,
    Both,
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
    taskList: Vec<Rc<Box<TaskType>>>,
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

    
}