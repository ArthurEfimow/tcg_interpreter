use serde::{Deserialize, Serialize};

#[macro_export] macro_rules! normal_draw{
    ()=>{{
            Effect::Colon(ActivationCondition::When(ActionTrigger::IsPhase("Draw".to_string()),Effect::AlsoAfterThat(Box::new(Effect::Draw(TargetPlayer::Active,1)),Box::new(Effect::NextPhase))))
        }}
}

#[macro_export] macro_rules! normal_summon{
    ($a:expr,$b:expr)=>{{
        Effect::Colon(ActivationCondition::Activate(ActionTrigger::And(Box::new(ActionTrigger::IsPhase("Main".to_string())),Box::new(ActionTrigger::OncePerTurn))),Box::new(Effect::Summon(SummonAction::Normal,$a,$b)))
    }}
}

#[derive(Clone,Copy,Serialize,Debug, Deserialize,PartialEq)]
pub enum Stat{
    ATK,
    DEF,
    ATKDEF,
    NoStat,
}

#[derive(Clone,Copy,Serialize,Debug, Deserialize,PartialEq)]
pub enum StatChange{
    RaiseStat(Stat,i32),
    NoChange,
}

#[derive(Clone,Serialize,Debug, Deserialize,PartialEq)]
pub enum Target{
    Card(String),
    Zone(String),
}

#[derive(Clone,Copy,Serialize,Debug, Deserialize,PartialEq)]
pub enum TargetPlayer{
    Owner,
    Opponent,
    Active,
    Inactive,
    Both,
    Choose,
    Random,
}

#[derive(Clone,Copy,Serialize,Debug, Deserialize,PartialEq)]
pub enum MonsterType{
    Normal,
    Fusion,
    Flip,
    Effect,
    Warrior,
    Beast,
    Spellcaster,
    Zombie,
    Aqua,
    Fiend,
    Insect,
    Plant,
    Machine,
    Dinosaur,
    Fairy,
    Dragon,
    Thunder,
    BeastWarrior,
    WingedBeast,
    Fish,
}

#[derive(Clone,Serialize,Debug, Deserialize,PartialEq)]
pub enum TargetCondition{
    And(Box<TargetCondition>,Box<TargetCondition>),
    AndV(Vec<TargetCondition>),
    Or(Box<TargetCondition>,Box<TargetCondition>),
    Not(Box<TargetCondition>),
    IsMonster,
    HasMonsterType(MonsterType),
    OnField,
    InHand(TargetPlayer),
    FaceUp,
    ControlledBy(TargetPlayer),
    Lowest(Stat),
    HigherEq(Stat,i32),
    That, //is mentioned
    IsTarget,
    ThisCard,
    IsZone,
    IsSpellZone,
    IsMonsterZone,
    IsEmpty,
    BelongsTo(TargetPlayer),
    NoCondition,
}

#[derive(Clone,Serialize,Debug, Deserialize,PartialEq)]
pub enum SummonAction{
    Normal,
    Flip,
}

#[derive(Clone,Serialize,Debug, Deserialize,PartialEq)]
pub enum ActionTrigger{
    And(Box<ActionTrigger>,Box<ActionTrigger>),
    AndV(Vec<ActionTrigger>),
    Or(Box<ActionTrigger>,Box<ActionTrigger>),
    Summon(Vec<SummonAction>,TargetCondition),
    IsPhase(String),
    ActivePlayer,
    OncePerTurn,
    OncePerGame,
    Never,
    Always,
}

#[derive(Clone,Serialize,Debug, Deserialize,PartialEq)]
pub enum ActivationCondition{
    When(ActionTrigger),
    Activate(ActionTrigger),
    Never,
}

#[derive(Clone,Serialize,Debug, Deserialize,PartialEq)]
pub enum Active{
    Target(TargetCondition),
    Tribute(TargetCondition),
    NoActive,
}

#[derive(Clone,PartialEq,Debug,Serialize, Deserialize)]
pub enum Effect{
    NoEffect,
    NextPhase,
    Draw(TargetPlayer,i32),
    Burn(TargetPlayer,i32), // Target, damage
    Put(String,String), // card on zone
    Heal(TargetPlayer,i32), // Target, LP
    Destroy(TargetCondition),
    Equip(TargetCondition,StatChange),
    Summon(SummonAction,String,String),
    Colon(ActivationCondition,Box<Effect>), // After Conditions are met
    SemiColon(Active,Box<Effect>), // Active are things to do (Targeting,Paying Cost), to make the effect happen. Active is not a Condition, but a part of the effect resolve
    AlsoAfterThat(Box<Effect>,Box<Effect>),
}

