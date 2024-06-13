use core::convert::{From, TryFrom};
use crate::game_zones::types::{DamageType, DamageTypeParseError};

/// Reserved symbols and keywords in our lanaguage
pub const SYMBOLS: [&'static str; 30] = [
    "{",
    "}",
    "(",
    ")",
    "[",
    "]",
    "@",
    "^",
    "#",
    ":",
    ";",
    "&",
    "+",
    "-",
    "*",
    "/",
    "|",
    ".",
    ",",
    "!",
    "~",
    "=",
    "in",
    "from",
    "for",
    "if",
    "else",
    "when",
    "func",
    "target",
];

/// This is a token => a fundamental piece of the language, representing an atomic syntactic unit
pub trait Token<T> {
    /// All tokens have a string value
    fn to_string(self) -> String;
    /// Some tokens have a parsed value from their string value
    fn get_value(self) -> T;
}

#[derive(Debug, Clone)]
pub struct BoolToken {
    string_value: String,
    bool_value: bool
}

impl Token<bool> for BoolToken {
    fn to_string(self) -> String {
        self.string_value
    }

    fn get_value(self) -> bool {
        self.bool_value
    }
}

impl TryFrom<String> for BoolToken {
    type Error = core::str::ParseBoolError;

    fn try_from(val: String) -> Result<Self, core::str::ParseBoolError> {
        let bool_val = val.parse::<bool>()?;
        return Ok(BoolToken { bool_value: bool_val, string_value: val });
    }
}

#[derive(Debug, Clone)]
pub struct StringToken {
    string_value: String
}

impl Token<String> for StringToken {
    fn to_string(self) -> String {
        self.string_value
    }

    fn get_value(self) -> String {
        self.string_value
    }
}

impl From<String> for StringToken {
    fn from(val: String) -> Self {
        StringToken {
            string_value: val
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntToken {
    string_value: String,
    int_value: u16
}

impl Token<u16> for IntToken {
    fn to_string(self) -> String {
        self.string_value
    }

    fn get_value(self) -> u16 {
        self.int_value
    }
}

impl TryFrom<String> for IntToken {
    type Error = std::num::ParseIntError;

    fn try_from(val: String) -> Result<Self, std::num::ParseIntError> {
        let int_val = val.parse::<u16>()?;
        return Ok(IntToken { int_value: int_val, string_value: val });
    }
}

#[derive(Debug, Clone)]
pub struct DamageTypeToken {
    string_value: String,
    damage_type: DamageType
}

impl Token<DamageType> for DamageTypeToken {
    fn to_string(self) -> String {
        self.string_value
    }

    fn get_value(self) -> DamageType {
        self.damage_type
    }
}

impl TryFrom<String> for DamageTypeToken {
    type Error = DamageTypeParseError;

    fn try_from(val: String) -> Result<Self, DamageTypeParseError> {
        match val.to_lowercase().as_str() {
            "fire" => Ok(DamageTypeToken{ string_value: val, damage_type: DamageType::Fire }),
            "lightning" => Ok(DamageTypeToken{ string_value: val, damage_type: DamageType::Lightning }),
            "acid" => Ok(DamageTypeToken{ string_value: val, damage_type: DamageType::Acid }),
            "necrotic" => Ok(DamageTypeToken{ string_value: val, damage_type: DamageType::Necrotic }),
            "ice" => Ok(DamageTypeToken{ string_value: val, damage_type: DamageType::Ice }),
            "psychic" => Ok(DamageTypeToken{ string_value: val, damage_type: DamageType::Psychic }),
            "force" => Ok(DamageTypeToken{ string_value: val, damage_type: DamageType::Force }),
            "divine" => Ok(DamageTypeToken{ string_value: val, damage_type: DamageType::Divine }),
            _ => Err(DamageTypeParseError)
        }
    }
}

/// The various token types
pub enum Tokens {
    /// Token is a numerical value
    Numeric(IntToken),
    /// Token is an identifier
    Identifier(StringToken),
    /// Token is a static keyword or symbol
    Symbol(StringToken),
    /// Token is a damage type literal
    DamageType(DamageTypeToken),
    /// Token is a boolean literal
    Boolean(BoolToken),
    // Double-slash token, which means we ignore everything until a newline
    Comment,
    /// Indicates the end of file (not really associated with a real token value)
    EOF
}