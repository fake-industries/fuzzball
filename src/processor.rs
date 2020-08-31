use crate::parameters::FuzzBallParameters;
use vst::buffer::AudioBuffer;

use std::sync::Arc;

const MINIMUM_FUZZ: f32 = 0.01;
const MAXIMUM_FUZZ: f32 = 0.99;
const MAXIMUM_BOUNCE: f32 = 0.99;

pub struct FuzzBallProcessor;

impl FuzzBallProcessor {
    pub fn process(
        parameters: Arc<FuzzBallParameters>,
        audio_buffer: &mut AudioBuffer<f32>,
    ) {
        let mut fuzz = parameters.fuzz.get();
        let mut bounce = parameters.bounce.get();
        let volume = parameters.volume.get();

        if fuzz > MAXIMUM_FUZZ {
            fuzz = MAXIMUM_FUZZ;
        } else if fuzz < MINIMUM_FUZZ {
            fuzz = MINIMUM_FUZZ;
        }

        if bounce > MAXIMUM_BOUNCE {
            bounce = MAXIMUM_BOUNCE;
        }

        for (input_buffer, output_buffer) in audio_buffer.zip() {
            for (input_sample, output_sample) in
                input_buffer.iter().zip(output_buffer)
            {
                if *input_sample > 0.0 {
                    if bounce > 0.6 {
                        *output_sample = *input_sample
                            * (volume)
                            * (fuzz.log(*input_sample)
                                + input_sample.log(fuzz))
                            * fuzz.log(bounce);
                    } else if bounce < 0.4 {
                        *output_sample = *input_sample
                            * (volume)
                            * (fuzz.log(*input_sample)
                                - input_sample.log(fuzz)
                                - fuzz.log(bounce));
                    } else {
                        *output_sample = *input_sample
                            * (volume)
                            * (fuzz.log(*input_sample)
                                + input_sample.log(fuzz));
                    }
                }
            }
        }
    }
}