pub fn text_to_effect(text : &str) -> Effect{
   if text_is_colon_effect(text) {let parts: Vec<&str> = text.split(":").collect(); return Effect::Colon(text_to_activation_condition(&parts[0]),Box::new(text_to_effect(&parts[1])))}
   if text_is_semicolon_effect(text) {let parts: Vec<&str> = text.split(";").collect(); return Effect::SemiColon(text_to_active(&parts[0]),Box::new(text_to_effect(&parts[1])))}
   let unclean: Vec<&str> = text.split(" ").collect();
   let clean = clean_words(&unclean);
   let words: Vec<&str> = clean.iter().map(AsRef::as_ref).collect();
   if text_is_burn_effect(&words) {return Effect::Burn(excavate_target_player_from_text(&words),excavate_number_from_words(0,&words))}
   if text_is_heal_effect(&words) {return Effect::Heal(excavate_target_player_from_text(&words),excavate_number_from_words(0,&words))}
   if text_is_destruction_effect(&words) {return Effect::Destroy(excavate_target_condition_from_text(&words))}
   if text_is_equip_effect(&words) {return Effect::Equip(excavate_target_condition_from_text(&words),excavate_stat_change_from_text(&words))}
   Effect::NoEffect
}

pub fn text_to_active(text : &str) -> Active{
   let unclean: Vec<&str> = text.split(" ").collect();
   let clean = clean_words(&unclean);
   let words: Vec<&str> = clean.iter().map(AsRef::as_ref).collect();
   if text_is_targeting(&words) {return Active::Target(excavate_target_condition_from_text(&words))}
   Active::NoActive
}

pub fn text_to_activation_condition(text : &str) -> ActivationCondition{
   let unclean: Vec<&str> = text.split(" ").collect();
   let clean = clean_words(&unclean);
   let words: Vec<&str> = clean.iter().map(AsRef::as_ref).collect();
   if text_is_when_condition(&words) {return ActivationCondition::When(excavate_action_trigger_from_text(&words))}
   ActivationCondition::Never
}

pub fn clean_words(unclean : &Vec<&str>) -> Vec<String>{
   let mut output = vec!(); 
   let mut index = 0;
   while index < unclean.len(){
       if unclean[index].contains("Life") && unclean.len() > index+1 && unclean[index+1].contains("Points") {output.push(String::from("LP"));index += 2;continue;}
       if unclean[index].contains("'s") {output.push(unclean[index].replace("'s",""));index += 1;continue;}
       if unclean[index].contains("ATK") {output.push(unclean[index].replace("",""));index += 1;continue;}
       if unclean[index].contains("DEF") {output.push(unclean[index].replace("",""));index += 1;continue;}
       if unclean[index] == "all" {index += 1;continue;}
       if unclean[index] == "the" {index += 1;continue;}
       output.push(unclean[index].to_lowercase());
       index +=1;
   }
   output
}

pub fn text_is_burn_effect(words : &Vec<&str>) -> bool{
   (words.contains(&"damage") && words.contains(&"to"))
   || (words.contains(&"LP") && words.contains(&"decrease"))
}

pub fn text_is_heal_effect(words : &Vec<&str>) -> bool{
   words.contains(&"LP") && words.contains(&"increase")
}

pub fn text_is_destruction_effect(words : &Vec<&str>) -> bool{
   words.contains(&"destroy")
}

pub fn text_is_equip_effect(words : &Vec<&str>) -> bool{
   words.contains(&"equip") || words.contains(&"equipped")
}

pub fn text_is_targeting(words : &Vec<&str>) -> bool{
   words.contains(&"target")
}

pub fn text_is_colon_effect(text : &str) -> bool{
   text.contains(&":")
}

pub fn text_is_semicolon_effect(text : &str) -> bool{
   text.contains(&";")
}

pub fn text_is_when_condition(words : &Vec<&str>) -> bool{
   words.contains(&"when")
}


pub fn excavate_number_from_words(index : usize,words : &Vec<&str>) -> i32{
    let mut myindex = index;
    while myindex < words.len(){
        let meta = words[myindex].parse::<i32>();
        match meta {
            Ok(value) => return value,
            _ => {}
        }
        myindex+=1; 
    }
    0
}

pub fn excavate_target_player_from_text(words : &Vec<&str>) -> TargetPlayer{
    for my_string in words{
        if my_string.contains("opponent") {return TargetPlayer::Opponent}
    }
    TargetPlayer::Owner
}

