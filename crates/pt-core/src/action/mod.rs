//! Action execution system.

pub mod executor;
pub mod recovery;
#[cfg(unix)]
pub mod signal;

pub use executor::{
    ActionError, ActionExecutor, ActionResult, ActionRunner, ActionStatus, ExecutionError,
    ExecutionResult, ExecutionSummary, IdentityProvider, NoopActionRunner, StaticIdentityProvider,
};
pub use recovery::{plan_recovery, ActionFailure, FailureKind, RecoveryDecision, RetryPolicy};
#[cfg(target_os = "linux")]
pub use signal::LiveIdentityProvider;
#[cfg(unix)]
pub use signal::{SignalActionRunner, SignalConfig};
