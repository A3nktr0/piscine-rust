#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub age: u8,
}

impl Member {
    pub fn get_promotion(&mut self) {
        self.role = match self.role {
            Role::Associate => Role::Soldier,
            Role::Soldier => Role::Caporegime,
            Role::Caporegime => Role::Underboss,
            Role::Underboss => Role::Underboss,
        };
    }

    pub fn new(name: &str, role: Role, age: u8) -> Self {
        Self {
            name: name.to_string(),
            role,
            age,
        }
    }
}

// pub fn new(name: &str, role: Role, age: u8) -> Member {
//     Member {
//         name: name.to_string(),
//         role,
//         age,
//     }
// }
