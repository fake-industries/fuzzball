#[macro_use]
extern crate vst;

mod editor;
mod parameters;
mod processor;

use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, Info, Plugin, PluginParameters};

use std::sync::Arc;

struct FuzzBall {
    parameters: Arc<parameters::FuzzBallParameters>,
}

impl Default for FuzzBall {
    fn default() -> FuzzBall {
        FuzzBall {
            parameters: Arc::new(parameters::FuzzBallParameters::default()),
        }
    }
}

impl Plugin for FuzzBall {
    fn get_info(&self) -> Info {
        Info {
            name: "Fuzz Ball".to_string(),
            vendor: "Fake Industries".to_string(),
            unique_id: 999666999,
            version: 1,
            inputs: 2,
            outputs: 2,
            parameters: 3,
            category: Category::Effect,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        processor::FuzzBallProcessor::process(self.parameters.clone(), buffer);
    }

    fn get_parameter_object(&mut self) -> Arc<dyn (PluginParameters)> {
        Arc::clone(&self.parameters) as Arc<dyn (PluginParameters)>
    }

    fn get_editor(&mut self) -> Option<Box<dyn (Editor)>> {
        editor::PluginUI::new(self.parameters.clone())
    }
}

plugin_main!(FuzzBall);
