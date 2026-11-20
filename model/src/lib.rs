//! Data model crate for xkdPlanner.
//!
//! Contains serializable records and in-memory collections used by controllers.

// HINT: these are skeletons — parameters are named for teaching purposes and
// bodies are `todo!()`. Silence the resulting warnings while you fill them in.
#![allow(unused_variables, dead_code)]

pub mod collections;
pub mod records;

pub use collections::daily_collection::DailyCollection;
pub use collections::ptimer_collection::{PtimerCollection, PtimerSession};
pub use collections::timeline_collection::{TimelineCollection, TimelineEntry};
pub use records::slog_record::SlogRecord;
pub use records::task_record::{Task, STATUS_COMPLETED, STATUS_PENDING};
pub use records::user_record::User;
