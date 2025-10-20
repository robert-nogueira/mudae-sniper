use serenity_self::all::{
    ChannelId, Http, MessageCollector, Reaction, ReactionCollector,
    ShardMessenger,
};
use std::sync::Arc;
use tokio::sync::oneshot::Sender;

use super::types::{CommandFeedback, CommandType, FeedbackType};

pub struct CommandContext {
    pub command_type: CommandType,
    pub result_tx: Sender<CommandFeedback>,
    pub shard: ShardMessenger,
    pub expected_feedback: FeedbackType,
    pub target_channel: ChannelId,
    pub http: Arc<Http>,
}

// impl CommandContext<Message> {
//     pub(super) async fn execute(&self) -> Message {
//         self.target_channel
//             .say(self.http.clone(), self.command.to_string())
//             .await
//             .expect("fail on send {command:?}");

//         let mut collector = MessageCollector::new(&self.shard)
//             .channel_id(self.target_channel)
//             .author_id(432610292342587392.into())
//             .timeout(std::time::Duration::from_secs(30))
//             .filter(move |m| REGEX_GET_NUMBERS.is_match(&m.content))
//             .stream();

//         collector.next().await.expect("no message feedback")
//     }
// }

// impl CommandContext<Reaction> {
//     pub(super) async fn execute(&self) -> Reaction {
//         self.target_channel
//             .say(self.http.clone(), self.command.to_string())
//             .await
//             .expect("fail on send {command:?}");

//         let mut collector = ReactionCollector::new(&self.shard)
//             .channel_id(self.target_channel)
//             .author_id(432610292342587392.into())
//             .timeout(std::time::Duration::from_secs(30))
//             .filter(move |r| matches!(&r.emoji, serenity_self::all::ReactionType::Unicode(u) if u == "✅"))
//             .stream();

//         collector.next().await.expect("no reaction feedback")
//     }
// }
