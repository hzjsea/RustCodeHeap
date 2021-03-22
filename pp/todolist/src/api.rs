use std::fs;
use std::fmt::{Debug,Display};
use prettytable::*;
use serde::{Serialize,Deserialize};
use serde_json as json;
use std::ops::Drop;

pub static BACKUP_FILE: &str = "tasks.json";

#[derive(Debug,Serialize,Deserialize)]
pub struct ToDoList<T> 
    where T: Display + Serialize,
{
    pub tasks: Vec<T>,
    name: String,
}


impl <T> ToDoList<T> 
    where T: Display + Serialize 
{
    pub fn new(name: String) -> ToDoList<T>{
        ToDoList{
            tasks: Vec::new(),
            name: name
        }
    }
    
    pub fn run(&mut self, inst: Instruction<T>) {
        match inst {
            Instruction::Add(t) => self.tasks.push(t),
            Instruction::Remove(index) => {
                self.tasks.remove(index - 1);
            },
            Instruction::Modify(index, t) => self.tasks[index-1] = t,
            Instruction::Print => {
                if !self.tasks.is_empty(){
                    let mut table = Table::new();

                    for (i,s) in self.tasks.iter().enumerate(){
                        table.add_row(row![(i + 1).to_string(), s]);
                    }
                    println!("\n\n{}'s To-Do List:\n", self.name);
                    table.printstd();
                } else {
                    println!("No tasks to print for {}",self.name)
                }
            }
            
        }
    }
}

impl<T> Drop for ToDoList<T>
where  T:Display + Serialize
{
    fn drop(&mut self) {
        let userdata = json::to_string_pretty(self).unwrap();
        fs::write(BACKUP_FILE,userdata).unwrap();
    }
}


pub enum Instruction<T> {
    Add(T),
    Remove(usize),
    Modify(usize, T),
    Print,
}


