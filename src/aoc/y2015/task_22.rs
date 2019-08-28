use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
struct Character {
    hit_points: i16,
    mana: i16,
    damage: i16,
    armor: i16,
}

impl Character {
    fn player(hit_points: i16, mana: i16) -> Self { Character { hit_points, mana, damage: 0, armor: 0 } }
    fn boss(hit_points: i16, damage: i16) -> Self { Character { hit_points, damage, mana: 0, armor: 0 } }

    fn damage(&self, damage: i16) -> Self {
        let damage_done = i16::max(damage - self.armor, 1);
        Character {
            hit_points: self.hit_points - damage_done,
            mana: self.mana,
            damage: self.damage,
            armor: self.armor,
        }
    }

    fn spend_mana(&self, mana: i16) -> Self {
        Character {
            hit_points: self.hit_points,
            mana: self.mana - mana,
            damage: self.damage,
            armor: self.armor,
        }
    }

    fn as_caster(&self, this_step: &usize, buffs: &Vec<&Effect>) -> Self {
        let mut mana = self.mana;
        let mut hit_points = self.hit_points;
        let mut armor = self.armor;
        buffs.iter().for_each(|effect| {
            mana += effect.buff.mana();
            hit_points += effect.buff.heal();
            if this_step - effect.casted_at == 1 {
                armor += effect.buff.armor()
            } else if this_step - effect.casted_at == effect.buff.steps() {
                armor -= effect.buff.armor()
            }
        });
        return Character {
            mana,
            hit_points,
            armor,
            damage: self.damage,
        };
    }
    fn as_target(&self, _: &usize, buffs: &Vec<&Effect>) -> Self {
        let mut hit_points = self.hit_points;
        buffs.iter().for_each(|effect| {
            hit_points -= effect.buff.damage()
        });
        return Character {
            hit_points,
            mana: self.mana,
            damage: self.damage,
            armor: self.armor,
        };
    }
}

struct Effect {
    casted_at: usize,
    buff: Box<Buff>,
}

impl Effect {
    fn is_finished(&self, turn: &usize) -> bool { self.buff.steps() < turn - self.casted_at }
}

#[derive(PartialEq)]
enum Buff {
    MagicMissle,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Buff {
    fn effect(&self, step: usize) -> Effect {
        Effect { casted_at: step, buff: Box::new(self.clone()) }
    }

    fn cost(&self) -> i16 {
        match self {
            Buff::MagicMissle => 53,
            Buff::Drain => 73,
            Buff::Poison => 173,
            Buff::Recharge => 229,
            Buff::Shield => 113
        }
    }

    fn heal(&self) -> i16 {
        match self {
            Buff::Drain => 2,
            _ => 0
        }
    }

    fn armor(&self) -> i16 {
        match self {
            Buff::Shield => 7,
            _ => 0
        }
    }

    fn mana(&self) -> i16 {
        match self {
            Buff::Recharge => 101,
            _ => 0
        }
    }

    fn damage(&self) -> i16 {
        match self {
            Buff::MagicMissle => 4,
            Buff::Drain => 2,
            Buff::Poison => 3,
            _ => 0
        }
    }

    fn steps(&self) -> usize {
        match self {
            Buff::MagicMissle => 1,
            Buff::Drain => 1,
            Buff::Poison => 6,
            Buff::Recharge => 5,
            Buff::Shield => 6
        }
    }
}

pub fn run() {
    let input = File::open("src/aoc/y2015/task_22").unwrap();
    let input = BufReader::new(input);

    let boss = input.lines()
        .filter_map(|l| l.ok())
        .map(|l: String| l.split(": ").map(|s| s.to_string()).collect())
        .map(|s: Vec<String>| s[1].parse::<i16>())
        .filter_map(|s| s.ok())
        .collect::<Vec<i16>>();

    let boss = Character::boss(boss[0], boss[1]);
    let player = Character::player(50, 500);
//    let boss = Character::boss(13, 8);
//    let player = Character::player(10, 250);

    let buffs = vec![Buff::MagicMissle, Buff::Drain, Buff::Shield, Buff::Poison, Buff::Recharge];

    let result = game(&player, &boss, &buffs, vec![], 0).unwrap();

    println!("Result: {}", result)
}

fn game(player: &Character, boss: &Character, buffs: &Vec<Buff>, effects: Vec<&Effect>, step: usize) -> Option<i16> {
    let player = player.as_caster(&step, &effects);
    let boss = boss.as_target(&step, &effects);

//    if step >= 30 {
//        return None;
//    }

    if player.hit_points <= 0 {
        return None;
    } else if boss.hit_points <= 0 {
        return Some(0);
    }

    let active_effects: Vec<&Effect> = effects.into_iter().filter(|e| !e.is_finished(&step)).collect();

    if step % 2 == 0 {
        let mut result = None;
        for i in 0..buffs.len() {
            let new_buff = &buffs[i];
            if player.mana >= new_buff.mana() && !active_effects.iter().any(|e| e.buff == new_buff) {
                let mut new_active_effects = active_effects.clone();
                new_active_effects.push(&new_buff.effect(step));
                let player = player.spend_mana(new_buff.cost());
                let r = game(&player, &boss, buffs, new_active_effects, step + 1).map(|c| {
                    c + new_buff.cost()
                });
                if result.is_none() || (r.is_some() && r.unwrap() < result.unwrap()) {
                    result = r;
                }
            }
            if step <= 2 {
                println!("{}:{}", step, i);
            }
        }
        return result;
    } else {
        let player = player.damage(boss.damage);
        return game(&player, &boss, buffs, active_effects, step + 1);
    }
}

pub fn run_e() {
    let input = File::open("src/aoc/y2015/task_22").unwrap();
    let input = BufReader::new(input);
}