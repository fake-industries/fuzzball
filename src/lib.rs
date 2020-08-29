#[macro_use]
extern crate vst;
extern crate vst_gui;

use vst::buffer::AudioBuffer;
use vst::editor::Editor;
use vst::plugin::{Category, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

use std::sync::Arc;

const MINIMUM_FUZZ: f32 = 0.01;
const MAXIMUM_FUZZ: f32 = 0.99;
const MAXIMUM_BOUNCE: f32 = 0.99;
const HEIGHT: i32 = 320;
const WIDTH: i32 = 340;
const HTML: &'static str = r#"
<!doctype html>
  <head>
    <meta charset="utf-8">
    <meta http-equiv="x-ua-compatible" content="ie=edge">
    <title>Fuzz Ball</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style type="text/css">
body {
  background: #D8D8D8;
  color: #A4A4A4;
  font-family: sans-serif;
}
label {
  color: #585858;
}
h1 {
  font-size: 3em;
  letter-spacing: 0.175em;
  margin: 0em;
  text-shadow: 4px 4px 0px rgba(0,0,0,0.1), 7px 7px 7px rgba(0, 0, 0, 1);
}
input[type=range] {
  -webkit-appearance: none;
  margin: 18px 0;
  width: 100%;
}
input[type=range]:focus {
  outline: none;
}
input[type=range]::-webkit-slider-runnable-track {
  background: #A4A4A4;
  border-radius: 1.3px;
  border: 0.2px solid #010101;
  box-shadow: 1px 1px 1px #000000, 0px 0px 1px #0d0d0d;
  cursor: pointer;
  height: 8.4px;
  width: 100%;
}
input[type=range]::-webkit-slider-thumb {
  -webkit-appearance: none;
  background: #ffffff;
  border: 1px solid #000000;
  border-radius: 3px;
  box-shadow: 1px 1px 1px #000000, 0px 0px 1px #0d0d0d;
  cursor: pointer;
  height: 36px;
  margin-top: -14px;
  width: 16px;
}
input[type=range]:focus::-webkit-slider-runnable-track {
  background: #585858;
}
input[type=range]::-moz-range-track {
  background: #A4A4A4;
  border: 0.2px solid #010101;
  border-radius: 1.3px;
  box-shadow: 1px 1px 1px #000000, 0px 0px 1px #0d0d0d;
  cursor: pointer;
  height: 8.4px;
  width: 100%;
}
input[type=range]::-moz-range-thumb {
  background: #ffffff;
  border: 1px solid #000000;
  border-radius: 3px;
  box-shadow: 1px 1px 1px #000000, 0px 0px 1px #0d0d0d;
  cursor: pointer;
  height: 36px;
  width: 16px;
}
input[type=range]::-ms-track {
  background: transparent;
  border-color: transparent;
  border-width: 16px 0;
  color: transparent;
  cursor: pointer;
  height: 8.4px;
  width: 100%;
}
input[type=range]::-ms-fill-lower {
  background: #2a6495;
  border: 0.2px solid #010101;
  border-radius: 2.6px;
  box-shadow: 1px 1px 1px #000000, 0px 0px 1px #0d0d0d;
}
input[type=range]::-ms-fill-upper {
  background: #A4A4A4;
  border: 0.2px solid #010101;
  border-radius: 2.6px;
  box-shadow: 1px 1px 1px #000000, 0px 0px 1px #0d0d0d;
}
input[type=range]::-ms-thumb {
  background: #ffffff;
  border: 1px solid #000000;
  border-radius: 3px;
  box-shadow: 1px 1px 1px #000000, 0px 0px 1px #0d0d0d;
  cursor: pointer;
  height: 36px;
  width: 16px;
}
input[type=range]:focus::-ms-fill-lower {
  background: #A4A4A4;
}
input[type=range]:focus::-ms-fill-upper {
  background: #585858;
}
button {
  background: #A4A4A4;
  border: 0.2px solid #010101;
  border-radius: 1.3px;
  box-shadow: 1px 1px 1px #000000, 0px 0px 1px #0d0d0d;
  color: #ffffff;
  cursor: pointer;
  font-weight: bold;
}
.center {
  text-align: center;
}
    </style>
  </head>
    <body>
      <h1 class="center">Fuzz Ball</h1>
      <label for="fuzz">Fuzz</label>
      <input id="fuzz" type="range" min="0" max="1.0" value="0.5" step="0.01"/>
      <br/>
      <label for="bounce">Bounce</label>
      <input id="bounce" type="range" min="0" max="1.0" value="0.5" step="0.01"/>
      <br/>
      <label for="volume">Volume</label>
      <input id="volume" type="range" min="0" max="1.0" value="0.5" step="0.01"/>
      <br />
      <div class="center">
        <button onclick="setup('eight-ball');">8 Ball</button>
        <button onclick="setup('first-ball');">First Ball</button>
        <button onclick="setup('hyper-ball');">Hyper Ball</button>
        <button onclick="setup('wrecking-ball');">Wrecking Ball</button>
      </div>
      <hr />
      <div class="center">
        Built by Fake Industries.
      </div>
    </body>
    <script>
var fuzz = document.getElementById("fuzz");
var bounce = document.getElementById("bounce");
var volume = document.getElementById("volume");

function setup(ball) {
  if (ball == "eight-ball") {
      fuzz.value = 0.05;
      bounce.value = 0.25;
      volume.value = 0.60;
  } else if (ball == "first-ball") {
      fuzz.value = 0.5;
      bounce.value = 0.5;
      volume.value = 0.5;
  } else if (ball == "hyper-ball") {
      fuzz.value = 0.81;
      bounce.value = 0.18;
      volume.value = 0.61;
  } else if (ball == "wrecking-ball") {
      fuzz.value = 0.97;
      bounce.value = 0.94;
      volume.value = 0.90;
  };

  fuzz.dispatchEvent(new Event('change'));
  bounce.dispatchEvent(new Event('change'));
  volume.dispatchEvent(new Event('change'));
};

fuzz.value = external.invoke("getFuzz");
bounce.value = external.invoke("getBounce");
volume.value = external.invoke("getVolume");

fuzz.addEventListener("change", function(event) {
  external.invoke("setFuzz " + event.target.value);
});
bounce.addEventListener("change", function(event) {
  external.invoke("setBounce " + event.target.value);
});
volume.addEventListener("change", function(event) {
  external.invoke("setVolume " + event.target.value);
});
    </script>
  </head>
</html>
"#;

struct FuzzBall {
    params: Arc<FuzzBallParameters>,
}

struct FuzzBallParameters {
    fuzz: AtomicFloat,
    bounce: AtomicFloat,
    volume: AtomicFloat,
}

fn javascript_callback(parameters: Arc<FuzzBallParameters>) -> vst_gui::JavascriptCallback {
    Box::new(move |message: String| {
        let mut tokens = message.split_whitespace();

        let command = tokens.next().unwrap_or("");
        let argument = tokens.next().unwrap_or("").parse::<f32>();

        match command {
            "getFuzz" => {
                return parameters.fuzz.get().to_string();
            }
            "getBounce" => {
                return parameters.bounce.get().to_string();
            }
            "getVolume" => {
                return parameters.volume.get().to_string();
            }
            "setFuzz" => {
                if argument.is_ok() {
                    parameters.fuzz.set(argument.unwrap());
                }
            }
            "setBounce" => {
                if argument.is_ok() {
                    parameters.bounce.set(argument.unwrap());
                }
            }
            "setVolume" => {
                if argument.is_ok() {
                    parameters.volume.set(argument.unwrap());
                }
            }
            _ => {}
        }

        String::new()
    })
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
        let mut bounce = self.params.bounce.get();
        let volume = self.params.volume.get();

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

    fn get_editor(&mut self) -> Option<Box<dyn(Editor)>> {
        let gui = vst_gui::new_plugin_gui(
            String::from(HTML),
            javascript_callback(self.params.clone()),
            Some((WIDTH, HEIGHT)),
        );
        Some(Box::new(gui))
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
