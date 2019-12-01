use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{self, Reverse};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Modifier {
    Weak(String),
    Immune(String),
}

#[derive(Clone, Debug)]
struct Unit {
    count: u32,
    hp: u32,
    modifiers: Vec<Modifier>,
    damage: u32,
    attack_type: String,
    initiative: u8,
}

impl Unit {
    fn effective_power(&self) -> u32 {
        self.count * self.damage
    }

    fn damage_to(&self, enemy: &Unit) -> u32 {
        let damage_modifier = enemy
            .modifiers
            .iter()
            .filter_map(|m| match m {
                Modifier::Weak(ref t) if *t == self.attack_type => Some(2),
                Modifier::Immune(ref t) if *t == self.attack_type => Some(0),
                _ => None,
            })
            .nth(0)
            .unwrap_or(1);
        self.effective_power() * damage_modifier
    }

    fn take_damage(&mut self, damage: u32) -> u32 {
        let units_lost = damage / self.hp;
        self.count = self.count.saturating_sub(units_lost);
        units_lost
    }
}

lazy_static! {
    static ref UNIT_PATTERN: Regex = Regex::new(r"(\d+) units each with (\d+) hit points (\((.+)\) )?with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
    static ref MODIFIER_PATTERN: Regex = Regex::new(r"(weak|immune) to ((\w+)(, \w+)*)").unwrap();
}

impl FromStr for Unit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = UNIT_PATTERN
            .captures(s)
            .ok_or_else(|| failure::err_msg("does not match unit pattern"))?;
        let mut modifiers = Vec::new();
        if let Some(ms) = c.get(4) {
            for m in ms.as_str().split(';').map(|m| m.trim()) {
                let cm = MODIFIER_PATTERN
                    .captures(m)
                    .ok_or_else(|| failure::err_msg("does not match modifier pattern"))?;
                for a in cm[2].split(',').map(|m| m.trim()) {
                    let modifier = match &cm[1] {
                        "weak" => Modifier::Weak(a.to_string()),
                        "immune" => Modifier::Immune(a.to_string()),
                        _ => unreachable!(),
                    };
                    modifiers.push(modifier);
                }
            }
        }

        Ok(Unit {
            count: c[1].parse()?,
            hp: c[2].parse()?,
            modifiers,
            damage: c[5].parse()?,
            attack_type: c[6].to_string(),
            initiative: c[7].parse()?,
        })
    }
}

fn parse(input: &str) -> Result<(Vec<Unit>, Vec<Unit>), Error> {
    let mut lines = input.trim().split('\n');

    lines
        .next()
        .ok_or_else(|| failure::err_msg("no immune system"))?;
    let mut immune_system = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        immune_system.push(line.parse::<Unit>()?);
    }

    lines
        .next()
        .ok_or_else(|| failure::err_msg("no infection"))?;
    let mut infection = Vec::new();
    for line in lines {
        infection.push(line.parse()?);
    }

    Ok((immune_system, infection))
}

fn sort_for_target_selection(units: &mut [Unit]) {
    units.sort_unstable_by_key(|u| (Reverse(u.effective_power()), Reverse(u.initiative)));
}

#[derive(Clone, Copy, Debug)]
enum TargetKind {
    ImmuneSystem,
    Infection,
}

#[derive(Debug)]
struct Target {
    kind: TargetKind,
    attacker: usize,
    defender: usize,
}

impl Target {
    fn new(kind: TargetKind, attacker: usize, defender: usize) -> Target {
        Target {
            kind,
            attacker,
            defender,
        }
    }

    fn get<'a>(&self, immune_system: &'a [Unit], infection: &'a [Unit]) -> (&'a Unit, &'a Unit) {
        let (attackers, defenders) = match self.kind {
            TargetKind::ImmuneSystem => (immune_system, infection),
            TargetKind::Infection => (infection, immune_system),
        };
        (&attackers[self.attacker], &defenders[self.defender])
    }

    fn get_mut<'a>(
        &self,
        immune_system: &'a mut [Unit],
        infection: &'a mut [Unit],
    ) -> (&'a mut Unit, &'a mut Unit) {
        let (attackers, defenders) = match self.kind {
            TargetKind::ImmuneSystem => (immune_system, infection),
            TargetKind::Infection => (infection, immune_system),
        };
        (&mut attackers[self.attacker], &mut defenders[self.defender])
    }

    fn score(&self, attackers: &[Unit], defenders: &[Unit]) -> (u32, u32, u8) {
        let attacker = &attackers[self.attacker];
        let defender = &defenders[self.defender];
        (
            attacker.damage_to(defender),
            defender.effective_power(),
            defender.initiative,
        )
    }
}

