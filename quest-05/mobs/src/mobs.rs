pub mod boss;
pub mod member;

pub use member::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: boss::Boss,
    pub members: Vec<member::Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members.push(member::Member::new(name, Role::Associate, age))
    }

    pub fn attack(&mut self, enemy: &mut Mob) {
        let power = self.get_power();
        let enemy_power = enemy.get_power();

        if power <= enemy_power {
            self.lose_member(enemy);
        } else {
            enemy.lose_member(self);
        }
    }

    pub fn get_power(&self) -> usize {
        self.members
            .iter()
            .map(|m| match m.role {
                Role::Underboss => 4,
                Role::Caporegime => 3,
                Role::Soldier => 2,
                Role::Associate => 1,
            })
            .sum()
    }

    pub fn lose_member(&mut self, enemy: &mut Mob) {
        if self.members.is_empty() {
            self.lose_to(enemy);
        } else {
            self.members.pop();
            if self.members.is_empty() {
                self.lose_to(enemy);
            }
        }
    }

    pub fn lose_to(&mut self, enemy: &mut Mob) {
        enemy.cities.extend(self.cities.drain(..));
        enemy.wealth += self.wealth;
        self.wealth = 0;
    }

    pub fn steal(&mut self, enemy: &mut Mob, to_steal: u32) {
        if enemy.wealth < to_steal {
            self.wealth += enemy.wealth;
            enemy.wealth = 0;
        } else {
            self.wealth += to_steal;
            enemy.wealth -= to_steal;
        }
    }

    pub fn conquer_city(&mut self, mobs: Vec<Mob>, city_name: String, value: u8) {
        let exist = mobs
            .iter()
            .any(|mob| mob.cities.iter().any(|(name, _)| name == &city_name));
        if !exist {
            self.cities.push((city_name, value));
        }
    }
}
