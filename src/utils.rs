use std::str::FromStr;

use chrono::{DateTime, Duration, Utc};
use regex::Regex;

use crate::snipers::Statistics;

pub fn extract_statistics(text: &str) -> Option<Statistics> {
    fn parse_num<T: FromStr>(s: &str) -> Option<T> {
        let mut s = s.replace(".", "");
        s = s.replace(",", "");
        s.parse::<T>().ok()
    }

    fn parse_duration_from_line(
        line_index: usize,
        lines: &[&str],
        values_str: &mut Vec<&str>,
    ) -> Option<DateTime<Utc>> {
        let regex_get_number = Regex::new(r"\d+(?:[.,]\d{3})*").unwrap();
        let line = lines.get(line_index)?;
        let count = regex_get_number.find_iter(line).count();
        if count == 2 {
            let min_str = values_str.remove(line_index + 1);
            Some(
                Utc::now()
                    + Duration::hours(parse_num(values_str[line_index])?)
                    + Duration::minutes(parse_num(min_str).unwrap_or(0)),
            )
        } else if count == 1 {
            Some(Utc::now() + Duration::minutes(parse_num(values_str[line_index])?))
        } else {
            None
        }
    }

    let regex_get_number = Regex::new(r"\d+(?:[.,]\d{3})*").unwrap();
    let mut values_str: Vec<&str> = regex_get_number
        .find_iter(text)
        .map(|m| m.as_str())
        .collect();
    let mut lines: Vec<&str> = text.lines().collect();

    // This split is necessary because only in English this information comes as a single line,
    // while in other languages it's already split into two separate lines.
    if text.contains("you") && lines.len() > 1 {
        let parts: Vec<&str> = lines[1].splitn(2, ". ").collect();
        if parts.len() == 2 {
            lines.splice(1..2, vec![parts[0], parts[1]]);
        }
    }

    let claim_time: DateTime<Utc> = parse_duration_from_line(0, &lines, &mut values_str)?;
    let rolls_remaining: u8 = parse_num::<u8>(values_str[1])?;
    let next_rolls: DateTime<Utc> = parse_duration_from_line(2, &lines, &mut values_str)?;
    let next_daily: DateTime<Utc> =
        parse_duration_from_line(3, &lines, &mut values_str).unwrap_or(Utc::now());
    let next_kakera_react: DateTime<Utc> =
        parse_duration_from_line(4, &lines, &mut values_str).unwrap_or(Utc::now());
    let kakera_power: u8 = parse_num::<u8>(values_str[5])?;
    let kakera_cost: u8 = parse_num::<u8>(values_str[6])?;
    let kakera_cost_half: u8 = kakera_cost / 2; // skip line 7
    let kakera_stock: u32 = parse_num::<u32>(values_str[8])?;
    let next_rt: Option<DateTime<Utc>> = if lines[9].contains("$rt") {
        parse_duration_from_line(9, &lines, &mut values_str)
    } else {
        None
    };
    let next_dk: DateTime<Utc> = if lines[10].contains("$dk") {
        parse_duration_from_line(9, &lines, &mut values_str).unwrap_or(Utc::now())
    } else {
        Utc::now()
    };
    let rolls_reset_stock = parse_num::<u16>(values_str[10])?;

    let status = Statistics {
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
    };
    Some(status)
}
