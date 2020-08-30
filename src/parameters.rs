use vst::plugin::PluginParameters;
use vst::util::AtomicFloat;

pub struct FuzzBallParameters {
    pub fuzz: AtomicFloat,
    pub bounce: AtomicFloat,
    pub volume: AtomicFloat,
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
        }
        .to_string()
    }
}
