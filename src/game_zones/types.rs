#[derive(Debug, Clone)]
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
pub struct  DamageTypeParseError;
