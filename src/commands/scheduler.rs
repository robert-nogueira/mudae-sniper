use super::context::CommandContext;
use super::types::{CommandFeedback, FeedbackType};
use crate::utils::{REGEX_GET_NUMBERS, get_local_time};
use chrono::DateTime;
use chrono_tz::Tz;
use serenity_self::all::{MessageCollector, ReactionCollector};
use serenity_self::futures::StreamExt;
use std::sync::Arc;
use std::time::Duration as TimeDuration;
use tokio::sync::Mutex;
use tokio::time::sleep;

struct CommandQueue {
    items: Vec<CommandContext>,
}

impl CommandQueue {
    fn new() -> Self {
        CommandQueue { items: Vec::new() }
    }

    fn push(&mut self, ctx: CommandContext) {
        self.items.push(ctx)
    }

    fn pop(&mut self) -> Option<CommandContext> {
        self.items.pop()
    }
}

pub struct CommandScheduler {
    // pub last_command: DateTime<Tz>,
    queue: Arc<Mutex<CommandQueue>>,
}

impl CommandScheduler {
    pub fn new() -> Self {
        CommandScheduler {
            // last_command: get_local_time(),
            queue: Arc::new(Mutex::new(CommandQueue::new())),
        }
    }

    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            loop {
                let next = {
                    let mut queue_guard = self.queue.lock().await;
                    queue_guard.pop()
                };

                if let Some(next) = next {
                    let result = self.task_execute_command(&next).await;
                    let _ = next.result_tx.send(result);
                } else {
                    sleep(std::time::Duration::from_millis(500)).await;
                }
            }
        });
    }

    pub async fn schedule_command(&self, ctx: CommandContext) {
        self.queue.lock().await.push(ctx);
    }

    pub async fn task_execute_command(
        &self,
        ctx: &CommandContext,
    ) -> CommandFeedback {
        ctx.target_channel
            .say(&ctx.http, ctx.command_type.to_string())
            .await
            .expect("fail on send {command:?}");
        let result: Option<CommandFeedback> = match ctx.expected_feedback {
            FeedbackType::Message => {
                let mut collector = MessageCollector::new(&ctx.shard)
                    .channel_id(ctx.target_channel)
                    .author_id(432610292342587392.into())
                    .timeout(TimeDuration::from_secs(30))
                    .filter(move |m| REGEX_GET_NUMBERS.is_match(&m.content))
                    .stream();
                collector.next().await.map(CommandFeedback::Msg)
            }
            FeedbackType::Reaction => {
                let mut collector = ReactionCollector::new(&ctx.shard)
                    .channel_id(ctx.target_channel)
                    .author_id(432610292342587392.into())
                    .timeout(TimeDuration::from_secs(30))
                    .filter(move |r| match &r.emoji {
                        serenity_self::all::ReactionType::Unicode(unicode) => {
                            unicode == "✅"
                        }
                        _ => false,
                    })
                    .stream();
                collector.next().await.map(CommandFeedback::React)
            }
        };
        result.unwrap()
    }
}
