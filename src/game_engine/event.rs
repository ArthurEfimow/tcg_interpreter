use serde::{Deserialize, Serialize};
use crate::game_engine::Gamefield;
use crate::game_engine::effect::Effect;
use crate::game_engine::turn::Player;
use crate::game_engine::effect::TargetPlayer;
use crate::game_engine::card::*;
use crate::game_engine::card;
use crate::game_engine::effect::TargetCondition;
use crate::game_engine::effect::SummonAction;
use crate::game_engine::action::ActionState;
use crate::game_engine::action::Action;

type Next = Box<Event>;
type Actions = Vec<Action>;

const NS: &str = "Normalsummon";
const NSC: &str = "normal_summon_card";
const NSZ: &str = "normal_summon_zone";


#[derive(Clone,Serialize,Debug, Deserialize,PartialEq)]
pub enum Event{
    Choice(TargetCondition,String,TargetCondition,Box<Event>,Box<Event>), // Question, wich variable needs to be set, does the variable fullfills Quest, Yes, No
    Chain(Actions,Next),
    Cancel(String),
    Done(String),
    Fail(String),
    Start(String,Next),
}

fn s(input : &str) -> String{
    String::from(input)
}

fn v(effect : &Effect) -> String{
    serde_json::to_string(effect).unwrap()
}

fn nst() -> Effect{
    Effect::Summon(SummonAction::Normal,s(NSC),s(NSZ))
}

fn nscc() -> TargetCondition{
    TargetCondition::AndV(vec!(TargetCondition::InHand(TargetPlayer::Active),TargetCondition::IsMonster))
}

fn nszc() -> TargetCondition{
    TargetCondition::AndV(vec!(TargetCondition::BelongsTo(TargetPlayer::Active),TargetCondition::IsMonsterZone,TargetCondition::IsEmpty))
}

fn nsac() -> ActivationCondition{
    ActivationCondition::Activate(ActionTrigger::AndV(vec!(ActionTrigger::IsPhase("Main".to_string()),ActionTrigger::OncePerTurn,ActionTrigger::ActivePlayer)))
}

pub fn normal_summon_event() -> Event {
    let NormalSummon4 = Event::Chain(vec!(Action::Declare(ActionState::Finish,s(NS),v(&nst()))),Box::new(Event::Done(s(NS))));
    let NormalSummon3 = Event::Chain(vec!(Action::ExecuteEffect(nst())),Box::new(NormalSummon4));
    let NormalSummon2 = Event::Chain(vec!(Action::Declare(ActionState::Activate,s(NS),v(&nst()))),Box::new(NormalSummon3));
    let NormalSummon1 = Event::Choice(nszc(),s(NSZ),nszc(),Box::new(NormalSummon2),Box::new(Event::Cancel(s(NS))));
    let NormalSummon0 = Event::Choice(nscc(),s(NSC),nscc(),Box::new(NormalSummon1),Box::new(Event::Cancel(s(NS))));
    Event::Start(s(NS),Box::new(NormalSummon0))
}