pub fn excavate_stat_from_text(words : &Vec<&str>) -> Stat{
   let mut index = 0;
   while index < words.len(){
        if words[index].contains("ATK") && words.len() > index+1 && words[index+1].contains("and") && words.len() > index+2 && words[index+2].contains("DEF")  {return Stat::ATKDEF}
        if words[index].contains("ATK/DEF") {return Stat::ATKDEF}
        if words[index].contains("ATK") {return Stat::ATK}
        if words[index].contains("DEF") {return Stat::ATK}
        index+=1;
    }
    Stat::NoStat
}


pub fn create_and(transform : &mut Vec<TargetCondition>) -> TargetCondition {
    if transform.len() <= 0 {return TargetCondition::NoCondition}
    if transform.len() == 1 {return transform.pop().unwrap()}
    TargetCondition::And(Box::new(transform.pop().unwrap()),Box::new(create_and(transform)))
}

pub fn create_and_trigger(transform : &mut Vec<ActionTrigger>) -> ActionTrigger {
    if transform.len() <= 0 {return ActionTrigger::Always}
    if transform.len() == 1 {return transform.pop().unwrap()}
    ActionTrigger::And(Box::new(transform.pop().unwrap()),Box::new(create_and_trigger(transform)))
}


pub fn excavate_target_condition_from_text(words : &Vec<&str>) -> TargetCondition{
   let mut output : Vec<TargetCondition> = vec!(); 
   let mut index = 0;
   while index < words.len(){
       if words[index].contains("monster") {output.push(TargetCondition::IsMonster)}
       else if words[index].contains("face-up") {output.push(TargetCondition::FaceUp)}
       else if words[index].contains("warrior-beast") {output.push(TargetCondition::HasMonsterType(MonsterType::BeastWarrior))}
       else if words[index].contains("winged") {output.push(TargetCondition::HasMonsterType(MonsterType::WingedBeast))}
       else if words[index].contains("warrior") {output.push(TargetCondition::HasMonsterType(MonsterType::Warrior))}
       else if words[index].contains("beast") {output.push(TargetCondition::HasMonsterType(MonsterType::Beast))}
       else if words[index].contains("spellcaster") {output.push(TargetCondition::HasMonsterType(MonsterType::Spellcaster))}
       else if words[index].contains("zombie") {output.push(TargetCondition::HasMonsterType(MonsterType::Zombie))}
       else if words[index].contains("aqua") {output.push(TargetCondition::HasMonsterType(MonsterType::Aqua))}
       else if words[index].contains("fiend") {output.push(TargetCondition::HasMonsterType(MonsterType::Fiend))}
       else if words[index].contains("insect") {output.push(TargetCondition::HasMonsterType(MonsterType::Insect))}
       else if words[index].contains("plant") {output.push(TargetCondition::HasMonsterType(MonsterType::Plant))}
       else if words[index].contains("machine") {output.push(TargetCondition::HasMonsterType(MonsterType::Machine))}
       else if words[index].contains("dinosaur") {output.push(TargetCondition::HasMonsterType(MonsterType::Dinosaur))}
       else if words[index].contains("fairy") {output.push(TargetCondition::HasMonsterType(MonsterType::Fairy))}
       else if words[index].contains("dragon") {output.push(TargetCondition::HasMonsterType(MonsterType::Dragon))}
       else if words[index].contains("thunder") {output.push(TargetCondition::HasMonsterType(MonsterType::Thunder))}
       else if words[index].contains("fish") {output.push(TargetCondition::HasMonsterType(MonsterType::Fish))}
       else if words[index].contains("that") {output.push(TargetCondition::That)}
       else if words[index].contains("controls") {output.push(TargetCondition::ControlledBy(excavate_target_player_from_text(words)))}
       else if words[index].contains("lowest") {output.push(TargetCondition::Lowest(excavate_stat_from_text(words)))}
       else if words[index].contains("or")  && words.len() > index+1 && words[index+1].contains("more") {output.push(TargetCondition::HigherEq(excavate_stat_from_text(words),excavate_number_from_words(index-2,words)))}
       else if words[index].contains("on") && words.len() > index+1 && words[index+1].contains("field") {output.push(TargetCondition::OnField)}
       index +=1;
   }
   if output.len() == 0 { return TargetCondition::That}
   if output.len() == 1 { return output[0].clone()}
   TargetCondition::AndV(output)
}

pub fn excavate_stat_change_from_text(words : &Vec<&str>) -> StatChange{
   let mut index = 0;
   while index < words.len(){
       if words[index].contains("gains") || words[index].contains("increases")  {return StatChange::RaiseStat(excavate_stat_from_text(words),excavate_number_from_words(index,words))}
       index +=1;
   }
   StatChange::NoChange
}

