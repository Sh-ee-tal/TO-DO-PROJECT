// Serialize and Deserialize needed to be implement on struct Customer
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
    pub password: String,
    pub todo: Vec<String>,
}
//To create new customer by taking customer name and password as inputs
pub fn New_Customer(data: &mut HashMap<String, Customer>, customer_name: String, pass: String) {
    let new_customer = Customer {
        password: pass,
        todo: vec![],
    };
    data.insert(customer_name, new_customer);
}
//To read ToDo list
pub fn Read(list: &mut Vec<String>) {
    for li in list {
        println!("{}", li);
    }
}
//To add into ToDo list
pub fn Add(list: &mut Vec<String>, tbadd: String) {
    list.push(tbadd);
    for li in list {
        println!("{}", li);
    }
}
//To remove particular part from ToDo
pub fn Del(list: &mut Vec<String>, tbdel: usize) {
    list.remove(tbdel);
    for li in list {
        println!("{}", li);
    }
}
//To edit any part of ToDo
pub fn Edit(list: &mut Vec<String>, tbedit: usize, edit: String) {
    list[tbedit] = edit;
    for li in list {
        println!("{}", li);
    }
}
