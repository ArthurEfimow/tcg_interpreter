use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string::String;

#[derive(Clone,Serialize, Deserialize,Debug,PartialEq)]
pub struct BasicDataStructure {
    pub numbers : HashMap<String,i32>,
    pub value   : HashMap<String,String>,
    pub values  : HashMap<String,Vec<String>>,
    pub substru : HashMap<String,BasicDataStructure>,
}

impl BasicDataStructure{

    pub fn create_ebds() -> BasicDataStructure{
        BasicDataStructure{
            numbers : HashMap::new(),
            value   : HashMap::new(),
            values  : HashMap::new(),
            substru : HashMap::new()
        }
    }
    
    pub fn set_value(&mut self,key: &str,value : &str) {
        self.value.insert(String::from(key),String::from(value));
    }
    
    pub fn set_number(&mut self,key: &str,value : i32) {
        self.numbers.insert(String::from(key),value);
    }
    
   pub fn set_substru(&mut self,key: &str,value : BasicDataStructure) {
        self.substru.insert(String::from(key),value);
    }
    
    pub fn get_number(& self,key: &str) -> i32 {
        match self.numbers.get(key) {
            Some(num) => *num,
            None => 0,
        }
    }
    
    pub fn get_value(& self,key: &str) -> &str {
        match self.value.get(key) {
            Some(val) => &val,
            None => &"",
        }
    }
    
    pub fn values_is_empty(& self,key: &str) -> bool {
        match self.values.get(key) {
            Some(val) => val.is_empty(),
            None => true,
        }
    }
    
    pub fn get_values(&mut self,key: &str) -> Vec<String> {
        match self.values.get(key) {
            Some(val) => val.clone(),
            None => vec!(),
        }
    }
    
    pub fn get_substru(&self,key: &str) -> Option<&BasicDataStructure> {
        self.substru.get(key)
    }
    
    pub fn get_substru_mut(&mut self,key: &str) -> Option<&mut BasicDataStructure> {
        self.substru.get_mut(key)
    }
    
    pub fn set_values(&mut self,key: &str,values : &Vec<String>) {
        self.values.insert(String::from(key),values.clone());
    }
    
    pub fn add_values(&mut self,key: &str,value : &str) {
        let mut meta = self.get_values(key);
        meta.push(String::from(value));
        self.set_values(key,&meta)
    }
        
    pub fn raise_flag(&mut self,flag : &str){
        match self.values.get_mut(&String::from("flags")) {
            None => {self.values.insert(String::from("flags"),vec!(String::from(flag)));},
            Some(list) => if !list.contains(&String::from(flag)) {list.push(String::from(flag))},
        }
    }
    
    pub fn remove_flag(&mut self,flag : &str){
        self.remove_from_values("flags",flag)
    }
    
    pub fn remove_value(&mut self,value : &str){
        self.value.remove(&String::from(value));
    }
    
    pub fn remove_from_values(&mut self,values : &str,value : &str){
         match self.values.get_mut(&String::from(values)) {
            None => {},
            Some(list) => {list.retain(|x| String::from(x) != String::from(value));},
        }
        
    }
    
    pub fn decrease_number(&mut self,key: &str,num:i32) {
        let meta = self.get_number(key);
        self.set_number(key,meta-num);
    }
    
    pub fn increase_number(&mut self,key: &str,num:i32) {
        let meta = self.get_number(key);
        self.set_number(key,meta+num);
    }
    
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_get_empty_value() {
        let bds = BasicDataStructure::create_ebds();
        assert_eq!(bds.get_value(&"key"),"");    
    }
}


