use serenity_self::all::{ActionRowComponent, ButtonKind};

use crate::utils::InvalidStatisticsData;

pub enum CaptureError {
    MissingComponent,
    NotAButton(ActionRowComponent),
    InvalidButton(ButtonKind),
}

#[derive(Debug)]
pub enum UpdateStatisticsError {
    InvalidStatistics(InvalidStatisticsData),
    MissingCommandFeedback,
}
