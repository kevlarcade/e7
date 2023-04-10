#[derive(Debug)]
pub struct Hero {
    pub name: String,
    pub collected: bool,
    pub element: Element,
    pub class: Class,
    pub zodiac: Zodiac,
    pub gender: Gender,
    pub star: u8,
    pub awekened: u8,
    pub imprint: Imprint,
    pub related: Option<String>,
}

#[derive(Debug)]
pub enum Element {
    Fire,
    Ice,
    Earth,
    Light,
    Dark,
}

#[derive(Debug)]
pub enum Class{
    Warrior,
    Knight,
    Thief,
    Ranger,
    Mage,
    SoulWeaver,
    Ingredients,
}

#[derive(Debug)]
pub enum Zodiac {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagitarius,
    Capricorn,
    Aquarius,
    Pisces,
}

#[derive(Debug)]
pub enum Gender {
    Female,
    Male,
}

#[derive(Debug)]
pub enum Imprint {
    None,
    D,
    C,
    B,
    A,
    S,
    SS,
    SSS,
}
