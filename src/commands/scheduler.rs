use super::context::CommandContext;
use super::{CollectorType, CommandFeedback};
use log::{debug, info};
use serenity_self::all::{
    ChannelId, MessageCollector, ReactionCollector, ShardMessenger,
};
use std::sync::Arc;
use std::time::Duration as TimeDuration;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{
    UnboundedReceiver, UnboundedSender, unbounded_channel,
};
use tokio::time::sleep;

pub struct CommandScheduler {
    rx: Mutex<UnboundedReceiver<CommandContext>>,
    tx: UnboundedSender<CommandContext>,
}

impl CommandScheduler {
    pub fn new() -> Arc<Self> {
        let (tx, rx) = unbounded_channel::<CommandContext>();
        Arc::new(Self {
            rx: Mutex::new(rx),
            tx,
        })
    }

    pub fn sender(&self) -> UnboundedSender<CommandContext> {
        self.tx.clone()
    }

    pub fn default_message_collector(
        &self,
        shard: &ShardMessenger,
        target_channel: ChannelId,
    ) -> MessageCollector {
        MessageCollector::new(shard)
            .channel_id(target_channel)
            .author_id(432610292342587392.into())
            .timeout(TimeDuration::from_secs(30))
    }

    pub fn default_reaction_collector(
        &self,
        shard: &ShardMessenger,
        target_channel: ChannelId,
    ) -> ReactionCollector {
        ReactionCollector::new(shard)
            .channel_id(target_channel)
            .author_id(432610292342587392.into())
            .timeout(TimeDuration::from_secs(30))
    }

    pub fn start(self: Arc<Self>) {
        info!(
            target: "mudae_sniper",
            "⏰ command scheduler started"
        );
        let this = self.clone();
        tokio::spawn(async move {
            while let Some(next) = {
                let mut rx = this.rx.lock().await;
                rx.recv().await
            } {
                self.task_execute(next).await;
                sleep(TimeDuration::from_millis(200)).await;
            }
        });
    }
}

impl CommandScheduler {
    pub async fn task_execute(&self, ctx: CommandContext) {
        debug!(
            target: "mudae_sniper",
            instance_id:? = ctx.target_channel;
            "⏰ command_scheduled: {}", ctx.command_type
        );
        ctx.target_channel
            .say(&ctx.http, ctx.command_type.to_string())
            .await
            .expect("fail on send {command:?}");
        match ctx.collector {
            CollectorType::Msg(collector) => {
                let feedback = collector.next().await;
                match feedback {
                    Some(f) => {
                        let _ = ctx
                            .result_tx
                            .send(Some(CommandFeedback::Msg(Box::new(f))));
                    }
                    None => {
                        let _ = ctx.result_tx.send(None);
                    }
                };
            }
            CollectorType::React(collector) => {
                let feedback = collector.next().await;
                match feedback {
                    Some(_) => {
                        let _ = ctx
                            .result_tx
                            .send(Some(CommandFeedback::React(())));
                    }
                    None => {
                        let _ = ctx.result_tx.send(None);
                    }
                };
            }
        }
    }
}
