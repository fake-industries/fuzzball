#[macro_use]
extern crate vst;
extern crate rand;

use rand::random;
use vst::buffer::AudioBuffer;
use vst::plugin::{Category, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

use std::sync::Arc;

struct FuzzBall {
    params: Arc<FuzzBallParameters>,
}

struct FuzzBallParameters {
    fuzz: AtomicFloat,
}

impl Default for FuzzBall {
    fn default() -> FuzzBall {
        FuzzBall { params: Arc::new(FuzzBallParameters::default()) }
    }
}

impl Default for FuzzBallParameters {
    fn default() -> FuzzBallParameters {
        FuzzBallParameters { fuzz: AtomicFloat::new(0.5) }
    }
}

impl Plugin for FuzzBall {
    fn get_info(&self) -> Info {
        Info {
            name: "Fuzz Ball".to_string(),
            vendor: "Fake".to_string(),
            unique_id: 999666999,
            version: 1,
            inputs: 2,
            outputs: 2,
            parameters: 1,
            category: Category::Effect,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let fuzz = self.params.fuzz.get();
        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                *output_sample = *input_sample * (fuzz - (random::<f32>() * fuzz));
            }
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn(PluginParameters)> {
        Arc::clone(&self.params) as Arc<dyn(PluginParameters)>
    }
}

impl PluginParameters for FuzzBallParameters {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.fuzz.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, val: f32) {
        #[allow(clippy::single_match)]
        match index {
            0 => self.fuzz.set(val),
            _ => (),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2}", (self.fuzz.get() - 0.5) * 2f32),
            _ => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "fuzz",
            _ => "",
        }.to_string()
    }
}

plugin_main!(FuzzBall);
