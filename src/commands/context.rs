use serenity_self::all::{ChannelId, Http};
use std::sync::Arc;
use tokio::sync::oneshot::Sender;

use super::{
    CollectorType,
    types::{CommandFeedback, CommandType},
};

pub struct CommandContext {
    pub command_type: CommandType,
    pub result_tx: Sender<Option<CommandFeedback>>,
    pub collector: CollectorType,
    pub target_channel: ChannelId,
    pub http: Arc<Http>,
}
