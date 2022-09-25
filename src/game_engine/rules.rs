use serde::{Deserialize, Serialize};
use crate::game_engine::gamefield::Gamefield;

#[derive(Clone,Serialize, Deserialize,Debug,PartialEq)]
pub enum Rule{
    //Trigger(Condition,Event),
    Law
    //Init(Vec<BasicFunctions>),
}

pub fn init_game(rules : &Vec<Rule>, gf : &mut Gamefield){
    for rule in rules {
        match rule {
            //Rule::Init(commands) => for fun in commands { basic_function(fun,gf)},
            _ => {},
        }
    }
}
