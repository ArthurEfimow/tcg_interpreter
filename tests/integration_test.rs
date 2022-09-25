use TCG_Engine::game_engine;
use TCG_Engine::game_engine::event::Event;
use TCG_Engine::game_engine::effect::TargetCondition;
use TCG_Engine::game_engine::card;
use crate::game_engine::basic_data_structure::BasicDataStructure;
use TCG_Engine::game_engine::effect::Target;

pub fn create_giant() -> card::Card{
    let mut output = BasicDataStructure::create_ebds();
    output.set_value("ID","Giant");
    output.set_value("name","Hitotsu-Me Giant");
    output.set_value("card_type",stringify!(CardType::Monster(MonsterType::Normal)));
    output.set_value("passcode","76184692");
    output.set_number("ATK",1200);
    output.set_number("DEF",1000);
    output.set_number("LEVEL",4);
    output.set_value("attribute","EARTH");
    output.set_values("monster_type",&vec!("Beast-Warrior".to_string()));
    output.set_value("text","A one-eyed behemoth with thick, powerful arms made for delivering punishing blows.");
    output
}

#[test]
fn test_create_engine() {
    let mut engine = game_engine::create_game_engine();
    assert_eq!(engine.get_state(),"Inactive".to_string());
}

#[test]
fn test_start_engine() {
    let mut engine = game_engine::create_game_engine();
    engine.init();
    assert_eq!(engine.get_state(),"Normal".to_string());
}

#[test]
fn test_add_card() {
    let mut engine = game_engine::create_game_engine();
    engine.add_card(&create_giant());
    engine.init();
    assert_eq!(engine.get_state(),"Normal".to_string());
}

#[test]
fn test_resolve_quest() {
    let mut engine = game_engine::create_game_engine();
    engine.add_card(&create_giant());
    engine.init();
    assert_eq!(engine.set_event(&Event::Choice(TargetCondition::IsMonster,String::from("Monster"),TargetCondition::IsMonster,Box::new(Event::Done(String::from("Monster"))),Box::new(Event::Cancel(String::from("Monster"))))),(0,"OK".to_string()));
    assert_eq!(engine.in_equilibrium(),false);
    engine.resolve_state();
    assert_eq!(engine.get_state(),"Quest".to_string());
    engine.interpret_input(&serde_json::to_string(&Target::Card("Giant".to_string())).unwrap());
    engine.resolve_state();
    assert_eq!(engine.get_state(),"Processing".to_string());
    engine.resolve_state();
    assert_eq!(engine.get_state(),"Processing".to_string());
    engine.resolve_state();
    assert_eq!(engine.get_state(),"Normal".to_string());
}
