pub mod context;
pub mod scheduler;
pub mod types;

use std::sync::{Arc, LazyLock};

pub use context::*;
pub use scheduler::*;
pub use types::*;

pub static COMMAND_SCHEDULER: LazyLock<Arc<CommandScheduler>> =
    LazyLock::new(|| {
        let scheduler = Arc::new(CommandScheduler::new());
        scheduler.clone().start();
        scheduler
    });
