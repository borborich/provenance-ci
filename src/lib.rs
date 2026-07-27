pub mod c2pa_adapter;
pub mod config;
pub mod fetch;
pub mod model;
pub mod policy;
pub mod report;
pub mod runner;

pub use config::Config;
pub use model::{PolicyVerdict, RunResult};
pub use runner::{run, RunOptions};
