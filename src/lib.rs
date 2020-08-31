#[macro_use]
extern crate vst;

mod editor;
mod parameters;
mod plugin;
mod processor;

plugin_main!(plugin::FuzzBall);