pub fn excavate_action_trigger_from_text(words : &Vec<&str>) -> ActionTrigger{
    for my_string in words{
        if my_string.contains("summons") {return ActionTrigger::Summon(excavate_summon_actions_from_text(words),excavate_target_condition_from_text(words))}
    }
    ActionTrigger::Never
}

pub fn excavate_summon_actions_from_text(words : &Vec<&str>) -> Vec<SummonAction>{
   let mut output : Vec<SummonAction> = vec!(); 
   let mut index = 0;
   while index < words.len(){
       if words[index].contains("normal") {output.push(SummonAction::Normal)}
       else if words[index].contains("flip") {output.push(SummonAction::Flip)}
       index +=1;
   }
   output
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_text_is_burn_damage() {
        let yes = vec!("inflict","200","points","of", "damage", "to", "your", "opponent", "LP");
        assert_eq!(text_is_burn_effect(&yes),true);
    }
    #[test]
    fn test_excavate_number_from_words() {
        let words: Vec<&str> = vec!("inflict","200","points","of", "damage", "to", "your", "opponent", "LP");
        let value = excavate_number_from_words(0,&words);
        assert_eq!(value,200);
    }
    #[test]
    fn test_clean_words() {
        let text = "Inflict 200 points of damage to your opponent's Life Points.";
        let words: Vec<&str> = text.split(" ").collect();
        assert_eq!(clean_words(&words),vec!("inflict","200","points","of", "damage", "to", "your", "opponent", "LP"));
    }
    #[test]
    fn test_text_sparks_to_effect() {
        let text = "Inflict 200 points of damage to your opponent's Life Points.";
        assert_eq!(text_to_effect(text),Effect::Burn(TargetPlayer::Opponent,200));
    }
    #[test]
    fn test_text_hinotama_to_effect() {
        let text = "Inflict 500 damage to your opponent.";
        assert_eq!(text_to_effect(text),Effect::Burn(TargetPlayer::Opponent,500));
    }
    #[test]
    fn test_text_red_medicine_to_effect() {
        let text = "Increase your Life Points by 500 points.";
        assert_eq!(text_to_effect(text),Effect::Heal(TargetPlayer::Owner,500));
    }
    #[test]
    fn test_text_black_hole_to_effect() {
        let text = "Destroy all monsters on the field.";
        assert_eq!(text_to_effect(text),Effect::Destroy(TargetCondition::AndV(vec!(TargetCondition::IsMonster,TargetCondition::OnField))));
    }
    #[test]
    fn test_text_fissure_to_effect() {
        let text = "Destroy the 1 face-up monster your opponent controls that has the lowest ATK (your choice, if tied).";
        assert_eq!(text_to_effect(text),Effect::Destroy(TargetCondition::AndV(vec!(TargetCondition::FaceUp,TargetCondition::IsMonster,TargetCondition::ControlledBy(TargetPlayer::Opponent),TargetCondition::That,TargetCondition::Lowest(Stat::ATK)))));
    }
    #[test]
    fn test_text_trap_hole_to_effect() {
        let text = "When your opponent Normal or Flip Summons 1 monster with 1000 or more ATK: Target that monster; destroy that target.";
        assert_eq!(text_to_effect(text),Effect::Colon(ActivationCondition::When(ActionTrigger::Summon(vec!(SummonAction::Normal,SummonAction::Flip),TargetCondition::AndV(vec!(TargetCondition::IsMonster,TargetCondition::HigherEq(Stat::ATK,1000))))),Box::new(Effect::SemiColon(Active::Target(TargetCondition::AndV(vec!(TargetCondition::That,TargetCondition::IsMonster))),Box::new(Effect::Destroy(TargetCondition::That))))));
    }
    #[test]
    fn test_text_legendary_sword_to_effect() {
        let text = "Equip only to a Warrior monster. It gains 300 ATK/DEF.";
        assert_eq!(text_to_effect(text),Effect::Equip(TargetCondition::AndV(vec!(TargetCondition::HasMonsterType(MonsterType::Warrior),TargetCondition::IsMonster)),StatChange::RaiseStat(Stat::ATKDEF,300)));
    }
    #[test]
    fn test_text_beast_fangs_to_effect() {
        let text = "A Beast-Type monster equipped with this card increases its ATK and DEF by 300 points.";
        assert_eq!(text_to_effect(text),Effect::Equip(TargetCondition::AndV(vec!(TargetCondition::HasMonsterType(MonsterType::Beast),TargetCondition::IsMonster)),StatChange::RaiseStat(Stat::ATKDEF,300)));
    }
}
