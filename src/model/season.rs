use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Autumn,
}

impl FromStr for Season {
    type Err = ();

    fn from_str(s: &str) -> Result<Season, ()> {
        match s {
            "winter" => Ok(Season::Winter),
            "spring" => Ok(Season::Spring),
            "summer" => Ok(Season::Summer),
            "autumn" => Ok(Season::Autumn),
            _ => Err(()),
        }
    }
}

impl Season {
    pub fn get_month_range(&self) -> (u8, u8) {
        match self {
            &Season::Winter => (1, 3),
            &Season::Spring => (4, 6),
            &Season::Summer => (7, 9),
            &Season::Autumn => (10, 12),
        }
    }
}
