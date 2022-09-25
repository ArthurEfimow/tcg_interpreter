pub mod card;
pub mod rules;
pub mod gamefield;
pub mod basic_data_structure;
pub mod effect;
pub mod turn;
pub mod action;
pub mod zone;
pub mod event;

use serde::{Deserialize, Serialize};
use crate::game_engine::rules::Rule;
use crate::game_engine::gamefield::Gamefield;
use crate::game_engine::event::Event;
use crate::game_engine::effect::TargetCondition;
use crate::game_engine::card::Card;


#[derive(Clone,Serialize,Debug, Deserialize,PartialEq)]
pub enum GameMode{
    Inactive,
    Normal,
    Question(TargetCondition,String),
    Processing,
    Init,
}


#[derive(Clone,Serialize, Deserialize,Debug,PartialEq)]
pub struct GameEngine{
    pub gf : Gamefield,
    pub rules : Vec<Rule>,
    pub mode : GameMode,
    pub event: Option<Event>,
    pub input: Option<String>
    //pub actions  : vec<Action>
}

pub fn create_game_engine() -> GameEngine{
    let mut gf = Gamefield::create_empty_gamefield();
    GameEngine {gf,rules : vec!(),mode : GameMode::Inactive,event : None,input : None}
}

impl GameEngine {

    pub fn interpret_input(&mut self, input : &str){
        match &self.mode {
            GameMode::Question(condition,value_key) => if self.gf.fullfills_condition(&condition,&input) {self.gf.set_value(&value_key,input)},
            _ => {},
        }
    }
    
    pub fn add_card(&mut self,card : & Card) {
        self.gf.add_card(card)
    }
    
    pub fn resolve_event(&mut self,event : &Event){
        match event{
            Event::Choice(a,b,c,d,e) => self.resolve_choice(&a,b.clone(),&c,&d,&e),
            Event::Done(_) => self.event = None,
            _ => {},
        }
        
    }

    pub fn resolve_choice(&mut self,quest : &TargetCondition,answer : String, condition : &TargetCondition, success : &Event, fail : &Event){
        let answer_value = self.gf.get_value(&answer);
        if answer_value.len() == 0 {self.mode = GameMode::Question(quest.clone(),answer.clone());return}
        self.mode = GameMode::Processing;
        if self.gf.fullfills_condition(condition,answer_value.clone()) {self.event =  Some(success.clone())} else {self.event = Some(fail.clone())}
    }
    
    pub fn in_equilibrium(& self) -> bool {
        self.event == None && self.input == None && self.mode == GameMode::Normal
    }
    
    
    pub fn resolve_state(&mut self){
        if self.in_equilibrium() {return}
        if self.input != None {
            let mut input = self.input.as_ref().unwrap().clone();
            self.interpret_input(&input);
            self.input = None;
            return
        }
        if self.event != None {
            self.mode = GameMode::Processing;
            let mut event = self.event.as_ref().unwrap().clone();
            self.resolve_event(&event);
            return
        }
        match &self.mode{
            _ => self.mode = GameMode::Normal,
        }
        
    }
    
    pub fn init(&mut self) -> (usize,String){
        if self.mode != GameMode::Inactive {return (1,"Engine is already running".to_string())}
        self.mode = GameMode::Normal;
        (0,"OK".to_string())
    }
    
    pub fn set_event(&mut self,event : &Event) -> (usize,String){
        self.event = Some(event.clone());
        (0,"OK".to_string())
    }
    
    pub fn get_state(&mut self) -> String{
        match self.mode{
            GameMode::Inactive => "Inactive".to_string(),
            GameMode::Normal => "Normal".to_string(),
            GameMode::Question(_,_) => "Quest".to_string(),
            GameMode::Processing => "Processing".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}


