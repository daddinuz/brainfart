use std::fmt::{Debug, Display};

pub trait Show: Debug + Display {}

impl<T> Show for T where T: Debug + Display {}
