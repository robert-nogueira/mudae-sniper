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

    pub fn from_name(name: &str, value: u16) -> Option<Kakera> {
        match name {
            "kakeraP" => Some(Kakera::Purple(value)),
            "kakeraB" => Some(Kakera::Blue(value)),
            "kakeraT" => Some(Kakera::Teal(value)),
            "kakeraG" => Some(Kakera::Green(value)),
            "kakeraY" => Some(Kakera::Yellow(value)),
            "kakeraO" => Some(Kakera::Orange(value)),
            "kakeraR" => Some(Kakera::Red(value)),
            "kakeraW" => Some(Kakera::Rainbow(value)),
            "kakeraL" => Some(Kakera::Light(value)),
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
