#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};
use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(format!("Invalid antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("Invalid Rh factor: {}", s)),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.antigen == other.antigen {
            self.rh_factor.cmp(&other.rh_factor)
        } else {
            self.antigen.cmp(&other.antigen)
        }
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antigen = s
            .chars()
            .take_while(|c| c.is_alphabetic())
            .collect::<String>()
            .parse::<Antigen>()?;

        let rh_factor = s
            .chars()
            .skip_while(|c| c.is_alphabetic())
            .collect::<String>()
            .parse::<RhFactor>()?;

        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = match self.antigen {
            Antigen::A => "A",
            Antigen::AB => "AB",
            Antigen::B => "B",
            Antigen::O => "O",
        };
        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{:?}", format!("{}{}", a, rh))
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &BloodType) -> bool {
        self.donors().contains(other)
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut donors = Vec::new();

        let antigen = match self.antigen {
            Antigen::A => vec![Antigen::A, Antigen::O],
            Antigen::AB => vec![Antigen::A, Antigen::B, Antigen::AB, Antigen::O],
            Antigen::B => vec![Antigen::B, Antigen::O],
            Antigen::O => vec![Antigen::O],
        };

        antigen.iter().for_each(|antigen| match self.rh_factor {
            RhFactor::Positive => {
                vec![RhFactor::Positive, RhFactor::Negative]
                    .iter()
                    .for_each(|rh_factor| {
                        donors.push(BloodType {
                            antigen: antigen.clone(),
                            rh_factor: rh_factor.clone(),
                        });
                    });
            }
            RhFactor::Negative => {
                donors.push(BloodType {
                    antigen: antigen.clone(),
                    rh_factor: RhFactor::Negative,
                });
            }
        });

        donors
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut recipients = Vec::new();

        let antigen = match self.antigen {
            Antigen::O => vec![Antigen::A, Antigen::B, Antigen::AB, Antigen::O],
            Antigen::A => vec![Antigen::A, Antigen::AB],
            Antigen::B => vec![Antigen::B, Antigen::AB],
            Antigen::AB => vec![Antigen::AB],
        };

        antigen.iter().for_each(|antigen| match self.rh_factor {
            RhFactor::Negative => {
                vec![RhFactor::Positive, RhFactor::Negative]
                    .iter()
                    .for_each(|rh_factor| {
                        recipients.push(BloodType {
                            antigen: antigen.clone(),
                            rh_factor: rh_factor.clone(),
                        });
                    });
            }
            RhFactor::Positive => {
                recipients.push(BloodType {
                    antigen: antigen.clone(),
                    rh_factor: RhFactor::Positive,
                });
            }
        });
        recipients
    }
}
