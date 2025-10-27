use  std::rc::{Rc, Weak};
use super::consts::{TaskStatus, AbortType};

pub trait IUnit {
	fn id(&self) -> u64;
}

pub trait IClock{
	fn timestamp_in_mill(&self)->u64;
}





