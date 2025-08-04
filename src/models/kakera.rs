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
    fn value(&self) -> u16 {
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
