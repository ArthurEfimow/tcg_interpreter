use serde::{Deserialize, Serialize};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use crate::game_engine::basic_data_structure::BasicDataStructure;
use crate::game_engine::turn::Player;

pub type Card = BasicDataStructure;

#[derive(Clone,Copy,Serialize,Debug, Deserialize,PartialEq)]
pub enum MonsterType{
    Normal,
    Effect,
}

#[derive(Clone,Copy,Serialize,Debug, Deserialize,PartialEq)]
pub enum CardType{
    Monster(MonsterType),
    Spell,
    Trap,
}

pub fn create_gaia(id : String) -> BasicDataStructure{
    let mut output = BasicDataStructure::create_ebds();
    output.set_value("ID",&id);
    output.set_value("name","Hitotsu-Me Giant");
    output.set_value("card_type",stringify!(CardType::Monster(MonsterType::Normal)));
    output.set_value("passcode","76184692");
    output.set_number("ATK",1200);
    output.set_number("DEF",1000);
    output.set_number("LEVEL",4);
    output.set_number("owner",1);
    output.set_number("controller",1);
    output.set_value("attribute","EARTH");
    output.set_values("monster_type",&vec!("Beast-Warrior".to_string()));
    output.set_value("text","A one-eyed behemoth with thick, powerful arms made for delivering punishing blows.");
    output
}

pub fn generate_card() -> Card{
    let mut rng = thread_rng();
    let on_card : BasicDataStructure = create_gaia((0..40).map(|_| rng.sample(Alphanumeric) as char).collect());
    let mut card = on_card.clone();
    card.set_substru(&"HardData",on_card);
    card
}

 pub fn is_monster(card : &Card) -> bool{
        card.get_value("card_type").contains("Monster")
 }
 
 pub fn on_field(card : &Card) -> bool{
        card.get_value("zone").contains("Zone")
 }
 
pub fn in_zone(card : &Card,zone : &str) -> bool {
    card.get_value("zone").contains(zone) 
}

pub fn controlled_by(card : &Card,player : i32) -> bool {
    card.get_number("controller") == player
}
 
pub fn in_hand(player : Player ) -> Box<dyn Fn(&BasicDataStructure) -> bool>{
    Box::new(move |x| in_zone(x,&"Hand")&&in_zone(x,&("Player".to_string()+&player.to_string())))
}

 pub fn is_card(card : &Card) -> bool{
        true
 }
 
 pub fn summon(card0 : Option<&mut Card>,player : i32,summon_type : &str, zone : &str){
        if card0 == None {return}
        let mut card = card0.unwrap();
        //card.set_value("prev_zone",card.get_value("zone"));
        card.set_number("controller",player);
        card.set_value("summon_type",summon_type);
        card.set_value("zone",zone);
 }

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    fn create_monster_giant() -> BasicDataStructure{
        let mut output = BasicDataStructure::create_ebds();
        output.set_value("ID","Giant");
        output.set_value("name","Hitotsu-Me Giant");
        output.set_value("card_type",stringify!(CardType::Monster(MonsterType::Normal)));
        output.set_value("passcode","76184692");
        output.set_number("ATK",1200);
        output.set_number("DEF",1000);
        output.set_number("LEVEL",4);
        output.set_number("owner",1);
        output.set_number("controller",1);
        output.set_value("attribute","EARTH");
        output.set_value("zone","Player1_MonsterZone1");
        output.set_values("monster_type",&vec!("Beast-Warrior".to_string()));
        output.set_value("text","A one-eyed behemoth with thick, powerful arms made for delivering punishing blows.");
        output
    }
    #[test]
    fn test_is_monster_true() {
        let card = create_monster_giant();
        assert_eq!(is_monster(&card),true);
    }
    
    #[test]
    fn test_on_field_true() {
        let card = create_monster_giant();
        assert_eq!(on_field(&card),true);
    }
    
    #[test]
    fn test_on_field_false() {
        let mut card = create_monster_giant();
        card.set_value("zone","Player1_Hand");
        assert_eq!(on_field(&card),false);
    }
    
    #[test]
    fn test_in_hand_true() {
        let mut card = create_monster_giant();
        card.set_value("zone","Player1_Hand");
        assert_eq!(in_hand(1)(&card),true);
    }
    
    #[test]
    fn test_in_hand_false() {
        let mut card = create_monster_giant();
        assert_eq!(in_hand(1)(&card),false);
    }

}
