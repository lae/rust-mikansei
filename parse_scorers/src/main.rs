extern crate rusqlite;

use rusqlite::Connection;

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

#[derive(Debug, Copy, Clone, PartialEq)]
enum Attribute {
    Smile,
    Pure,
    Cool,
    Missing
}

impl Attribute {
    fn new(value: u8) -> Attribute {
        match value {
            1 => Attribute::Smile,
            2 => Attribute::Pure,
            3 => Attribute::Cool,
            _ => Attribute::Missing
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


fn collect_skills(c: &Connection) -> Result<Vec<Skill>, rusqlite::Error> {
    let mut stmt = try!(c.prepare("SELECT * from unit_skill_m WHERE skill_effect_type = 11 AND (trigger_type = 3 OR trigger_type = 4 OR trigger_type = 6) ORDER BY unit_skill_id"));
    let mut rows = try!(stmt.query(&[]));

    let mut skills = Vec::new();
    while let Some(result_row) = rows.next() {
        let row = try!(result_row);
        skills.push(Skill {
            id: row.get(0),
            trigger: Trigger::new(row.get(6)),
            levels: try!(collect_skill_levels(c, row.get(0)))
        });
    }
    Ok(skills)
}

fn collect_skill_levels(c: &Connection, skill_id: u16) -> Result<Vec<SkillLevel>, rusqlite::Error> {
    let mut stmt = try!(c.prepare("SELECT * from unit_skill_level_m WHERE unit_skill_id = :id ORDER BY skill_level"));
    let mut rows = try!(stmt.query_named(&[(":id", &skill_id)]));

    let mut skill_levels = Vec::new();
    while let Some(result_row) = rows.next() {
        let row = try!(result_row);
        skill_levels.push(SkillLevel {
            level: row.get(1),
            increase: row.get(4),
            step: row.get(6),
            rate: row.get(7)
        });
    }
    Ok(skill_levels)
}

fn calculate_skills(skills: &Vec<Skill>, notes: usize, perfect_notes: usize) -> Result<Vec<CalculatedSkill>, ()> {
    let mut calculated_skills = Vec::new();
    for skill in skills {
        let mut points_vec = Vec::new();
        // Try to look for skill levels 1-8, as 8 should be the max
        for lv in 1..9 {
            if let Ok(idx) = skill.levels.binary_search_by_key(&lv, |s| s.level) {
                let slvl = &skill.levels[idx];
                let points = match skill.trigger {
                    // We just assume FC since actual combo is too hard to track
                    Trigger::Note | Trigger::Combo => notes,
                    Trigger::Perfect => perfect_notes,
                    Trigger::Missing => 0
                } as f64 / slvl.step as f64 * slvl.increase * 2.5 * (slvl.rate as f64 / 100.0);
                // And we push, from lowest level to highest, average points scored
                points_vec.push(points.floor());
            } else {
                // Push zeroes to pad to 8 entries in case we're missing skill levels
                points_vec.push(0_f64);
            }
        }
        calculated_skills.push(CalculatedSkill {
            id: skill.id,
            trigger: skill.trigger,
            points: points_vec
        });
    }
    Ok(calculated_skills)
}

fn collect_cards(c: &Connection, skills: &Vec<Skill>) -> Result<Vec<Card>, rusqlite::Error> {
    // Select only μ's or Aqours members
    let mut stmt = try!(c.prepare("SELECT * FROM unit_m WHERE rarity > :rarity AND (unit_type_id <= 9 OR (unit_type_id >= 101 AND unit_type_id <= 109)) AND max_removable_skill_capacity > 2 ORDER BY unit_number"));
    let mut rows = try!(stmt.query_named(&[(":rarity", &2)]));

    let mut cards = Vec::new();
    while let Some(result_row) = rows.next() {
        let row = try!(result_row);
        let skill_id: u16 = row.get(13);
        if let Ok(_) = skills.binary_search_by_key(&skill_id, |s| s.id) {
            cards.push(Card {
                id: row.get(0),
                number: row.get(1),
                subtitle: row.get_checked(3).unwrap_or("".to_owned()),
                name: row.get(4),
                rarity: Rarity::new(row.get(11)),
                attribute: Attribute::new(row.get(12)),
                skill_id: skill_id
            });
        }
    }
    Ok(cards)
}

fn main() {
    let c = Connection::open_with_flags("./unit.db_", rusqlite::SQLITE_OPEN_READ_ONLY).unwrap();

    let skills = collect_skills(&c).unwrap();
    let mut cards = collect_cards(&c, &skills).unwrap();
    cards.retain(|c| c.rarity == Rarity::UR && c.attribute == Attribute::Smile);
    let calculated_skills = calculate_skills(&skills, 550, 468).unwrap();
    for card in &cards {
            if let Ok(idx) = calculated_skills.binary_search_by_key(&card.skill_id, |s| s.id) {
                let skill = &calculated_skills[idx];
                println!("https://sif.kirara.ca/card/{} {} {:?} {}",
                         //card.number, skill.points[2]-skill.points[1], card.name, skill.trigger);
                         card.number, card.name, skill.trigger, skill.points.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(" "));
                //println!("{} #{}: {}", card.name, card.number, skill.points[1]);
            }
    }
}
