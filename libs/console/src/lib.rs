// lib.rs
pub mod console;
pub mod command;
pub mod view;
mod command_executor;
mod key_handler;
mod app_handler;

pub use console::NodiumConsole;
