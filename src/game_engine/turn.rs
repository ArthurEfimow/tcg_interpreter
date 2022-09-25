use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::game_engine::basic_data_structure::BasicDataStructure;

pub type TurnCounter = BasicDataStructure;
pub type TurnNumber = i32;
pub type Player = i32;
pub type Phase = String;
pub type StepNumber = i32;
pub type TurnStmt = (TurnNumber,Player,Phase,StepNumber);

impl TurnCounter {

    pub fn create_turn_counter() -> TurnCounter{
        let mut output = BasicDataStructure::create_ebds();
        output.set_number(&"Turn",1);
        output.set_number(&"Player",1);
        output.set_number(&"Step",1);
        output.set_value(&"Phase",&"Main1");
        output
    }


    pub fn next_step(&mut self) {
        let mut step = self.get_number(&"Step");
        self.set_number(&"Step",step + 1);
    }
    
    pub fn next_phase(&mut self) {
        let meta = match self.get_value(&"Phase") {
            "Draw" => "Standby",
            "Standby" => "Main1",
            "Main1" => "Battle",
            "Battle" => "Main2",
            "Main2" => "End",
            "End" => "Draw",
            &_ => "Draw",
        };
        if meta.to_string() == "Draw".to_string() { return self.next_turn()}
        self.set_number(&"Step",1);
        self.set_value(&"Phase",&meta);
    }
    
    pub fn next_player(&mut self) {
        let meta = 3 - self.get_number(&"Step");
        self.set_number(&"Step",meta);
    }
    
    pub fn next_turn(&mut self) {
        let mut turn = self.get_number(&"Turn");
        self.set_number(&"Turn",turn + 1);
        self.next_player();
        self.set_number(&"Step",1);
        self.set_value(&"Phase",&"Draw");
    }
    
    pub fn get_stmt(&mut self) -> TurnStmt {
        (self.get_number(&"Turn"),self.get_number(&"Player"),self.get_value(&"Phase").to_string(),self.get_number(&"Step"))
    }
    
    pub fn get_player(& self) -> Player {
        self.get_number(&"Player")
    }
    
    
}
