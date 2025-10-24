use std::sync::LazyLock;

use chrono::{DateTime, Duration, Utc};
use chrono_tz::Tz;
use regex::Regex;

use crate::{
    entities::{
        badge::{Badge, BadgeLevel, BadgeType},
        statistics::Statistics,
    },
    settings::SETTINGS,
};

pub static REGEX_GET_NUMBERS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d+(?:[.,]\d{3})*").unwrap());

pub static REGEX_GET_BADGES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?im)^(?::[A-Za-z]+\w*:?)*\s*([^\d·\n:]+)\s+([IVXLC\d]+)\s+·[^\n]*?(\d{1,3}(?:[.,]\d{3})*)\s*:kakera:").unwrap()
});

#[derive(Debug)]
pub struct InvalidStatisticsData(&'static str);

impl std::fmt::Display for InvalidStatisticsData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "erro ao extrair estatísticas: {}", self.0)
    }
}

impl std::error::Error for InvalidStatisticsData {}

pub fn get_local_time() -> DateTime<Tz> {
    Utc::now().with_timezone(&SETTINGS.timezone)
}

pub fn extract_badges(text: &str) -> Vec<Badge> {
    let mut badges: Vec<Badge> = Vec::new();
    let matches = REGEX_GET_BADGES.captures_iter(text);
    for item in matches {
        let level_str: &str = &item[2];
        let level: BadgeLevel = BadgeLevel::from_roman_algarism(level_str)
            .expect("fail on extract badge level");
        let name: &str = &item[1];
        badges.push(Badge {
            badge_type: BadgeType::from_name(name)
                .expect("regex error on extract badge type"),
            level,
        });
    }
    badges
}

pub fn extract_statistics(
    text: &str,
) -> Result<Statistics, InvalidStatisticsData> {
    if !(11..13).contains(&text.lines().count()) {
        return Err(InvalidStatisticsData("invalid statistics input format"));
    };
    fn arr_to_duration(arr: &[u32; 2]) -> Duration {
        if arr[0] == 0 && arr[1] == 0 {
            return Duration::seconds(0);
        }
        println!("{:?}", arr);
        Duration::hours(arr[1] as i64) + Duration::minutes(arr[0] as i64)
    }

    let mut values: [[u32; 2]; 12] = [([0, 0]); 12];
    for (out_index, line) in text.lines().enumerate().take(12) {
        for (inner_index, value) in REGEX_GET_NUMBERS
            .find_iter(line)
            .filter_map(|x| x.as_str().parse::<u32>().ok())
            .enumerate()
            .take(2)
        {
            values[out_index][inner_index] = value;
        }
    }

    let now = get_local_time();
    let claim_time = now + arr_to_duration(&values[0]);
    let rolls_remaining = values[1][0] as u8;
    let next_rolls = now + arr_to_duration(&values[2]);
    let next_kakera_react = now + arr_to_duration(&values[3]);
    let kakera_power = values[4][0] as u8;
    let kakera_cost = values[5][0] as u8;
    let kakera_cost_half = values[6][1] as u8;
    let kakera_stock = values[7][0];
    let next_daily = now + arr_to_duration(&values[8]);
    let next_dk = now + arr_to_duration(&values[9]);
    let next_rt = if !values[10].is_empty() {
        Some(now + arr_to_duration(&values[10]))
    } else {
        None
    };
    let rolls_reset_stock = values[11][0] as u16;

    Ok(Statistics {
        claim_time,
        rolls_remaining,
        next_rolls,
        next_daily,
        next_kakera_react,
        kakera_power,
        kakera_cost,
        kakera_cost_half,
        kakera_stock,
        next_rt,
        next_dk,
        rolls_reset_stock,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_status_ptbr() {
        let text = "**allma_**, você __pode__ se casar agora mesmo! A próxima reinicialização é em **2h 40** min.
Você tem **10** rolls restantes.
A próxima reinicialização é em **40** min.
Você __pode__ pegar kakera agora!
Power: **100%**
Cada reação de kakera consume 100% de seu reaction power.
Seus Personagens com 10+ chaves consome metade do power (50%)
Stock: **257**:kakera:
Próximo reset do $daily em **9h 49** min.
$dk está pronto!
A recarga do $rt ainda não acabou. Tempo restante: **34h 35** min. ($rtu)
Você tem **32** rolls reset no estoque";
        let status = extract_statistics(text);
        assert!(status.is_ok());
    }

    #[test]
    fn test_get_status_en() {
        let text = "**allma_**, you __can__ claim right now! The next claim reset is in **1h 29** min.
You have **17** rolls left. Next rolls reset in **29** min.
You __can__ react to kakera right now!
Power: **110%**
Each kakera reaction consumes 36% of your reaction power.
Your characters with 10+ keys consume half the power (18%)
Stock: **7,611**<:kakera:469835869059153940>
Next $daily reset in **9h 45** min.
Next $dk in **9h 45** min.
The cooldown of $rt is not over. Time left: **36h 00** min. ($rtu)
You have **35** rolls reset in stock.";
        let status = extract_statistics(text);
        assert!(status.is_ok());
    }

    #[test]
    fn test_get_status_fr() {
        let text = "**allma_**, vous __pouvez__ vous marier dès maintenant ! Le prochain reset est dans **1h 29** min.
Vous avez **17** rolls restants.
Prochain rolls reset dans **29** min.
Vous __pouvez__ réagir aux kakera dès maintenant !
Power: **110%**
Chaque réaction à un kakera consomme 36% de votre pouvoir de réaction.
Vos personnages possédant 10+ keys consomment moitié moins de pouvoir (18%)
Stock: **7.611**<:kakera:469835869059153940>
Prochain $daily reset dans **9h 45** min.
Prochain $dk dans **9h 45** min.
$rt n'est pas encore disponible. Temps restant : **36h 00** min. ($rtu)
Vous avez **35** rolls reset en stock.";
        let status = extract_statistics(text);
        assert!(status.is_ok());
    }

    #[test]
    fn test_get_status_es() {
        let text =
				  "**allma_**, __puedes__ reclamar ahora mismo. El siguiente reclamo será en **1h 28** min.
Tienes **17** rolls restantes.
El siguiente reinicio será en **28** min.
¡__Puedes__ reaccionar a kakera en este momento!
Poder: **110%**
Cada reacción de kakera consume 36% de su poder de reacción.
Tus personajes con 10+ llaves, consumen la mitad del poder (18%)
Capital: **7.611**<:kakera:469835869059153940>
Siguiente reinicio de $daily en **9h 44** min.
Siguiente $dk en **9h 44** min.
El enfriamiento de $rt no ha terminado. Tiempo restante: **35h 59** min. ($rtu)
Tienes **35** reinicios de rolls en el inventario.";
        let status = extract_statistics(text);
        assert!(status.is_ok());
    }
}
