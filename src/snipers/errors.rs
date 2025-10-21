use serenity_self::all::{ActionRowComponent, ButtonKind};

pub enum CaptureError {
    MissingComponent,
    NotAButton(ActionRowComponent),
    InvalidButton(ButtonKind),
}
