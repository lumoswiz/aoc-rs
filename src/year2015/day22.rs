use arrayvec::ArrayVec;
use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum EffectKind {
    Shield,
    Poison,
    Recharge,
}

impl EffectKind {
    fn activate(&self, battle: &mut Battle) {
        match self {
            EffectKind::Shield => battle.mage.armor += 7,
            _ => {}
        }
    }

    fn tick(&self, battle: &mut Battle) {
        match self {
            EffectKind::Poison => battle.boss.hp -= 3,
            EffectKind::Recharge => battle.mage.mana += 101,
            _ => {}
        }
    }

    fn remove(&self, battle: &mut Battle) {
        match self {
            EffectKind::Shield => battle.mage.armor -= 7,
            _ => {}
        }
    }
}

#[derive(Clone, Debug)]
struct Effect {
    kind: EffectKind,
    timer: usize,
}

#[derive(Clone, Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn values() -> impl Iterator<Item = Spell> {
        const SPELLS: [Spell; 5] = [
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge,
        ];
        SPELLS.iter().cloned()
    }

    fn cast(&self) -> (i32, i32, i32, Option<Effect>) {
        match self {
            Spell::MagicMissile => (53, 4, 0, None),
            Spell::Drain => (73, 2, 2, None),
            Spell::Shield => (
                113,
                0,
                0,
                Some(Effect {
                    kind: EffectKind::Shield,
                    timer: 6,
                }),
            ),
            Spell::Poison => (
                173,
                0,
                0,
                Some(Effect {
                    kind: EffectKind::Poison,
                    timer: 6,
                }),
            ),
            Spell::Recharge => (
                229,
                0,
                0,
                Some(Effect {
                    kind: EffectKind::Recharge,
                    timer: 5,
                }),
            ),
        }
    }
}

#[derive(Clone, Debug)]
struct Mage {
    hp: i32,
    armor: i32,
    mana: i32,
}

#[derive(Clone, Debug)]
struct Boss {
    hp: i32,
    damage: i32,
}

#[derive(Clone, Debug)]
struct Battle {
    mage: Mage,
    boss: Boss,
    effects: ArrayVec<[Effect; 3]>,
}

impl Battle {
    fn turn(&mut self, spell: Spell) {}

    fn can_cast(&self, spell: Spell) -> bool {
        let (mana_cost, _, _, effect) = spell.cast();

    }

    fn valid_spells(&self) -> impl Iterator<Item = Spell> {
        
    }
}

impl FromStr for Battle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = BOSS_PATTERN
            .captures(s)
            .ok_or_else(|| failure::err_msg("does not match boss pattern"))?;

        Ok(Battle {
            mage: Mage {
                hp: 50,
                armor: 0,
                mana: 500,
            },
            boss: Boss {
                hp: c[1].parse()?,
                damage: c[2].parse()?,
            },
            effects: ArrayVec::new(),
        })
    }
}

lazy_static! {
    static ref BOSS_PATTERN: Regex = Regex::new(r"Hit Points: (\d+)\nDamage: (\d+)").unwrap();
}

pub fn puzzle1(input: &str) -> i32 {
    let battle = Battle::from_str(input).expect("failed to parse input");

    println!("{:?}", battle);

    0
}

pub fn puzzle2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(""), 0);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(""), 0);
    }
}
