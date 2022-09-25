use serde::{Deserialize, Serialize};
use crate::game_engine::Gamefield;
use crate::game_engine::effect::Effect;
use crate::game_engine::turn::Player;
use crate::game_engine::effect::TargetPlayer;
use crate::game_engine::card::*;
use crate::game_engine::zone::*;
use crate::game_engine::card;
use crate::game_engine::zone;
use crate::game_engine::effect::TargetCondition;
use crate::game_engine::basic_data_structure::BasicDataStructure;


#[derive(Clone,Serialize,Debug, Deserialize,PartialEq)]
pub enum Action{
    Declare(ActionState,String,String),
    ExecuteEffect(Effect),
}

#[derive(Clone,Copy,Serialize,Debug, Deserialize,PartialEq)]
pub enum ActionState{
    Activate,
    Execute,
    Finish,
}

#[derive(Clone,Copy,Serialize,Debug, Deserialize,PartialEq)]
pub enum ActionChoice{
    Mandatory,
    Optional,
}

#[derive(Clone,Copy,Serialize,Debug, Deserialize,PartialEq)]
pub enum ActionSpeed{
    Normal,
    Fast(i32),
    Fastest,
}

pub fn get_player(activator : Player,target : TargetPlayer,gf : & Gamefield) -> Player{
    match target {
        TargetPlayer::Active => gf.active_player(),
        TargetPlayer::Inactive => 3 - gf.active_player(),
        TargetPlayer::Owner => activator,
        TargetPlayer::Opponent => 3 - activator,
        TargetPlayer::Both => 3,
        _ => 0,
    }
}

fn compose_one(f: fn(&BasicDataStructure) -> bool) -> impl Fn(&BasicDataStructure) -> bool
{
    move |x| f(x)
}

fn compose_and(f: Box<dyn Fn(&BasicDataStructure) -> bool>, g: Box<dyn Fn(&BasicDataStructure) -> bool>) -> impl Fn(&BasicDataStructure) -> bool
{
    move |x| g(x) && f(x)
}

pub fn create_target_function(target : &TargetCondition,player : Player,gf : & Gamefield) -> Box<dyn Fn(&BasicDataStructure) -> bool> {
    match target {
        TargetCondition::And(a,b) => Box::new(compose_and(create_target_function(&a,player,gf),create_target_function(&b,player,gf))),//impl Fn(&Card) -> bool {for c in conditions{ if !create_target_function(c)(card) {return false}} true},
        TargetCondition::IsMonster => Box::new(compose_one(card::is_monster)),
        TargetCondition::OnField   => Box::new(compose_one(card::on_field)),
        TargetCondition::InHand(meta)   => card::in_hand(get_player(player,*meta,gf)),
        TargetCondition::IsSpellZone   => Box::new(compose_one(zone::is_spellzone)),
        _ => Box::new(compose_one(card::is_card)),
    }
}

pub fn do_action(player : Player,action : &Action,gf : &mut Gamefield){
    match action {
        Action::Declare(state,stmt,args) => gf.declare(&serde_json::to_string(&state).unwrap(),stmt,args),
        Action::ExecuteEffect(effect) => execute_effect(player,effect,gf),
    }
}

pub fn execute_effect(player : Player,effect : &Effect,gf : &mut Gamefield){

    match effect {
        //Action::Put(card,zone) => gf.put(gf.get_value(&card),gf.get_value(&zone)),
        Effect::Burn(target,num) => gf.effect_damage(get_player(player,*target,gf),*num),
        Effect::Heal(target,num) => gf.effect_heal(get_player(player,*target,gf),*num),
        Effect::Destroy(target) => gf.destroy(gf.find_cards(&create_target_function(&target,player,gf))),
        _ => {},
    }
}