fn add_targets(
    kind: TargetKind,
    attackers: &[Unit],
    defenders: &[Unit],
    targets: &mut Vec<Target>,
) {
    let mut targeted = HashSet::new();
    for ai in 0..attackers.len() {
        let mut defender_indices = (0..defenders.len()).filter(|i| !targeted.contains(i));
        let di = match defender_indices.next() {
            Some(i) => i,
            None => return,
        };

        let mut target = Target::new(kind, ai, di);
        for di in defender_indices {
            let t = Target::new(kind, ai, di);
            if t.score(attackers, defenders) > target.score(attackers, defenders) {
                target = t;
            }
        }

        if target.score(attackers, defenders).0 == 0 {
            continue;
        }

        targeted.insert(target.defender);
        targets.push(target);
    }
}

fn target_phase(
    immune_system: &mut Vec<Unit>,
    infection: &mut Vec<Unit>,
    targets: &mut Vec<Target>,
) {
    sort_for_target_selection(immune_system);
    sort_for_target_selection(infection);

    targets.clear();
    add_targets(TargetKind::ImmuneSystem, immune_system, infection, targets);
    add_targets(TargetKind::Infection, infection, immune_system, targets);

    targets.sort_unstable_by_key(|t| {
        let (attacker, _) = t.get(immune_system, infection);
        Reverse(attacker.initiative)
    });
}

fn remove_dead_units(units: &mut Vec<Unit>) {
    units.retain(|u| u.count > 0);
}

fn attack_phase(
    immune_system: &mut Vec<Unit>,
    infection: &mut Vec<Unit>,
    targets: &mut Vec<Target>,
) -> u32 {
    let mut total_lost = 0;

    for target in targets.iter() {
        let (attacker, defender) = target.get_mut(immune_system, infection);
        let damage = attacker.damage_to(defender);
        total_lost += defender.take_damage(damage);
    }

    remove_dead_units(immune_system);
    remove_dead_units(infection);

    total_lost
}

fn battle(immune_system: &mut Vec<Unit>, infection: &mut Vec<Unit>, boost: u32) {
    for unit in immune_system.iter_mut() {
        unit.damage += boost;
    }

    let mut targets = Vec::with_capacity(immune_system.len() + infection.len());
    while !immune_system.is_empty() && !infection.is_empty() {
        target_phase(immune_system, infection, &mut targets);
        let total_lost = attack_phase(immune_system, infection, &mut targets);
        if total_lost == 0 {
            break;
        }
    }
}

pub fn puzzle1(input: &str) -> u32 {
    let (mut immune_system, mut infection) = parse(input).expect("failed to parse input");
    battle(&mut immune_system, &mut infection, 0);

    cmp::max(
        immune_system.iter().map(|u| u.count).sum(),
        infection.iter().map(|u| u.count).sum(),
    )
}

pub fn puzzle2(input: &str) -> u32 {
    let (immune_system, infection) = parse(input).expect("failed to parse input");

    fn immune_system_win(immune_system: &[Unit], infection: &[Unit], boost: u32) -> Option<u32> {
        let mut immune_system = immune_system.to_vec();
        let mut infection = infection.to_vec();

        battle(&mut immune_system, &mut infection, boost);
        if infection.is_empty() {
            Some(immune_system.iter().map(|u| u.count).sum())
        } else {
            None
        }
    }

    (1..)
        .filter_map(|boost| immune_system_win(&immune_system, &infection, boost))
        .nth(0)
        .expect("immune system never wins")
}

#[cfg(test)]
mod tests {
    const INPUT: &str = r"
        Immune System:
        17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
        989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

        Infection:
        801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
        4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
    ";

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(INPUT), 5216);
    }

    #[test]
    fn puzzle2() {
        let (mut immune_system, mut infection) =
            super::parse(INPUT).expect("failed to parse input");
        super::battle(&mut immune_system, &mut infection, 1570);

        assert_eq!(immune_system.iter().map(|u| u.count).sum::<u32>(), 51);
    }
}
