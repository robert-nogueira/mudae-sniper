use serenity_self::all::Http;
use std::sync::Arc;
use tokio::sync::oneshot::Sender;

use crate::entities::instance::Instance;

use super::{
    CollectorType,
    types::{CommandFeedback, CommandType},
};

pub struct CommandContext {
    pub command_type: CommandType,
    pub result_tx: Sender<Option<CommandFeedback>>,
    pub collector: CollectorType,
    pub target_instance: Instance,
    pub http: Arc<Http>,
}
