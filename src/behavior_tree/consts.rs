#[derive(Clone, PartialEq)]
pub enum TaskStatus{
    Inactive,
	Running,
	Success,
	Failure,
}

impl TaskStatus {
    pub fn to_string(&self) -> &str {
        match self {
            TaskStatus::Inactive => "Inactive",
            TaskStatus::Running => "Running",
            TaskStatus::Success => "Success",
            TaskStatus::Failure => "Failure",
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
pub enum AbortType {
    None,
	Self_,
	LowerPriority,
	Both,
}

impl AbortType {
    pub fn to_string(&self) -> &str {
        match self {
            AbortType::None => "None",
            AbortType::Self_ => "Self_",
            AbortType::LowerPriority => "LowerPriority",
            AbortType::Both => "Both",
        }
    }
}

