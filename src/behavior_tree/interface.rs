use  std::rc::Rc;
use super::consts::TaskStatus;

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
    fn on_update(&mut self)->TaskStatus;
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
	fn send_sync_data(&self, data:Vec<u8>);
	fn rebuild_sync_datas(&self);
	fn set_sync_data_collector(&mut self, collector:SyncDataCollector);
	fn sync_data_collector(&self)->Option<SyncDataCollector>;
}

pub trait IParentTask :ITask{
    
}

pub trait ICompositeTask:IParentTask{

}

pub trait IDecoratorTask:IParentTask{

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
    
}
