use std::fmt::Display;

pub enum Kakera {
    Purple(u16),
    Blue(u16),
    Teal(u16),
    Green(u16),
    Yellow(u16),
    Orange(u16),
    Red(u16),
    Rainbow(u16),
    Light(u16),
}

impl Kakera {
    pub fn value(&self) -> u16 {
        match self {
            Kakera::Blue(v)
            | Kakera::Green(v)
            | Kakera::Light(v)
            | Kakera::Orange(v)
            | Kakera::Purple(v)
            | Kakera::Rainbow(v)
            | Kakera::Red(v)
            | Kakera::Teal(v)
            | Kakera::Yellow(v) => *v,
        }
    }

    pub fn from_emoji_id(id: u64, value: u16) -> Option<Kakera> {
        match id {
            1097914822462545951 => Some(Kakera::Purple(value)),
            1097914834244337784 => Some(Kakera::Blue(value)),
            1097914851772342322 => Some(Kakera::Teal(value)),
            1097914861419245639 => Some(Kakera::Green(value)),
            1097914885343543407 => Some(Kakera::Yellow(value)),
            1097914894558433451 => Some(Kakera::Orange(value)),
            1097914903915925716 => Some(Kakera::Red(value)),
            608192076286263297 => Some(Kakera::Rainbow(value)),
            1097914945699581973 => Some(Kakera::Light(value)),
            _ => None,
        }
    }
}

impl Display for Kakera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kakera::Blue(v) => write!(f, "Blue({v})"),
            Kakera::Green(v) => write!(f, "Green({v})"),
            Kakera::Light(v) => write!(f, "Light({v})"),
            Kakera::Orange(v) => write!(f, "Orange({v})"),
            Kakera::Purple(v) => write!(f, "Purple({v})"),
            Kakera::Rainbow(v) => write!(f, "Rainbow({v})"),
            Kakera::Red(v) => write!(f, "Red({v})"),
            Kakera::Teal(v) => write!(f, "Teal({v})"),
            Kakera::Yellow(v) => write!(f, "Yellow({v})"),
        }
    }
}
