#[macro_use]
extern crate vst;

use vst::buffer::AudioBuffer;
use vst::plugin::{Category, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

use std::sync::Arc;

const MINIMUM_FUZZ: f32 = 0.01;
const MAXIMUM_FUZZ: f32 = 0.99;

struct FuzzBall {
    params: Arc<FuzzBallParameters>,
}

struct FuzzBallParameters {
    fuzz: AtomicFloat,
    bounce: AtomicFloat,
    volume: AtomicFloat,
}

impl Default for FuzzBall {
    fn default() -> FuzzBall {
        FuzzBall { params: Arc::new(FuzzBallParameters::default()) }
    }
}

impl Default for FuzzBallParameters {
    fn default() -> FuzzBallParameters {
        FuzzBallParameters {
            fuzz: AtomicFloat::new(0.5),
            bounce: AtomicFloat::new(0.5),
            volume: AtomicFloat::new(0.5),
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
        let mut fuzz = self.params.fuzz.get();
        let volume = self.params.volume.get();
        let bounce = self.params.bounce.get();

        if fuzz > MAXIMUM_FUZZ {
            fuzz = MAXIMUM_FUZZ;
        } else if fuzz < MINIMUM_FUZZ {
            fuzz = MINIMUM_FUZZ;
        }

        for (input_buffer, output_buffer) in buffer.zip() {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {
                if bounce > 0.6 {
                    *output_sample = *input_sample * (volume) *
                        (fuzz.log(*input_sample) + input_sample.log(fuzz)) *
                        fuzz.log(bounce);
                } else if bounce < 0.4 {
                    *output_sample = *input_sample * (volume) *
                        (fuzz.log(*input_sample) - input_sample.log(fuzz) - fuzz.log(bounce));
                } else {
                    *output_sample = *input_sample * (volume) *
                        (fuzz.log(*input_sample) + input_sample.log(fuzz));
                }
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
            1 => self.bounce.get(),
            2 => self.volume.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, val: f32) {
        match index {
            0 => self.fuzz.set(val),
            1 => self.bounce.set(val),
            2 => self.volume.set(val),
            _ => (),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2}", self.fuzz.get()),
            1 => format!("{:.2}", (self.bounce.get() - 0.5) * 2f32),
            2 => format!("{:.2}", (self.volume.get() - 0.5) * 2f32),
            _ => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "fuzz",
            1 => "bounce",
            2 => "volume",
            _ => "",
        }.to_string()
    }
}

plugin_main!(FuzzBall);
