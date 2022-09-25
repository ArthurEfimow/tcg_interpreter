use serde::{Deserialize, Serialize};

use crate::model::card_type::*;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

#[derive(Clone,Debug,PartialEq,Serialize, Deserialize)]
pub enum Select{
    Card(String),
    Player(TargetPlayer),
    TargetZone(Zone),
}


#[derive(Clone,Debug,PartialEq,Serialize, Deserialize)]
pub enum Target{
    Card(Option<String>,Condition),
    Player(TargetPlayer),
    TargetZone(Option<Location>,Condition),
}


#[derive(Clone,Debug,Copy,PartialEq,Serialize, Deserialize)]
pub enum TargetPlayer{
    Both,
    Myself,
    Enemy,
}

#[derive(Clone,Debug,PartialEq,Serialize, Deserialize)]
pub enum PlayerPhase{
    Draw,
    Standby,
    Main1,
    Battle,
    Main2,
    End,
}

pub fn next_phase(phase : PlayerPhase) -> PlayerPhase {
        match phase {
            PlayerPhase::Draw =>   PlayerPhase::Standby,
            PlayerPhase::Standby => PlayerPhase::Main1,
    	    PlayerPhase::Main1 =>  PlayerPhase::Battle,
    	    PlayerPhase::Battle => PlayerPhase::Main2 ,
    	    PlayerPhase::Main2 =>  PlayerPhase::End ,
    	    PlayerPhase::End   =>  PlayerPhase::Draw,
        }
}

#[derive(PartialEq,Eq,Hash,Debug,Clone,Copy,Serialize, Deserialize)]
pub enum Location{
    Nowhere,
    Player1(Zone),
    Player2(Zone),
}



#[derive(PartialEq,Eq,Hash,Debug,Clone,Copy,Serialize, Deserialize,EnumIter)]
pub enum Zone{
    Nowhere,
    Deck,
    ExtraDeck,
    Hand,
    MonsterField1,
    MonsterField2,
    MonsterField3,
    MonsterField4,
    MonsterField5,
    SpellField1,
    SpellField2,
    SpellField3,
    SpellField4,
    SpellField5,
    Graveyard,
    Field,
    Ex,
}

#[derive(Clone,Debug,PartialEq,Serialize, Deserialize)]
pub enum Condition{
    And(Box<Condition>,Box<Condition>),
    Or(Box<Condition>,Box<Condition>),
    Not(Box<Condition>),
    Always,
    Never,
    Card(CardCondition),
    Zone(ZoneCondition),
    Phase(PlayerPhase),
}

#[derive(Clone,Debug,PartialEq,Serialize, Deserialize)]
pub enum CardCondition{
    IsType(MonsterType),
    OnField,
    FaceUp,
    Location(Zone),
}

#[derive(Clone,Debug,PartialEq,Serialize, Deserialize)]
pub enum ZoneCondition{
    MonsterZone,
    Free,
}
