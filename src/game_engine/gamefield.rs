use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::game_engine::card;
use crate::game_engine::action;
use crate::game_engine::effect;
use crate::game_engine::card::Card;
use crate::game_engine::zone::Zone;
use crate::game_engine::zone;
use crate::game_engine::basic_data_structure::BasicDataStructure;
use crate::game_engine::turn::TurnCounter;
use crate::game_engine::turn::TurnStmt;
use crate::game_engine::turn::Player;
use crate::game_engine::TargetCondition;

pub type Gamefield = BasicDataStructure;



impl Gamefield {

    pub fn create_empty_gamefield() -> Gamefield{
        //if Phase::Standby > Phase::Draw {println!("YES!")}
        //println!("{}",stringify!(Phase::Draw));
        let mut field = BasicDataStructure::create_ebds();
        field.set_substru(&"TurnCounter",TurnCounter::create_turn_counter());
        field.set_substru(&"Cards",BasicDataStructure::create_ebds());
        field.set_substru(&"Zones",zone::generate_all_zones());
        field.set_number(&"Player1_LifePoints",8000);
        field.set_number(&"Player2_LifePoints",8000);
        field
    }

    pub fn active_player(& self) -> Player{
        let meta = self.get_substru(&"TurnCounter");
        match meta {
            Some(Value) => Value.get_player(),
            None => 0,
        }
    }
    
    pub fn summon(&mut self,active_player : i32,summon_type : &str, card_id : &str,zone : &str){
        let card = self.get_card(card_id);
        card::summon(card,active_player,summon_type,zone);
    } 
    
    pub fn effect_damage(&mut self, player : Player,num : i32){
        let meta = format!("Player{}_LifePoints",player);
        self.decrease_number(&meta,num);
    }  
    
    pub fn effect_heal(&mut self, player : Player,num : i32){
        let meta = format!("Player{}_LifePoints",player);
        self.increase_number(&meta,num);
    }
    
    pub fn declare(&mut self, declaration : &str,name : &str,args : &str){
        self.set_value(&"Declaration",name);
        self.set_value(&"Declare",declaration);
        self.set_value(&"DeclarationArgs",args);
    }
    
    pub fn find_cards(&self, condition : &dyn Fn(&Card) -> bool) -> Vec<String>{
        let mut output = vec!();
        let meta = self.get_substru(&"Cards");
        let cards : &BasicDataStructure;
        match meta {
            Some(Value) => cards = Value,
            None => return output,
        }
        
        for (id,card) in &cards.substru {
            if condition(&card) { output.push(id.clone())}
        }
        output
    }
    
     pub fn get_card(&mut self, id : &str) -> Option<&mut Card>{
        let meta = self.get_substru_mut(&"Cards");
        let cards : &mut BasicDataStructure;
        match meta {
            Some(value) => cards = value,
            None => return None,
        }
        
        cards.get_substru_mut(id)
    }
    
    pub fn add_card(&mut self, card : &Card) {
        let meta = self.get_substru_mut(&"Cards");
        let cards : &mut BasicDataStructure;
        match meta {
            Some(value) => cards = value,
            None => return,
        }
        let id = card.get_value(&"ID");
        cards.set_substru(&id,card.clone());
    }
    
    pub fn get_card_unmut(& self, id : &str) -> Option<& Card>{
        let meta = self.get_substru(&"Cards");
        let cards : & BasicDataStructure;
        match meta {
            Some(value) => cards = value,
            None => return None,
        }
        
        cards.get_substru(id)
    }
    
    pub fn get_zone_unmut(& self, id : &str) -> Option<& Zone>{
        let meta = self.get_substru(&"Zone");
        let zone : & BasicDataStructure;
        match meta {
            Some(value) => zone = value,
            None => return None,
        }
        
        zone.get_substru(id)
    }
    
     pub fn get_target(& self, target : &effect::Target) -> Option<& BasicDataStructure>{
        match target{
            effect::Target::Card(id) => self.get_card_unmut(&id),
            effect::Target::Zone(id) => self.get_zone_unmut(&id),
        }
    }
    
    pub fn destroy(&mut self,_ : Vec<String>){
    }
    
    pub fn fullfills_condition(& self, condition : & TargetCondition,target : &str) -> bool{
        let meta : effect::Target = serde_json::from_str(target).unwrap();
        let fun = action::create_target_function(condition,0,self);
        fun(self.get_target(&meta).unwrap())
        
    }    
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_get_active_player() {
        let field = Gamefield::create_empty_gamefield();
        assert_eq!(field.active_player(),1);
    }
    #[test]
    fn test_effect_damage() {
        let mut field = Gamefield::create_empty_gamefield();
        assert_eq!(field.get_number(&"Player1_LifePoints"),8000);
        field.effect_damage(1,1);
        assert_eq!(field.get_number(&"Player1_LifePoints"),7999);
    }
}
