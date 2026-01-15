//! Action execution system.

pub mod executor;

pub use executor::{
    ActionError, ActionExecutor, ActionResult, ActionStatus, ExecutionError, ExecutionResult,
    ExecutionSummary, NoopActionRunner, StaticIdentityProvider,
};
