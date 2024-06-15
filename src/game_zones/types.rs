#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DamageType {
    None,
    Fire,
    Lightning,
    Force,
    Divine,
    Necrotic,
    Acid,
    Ice,
    Psychic
}

#[derive(Debug)]
pub struct DamageTypeParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dice {
    sides: u8
}

#[derive(Debug)]
pub enum ParseDiceError {
    /// Could not parse valid u8 after "d"
    InvalidSides(std::num::ParseIntError),
    /// Valid format starts with "d" (e.g. "d8")
    InvalidFormat
}

impl From<std::num::ParseIntError> for ParseDiceError {
    fn from(value: std::num::ParseIntError) -> Self {
        ParseDiceError::InvalidSides(value)
    }   
}

impl Dice {
    pub fn get_sides(self) -> u8 {
        self.sides
    }
    // TODO: roll()
}

impl TryFrom<&str> for Dice {
    type Error = ParseDiceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with('d') {
            let sides = value[1..].parse::<u8>()?;
            return Ok(Dice{ sides });
        }
        Err(Self::Error::InvalidFormat)
    }
}

impl TryFrom<String> for Dice {
    type Error = ParseDiceError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        return Dice::try_from(value.as_str());
    }
}
