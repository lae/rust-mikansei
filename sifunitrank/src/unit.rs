#[derive(PartialEq, Copy)]
enum Attribute {
    Smile,
    Pure,
    Cool,
    All
}
/*
impl Attribute {
    fn new(value: u8) -> Attribute {
        match value {
            1 => Attribute::Smile,
            2 => Attribute::Pure,
            3 => Attribute::Cool,
            5 => Attribute::Cool,
            _ => Attribute::Missing
        }
    }
}
#[derive(Debug, Copy, Clone)]
enum Trigger {
    Note,
    Combo,
    Perfect,
    Missing
}

impl Trigger {
    fn new(value: u8) -> Trigger {
        match value {
            3 => Trigger::Note,
            4 => Trigger::Combo,
            6 => Trigger::Perfect,
            _ => Trigger::Missing
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Rarity {
    N,
    R,
    SR,
    UR,
    SSR,
    Missing
}

impl Rarity {
    fn new(value: u8) -> Rarity {
        match value {
            1 => Rarity::N,
            2 => Rarity::R,
            3 => Rarity::SR,
            4 => Rarity::UR,
            5 => Rarity::SSR,
            _ => Rarity::Missing
        }
    }
}

#[derive(Debug)]
struct Skill {
    id: u16,
    trigger: Trigger,
    levels: Vec<SkillLevel>
}

#[derive(Debug)]
struct SkillLevel {
    level: u8,
    increase: f64,
    step: u8,
    rate: u8
}

#[derive(Debug)]
struct CalculatedSkill {
    id: u16,
    trigger: Trigger,
    points: Vec<f64>
}

#[derive(Debug)]
struct Card {
    id: u16,
    number: u16,
    subtitle: String,
    name: String,
    rarity: Rarity,
    attribute: Attribute,
    skill_id: u16
}
*/
