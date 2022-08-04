use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufReader;
mod tasks;
use crate::tasks::{Add, Customer, Del, Edit, New_Customer, Read};
fn main() {
    //Reading from dataset
    let file = File::open("data.json").expect("Error opening fille");
    let read = BufReader::new(file);
    let mut data: HashMap<String, Customer> =
        serde_json::from_reader(read).expect("Error reading JSON file");
    //Saving dataset
    fn save_data(data: &HashMap<String, Customer>) {
        //create returns result type
        serde_json::to_writer(File::create("data.json").unwrap(), &data)
            .expect("Error saving JSON file");
    }
    //Taking customer name as input
    println!("Enter your Name");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("fail to get customer name");
    //trim() needed to remove line space
    let trim_name = name.as_str().trim().to_string();
    //To get values from customer name which serves as key
    //get returns option type and immutable
    let values = &mut data.get_mut(&trim_name);
    match values {
        Some(x) => {
            println!("Enter your password");
            let mut pass = String::new();
            io::stdin()
                .read_line(&mut pass)
                .expect("fail to get password");
            let trim_pass = pass.as_str().trim().to_string();
            if x.password == trim_pass {
                //Giving various options to customer if password matches
                println!("Type options: Read,Add,Del,Edit todo");
                println!("Change Password");
                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("fail to get task");
                let trim_task = task.as_str().trim().to_string();
                //Read task
                if trim_task == "Read" {
                    Read(&mut x.todo);
                }
                //Add task
                else if trim_task == "Add" {
                    println!("Type to add");
                    let mut tadd = String::new();
                    io::stdin()
                        .read_line(&mut tadd)
                        .expect("fail to take input to  be add");
                    let trim_tadd = tadd.as_str().trim().to_string();
                    Add(&mut x.todo, trim_tadd);
                    save_data(&data);
                }
                //Delete task
                else if trim_task == "Del" {
                    println!("Type index to del");
                    let mut tbdel = String::new();
                    io::stdin()
                        .read_line(&mut tbdel)
                        .expect("fail to take input to be del");
                    let trim_tbdel = tbdel.as_str().trim().to_string();
                    let tdel = trim_tbdel.parse::<usize>().unwrap();
                    if tdel < x.todo.len() {
                        Del(&mut x.todo, tdel);
                        save_data(&data);
                    } else {
                        println!("Index entered does not exist");
                    }
                }
                //Edit task
                else if trim_task == "Edit" {
                    println!("Type index to Edit");
                    let mut tbedit = String::new();
                    io::stdin()
                        .read_line(&mut tbedit)
                        .expect("fail to take input to be edited");
                    let trim_tbedit = tbedit.as_str().trim().to_string();
                    let tedit = trim_tbedit.parse::<usize>().unwrap();
                    if tedit < x.todo.len() {
                        println!("Edit");
                        let mut edit = String::new();
                        io::stdin()
                            .read_line(&mut edit)
                            .expect("fail to take input to edit");
                        let trim_edit = edit.as_str().trim().to_string();
                        Edit(&mut x.todo, tedit, trim_edit);
                        save_data(&data);
                    } else {
                        println!("Index entered does not exist");
                    }
                }
                //Change Password task
                else if trim_task == "Change Password" {
                    println!("Type new password");
                    let mut changepass = String::new();
                    io::stdin()
                        .read_line(&mut changepass)
                        .expect("fail to change password");
                    let trim_changepass = changepass.as_str().trim().to_string();
                    x.password = trim_changepass;
                    save_data(&data);
                }
            } else {
                println!("Wrong Password");
            }
        }

        None => {
            println!("No such user exists");
            //Adding new customer to dataset
            println!("To add new customer,type customer name and password ");
            let mut new_customername = String::new();
            io::stdin()
                .read_line(&mut new_customername)
                .expect("fail to take new customer name ");
            let trim_newc = new_customername.as_str().trim().to_string();
            let mut new_password = String::new();
            io::stdin()
                .read_line(&mut new_password)
                .expect("fail to take new password ");
            let trim_newp = new_password.as_str().trim().to_string();
            New_Customer(&mut data, trim_newc, trim_newp);
            save_data(&data);
        }
    }
}
