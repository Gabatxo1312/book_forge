#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

pub mod config;
pub mod handlers;
pub mod helpers;
pub mod services;
