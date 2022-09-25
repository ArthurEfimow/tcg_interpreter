pub mod game_engine;
use crate::game_engine::card::generate_card;
use crate::game_engine::gamefield::Gamefield;
use crate::game_engine::GameEngine;
use crate::game_engine::basic_data_structure::BasicDataStructure;
use crate::game_engine::rules::Rule;
use crate::game_engine::rules::init_game;

fn main() {
    let card = generate_card();
    let j = serde_json::to_string(&card).unwrap();
    /*let lp1 = BasicFunctions::SetNumber(Target::Field,"Player1_LifePoints".to_string(),8000);
    let lp2 = BasicFunctions::SetNumber(Target::Field,"Player2_LifePoints".to_string(),8000);
    
    let pha = BasicFunctions::SetValues(Target::Field,"Phase".to_string(),vec!("Player1_Draw".to_string(),"Player1_Standby".to_string(),"Player1_Main1".to_string(),"Player1_Battle".to_string(),"Player1_Main2".to_string(),"Player1_End".to_string(),
                                                                               "Player2_Draw".to_string(),"Player2_Standby".to_string(),"Player2_Main1".to_string(),"Player2_Battle".to_string(),"Player2_Main2".to_string(),"Player2_End".to_string()));
    let phb = BasicFunctions::SetNumber(Target::Field,"Current_Phase_Index".to_string(),1);
    
    let zones = vec!("Player1_Deck".to_string(),"Player1_Hand".to_string(),"Player1_GY".to_string(),"Player1_EX".to_string(),"Player1_XDeck".to_string(),"Player1_Field".to_string(),"Player1_Monster1".to_string(),"Player1_Monster2".to_string(),"Player1_Monster3".to_string(),"Player1_Monster4".to_string(),"Player1_Monster5".to_string(),"Player1_Spell1".to_string(),"Player1_Spell2".to_string(),"Player1_Spell3".to_string(),"Player1_Spell4".to_string(),"Player1_Spell5".to_string(),
                     "Player2_Deck".to_string(),"Player2_Hand".to_string(),"Player2_GY".to_string(),"Player2_EX".to_string(),"Player2_XDeck".to_string(),"Player2_Field".to_string(),"Player2_Monster1".to_string(),"Player2_Monster2".to_string(),"Player2_Monster3".to_string(),"Player2_Monster4".to_string(),"Player2_Monster5".to_string(),"Player2_Spell1".to_string(),"Player2_Spell2".to_string(),"Player2_Spell3".to_string(),"Player2_Spell4".to_string(),"Player2_Spell5".to_string());
    
    
    let mut commands : Vec<BasicFunctions> = vec!(lp1,lp2,pha,phb);
    for zone in &zones {commands.push(BasicFunctions::SetValues(Target::Field,"Zone_".to_string()+&zone.clone(),vec!()));}
    commands.push(BasicFunctions::SetValues(Target::Field,"Zone".to_string(),zones));
    let rule = Rule::Init(commands);
    let mut ge = GameEngine {gf: create_empty_gamefield(),rules : vec!(rule) };
    init_game(&ge.rules,&mut ge.gf);
    let j = serde_json::to_string(&ge.gf).unwrap();*/
    Gamefield::create_empty_gamefield();
    println!("{}", j);
}

