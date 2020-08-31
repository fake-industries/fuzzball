use crate::editor::PluginUI;
use crate::parameters::FuzzBallParameters;
use crate::processor::FuzzBallProcessor;
use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, Info, Plugin, PluginParameters};

use std::sync::Arc;

pub struct FuzzBall {
    parameters: Arc<FuzzBallParameters>,
}

impl Default for FuzzBall {
    fn default() -> FuzzBall {
        FuzzBall {
            parameters: Arc::new(FuzzBallParameters::default()),
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
        FuzzBallProcessor::process(self.parameters.clone(), buffer);
    }

    fn get_parameter_object(&mut self) -> Arc<dyn (PluginParameters)> {
        Arc::clone(&self.parameters) as Arc<dyn (PluginParameters)>
    }

    fn get_editor(&mut self) -> Option<Box<dyn (Editor)>> {
        PluginUI::new(self.parameters.clone())
    }
}
