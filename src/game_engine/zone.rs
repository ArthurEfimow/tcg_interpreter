use serde::{Deserialize, Serialize};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use crate::game_engine::basic_data_structure::BasicDataStructure;
use std::string::String;

pub type Zone = BasicDataStructure;

#[derive(Clone,Copy,Serialize,Debug, Deserialize,PartialEq)]
pub enum ZoneName{
    MonsterZone,
    SpellZone,
    FieldZone,
    Hand,
    Deck,
}

pub fn generate_all_zones() -> BasicDataStructure{
    let mut output = BasicDataStructure::create_ebds();
    let mut i : i32 = 1;
    let mut j : i32 = 0;
    while i < 3 {
        output.set_substru(&String::from("Hand"),generate_zone(i,0,&String::from("Hand")));
        output.set_substru(&String::from("Deck"),generate_zone(i,0,&String::from("Deck")));
        output.set_substru(&String::from("FieldZone"),generate_zone(i,0,&String::from("FieldZone")));
        while j < 6 {output.set_substru(&String::from("MonsterZone"),generate_zone(i,j,&String::from("MonsterZone")));j+=1;}     
        j = 0; while j < 6 {output.set_substru(&String::from("SpellZone"),generate_zone(i,j,&String::from("SpellZone")));j+=1;}
        i+=1;
    }
    output
}

pub fn generate_zone(owner : i32, number : i32, name : &str) -> Zone{
    let mut output = BasicDataStructure::create_ebds();
    output.set_value("Name",&String::from(name));
    output.set_number("Number",number);
    output.set_number("Owner",owner);
    output
}

pub fn is_spellzone(zone : &Zone) -> bool{
        zone.get_value("Name").contains("Spell")
 }
 
pub fn is_monsterzone(zone : &Zone) -> bool{
        zone.get_value("Name").contains("Monster")
}

 pub fn is_zone(zone : &Zone) -> bool{
        true
 }
 
  pub fn is_empty(zone : &Zone) -> bool{
        zone.values_is_empty(&"Cards")
 }
 


