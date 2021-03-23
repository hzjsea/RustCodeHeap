use std::fmt::{Debug,Display};
pub struct TodoList<T>
    where T: Display
{
    desction: String,
    event: String,
    icon: String,
}

