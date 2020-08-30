#[macro_use]
extern crate vst;

mod editor;
mod parameters;

use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, Info, Plugin, PluginParameters};

use std::sync::Arc;

const MINIMUM_FUZZ: f32 = 0.01;
const MAXIMUM_FUZZ: f32 = 0.99;
const MAXIMUM_BOUNCE: f32 = 0.99;

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
        let mut fuzz = self.parameters.fuzz.get();
        let mut bounce = self.parameters.bounce.get();
        let volume = self.parameters.volume.get();

        if fuzz > MAXIMUM_FUZZ {
            fuzz = MAXIMUM_FUZZ;
        } else if fuzz < MINIMUM_FUZZ {
            fuzz = MINIMUM_FUZZ;
        }

        if bounce > MAXIMUM_BOUNCE {
            bounce = MAXIMUM_BOUNCE;
        }

        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                if bounce > 0.6 {
                    *output_sample = *input_sample
                        * (volume)
                        * (fuzz.log(*input_sample) + input_sample.log(fuzz))
                        * fuzz.log(bounce);
                } else if bounce < 0.4 {
                    *output_sample = *input_sample
                        * (volume)
                        * (fuzz.log(*input_sample) - input_sample.log(fuzz) - fuzz.log(bounce));
                } else {
                    *output_sample = *input_sample
                        * (volume)
                        * (fuzz.log(*input_sample) + input_sample.log(fuzz));
                }
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn (PluginParameters)> {
        Arc::clone(&self.parameters) as Arc<dyn (PluginParameters)>
    }

    fn get_editor(&mut self) -> Option<Box<dyn (Editor)>> {
        editor::PluginUI::new(self.parameters.clone())
    }
}

plugin_main!(FuzzBall);
