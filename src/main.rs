#[derive(Debug)]
struct Hero {
    name: String,
    collected: bool,
    element: Element,
    class: Class,
    zodiac: Zodiac,
    gender: Gender,
    star: u8,
    awekened: u8,
    imprint: Imprint,
    related: Option<String>,
}

#[derive(Debug)]
enum Element {
    Fire,
    Ice,
    Earth,
    Light,
    Dark,
}

#[derive(Debug)]
enum Class{
    Warrior,
    Knight,
    Thief,
    Ranger,
    Mage,
    SoulWeaver,
    Ingredients,
}

#[derive(Debug)]
enum Zodiac {
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
enum Gender {
    Female,
    Male,
}

#[derive(Debug)]
enum Imprint {
    None,
    D,
    C,
    B,
    A,
    S,
    SS,
    SSS,
}

fn main() {
    let achates = Hero {
        name: String::from("Achates"),
        collected: true,
        element: Element::Fire,
        class: Class::SoulWeaver,
        zodiac: Zodiac::Gemini,
        gender: Gender::Female,
        star: 5,
        awekened: 0,
        imprint: Imprint::S,
        related: Some(String::from("Shooting Star Achates")),
    };

    if let Some(related) = achates.related {
        println!("{} is related to {}", achates.name, related);
    } else {
        println!("{} does not have a related hero", achates.name);
    }

    let basar = Hero {
        name: String::from("Basar"),
        collected: false,
        element: Element::Earth,
        class: Class::Mage,
        zodiac: Zodiac::Aquarius,
        gender: Gender::Male,
        star: 0,
        awekened: 0,
        imprint: Imprint::None,
        related: None,
    };

    if let Some(related) = basar.related {
        println!("{} is related to {}", basar.name, related);
    } else {
        println!("{} does not have a related hero", basar.name);
    }

}
