#[macro_use] extern crate vst2;
extern crate libc;

use vst2::buffer::AudioBuffer;
use vst2::plugin::{Plugin, Info};
use vst2::editor::Editor;

use libc::c_void;

struct TapeSaturation {
    a: f32,
    b: f32,

    input_l_prev: f32,
    input_r_prev: f32,
    output_l_prev: f32,
    output_r_prev: f32,

    editor: TapeSaturationEditor,
}

impl Default for TapeSaturation {
    fn default() -> TapeSaturation {
        TapeSaturation {
            a: 1.0,
            b: 1.0,

            input_l_prev: 0.0,
            input_r_prev: 0.0,
            output_l_prev: 0.0,
            output_r_prev: 0.0,

            editor: Default::default()
        }
    }
}

impl Plugin for TapeSaturation {
    fn get_info(&self) -> Info {
        Info {
            name: "TapeSaturation".to_string(),
            vendor: "micah".to_string(),
            unique_id: 23781428,

            inputs: 2,
            outputs: 2,
            parameters: 2,

            ..Info::default()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.a,
            1 => self.b,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.a = value.max(0.001),
            1 => self.b = value.max(0.001),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "a".to_string(),
            1 => "b".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}", self.a * 100.0),
            1 => format!("{}", self.b * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            1 => "%".to_string(),
            _ => "".to_string(),
        }
    }

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        fn update(output_prev: f32, input_prev: f32, input: f32, a: f32, b: f32) -> f32 {
            let delta_input = input - input_prev;
            output_prev + a * ((input * 2.0).tanh() - output_prev) * delta_input.abs() + b * delta_input / (input * 2.0).cosh().powi(2)
        }

        let (inputs, mut outputs) = buffer.split();

        let (inputs_left, inputs_right) = inputs.split_at(1);
        let (mut outputs_left, mut outputs_right) = outputs.split_at_mut(1);

        let inputs_stereo = inputs_left[0].iter().zip(inputs_right[0].iter());
        let outputs_stereo = outputs_left[0].iter_mut().zip(outputs_right[0].iter_mut());

        for (input_pair, output_pair) in inputs_stereo.zip(outputs_stereo) {
            let (input_l, input_r) = input_pair;
            let (output_l, output_r) = output_pair;

            *output_l = update(self.output_l_prev, self.input_l_prev, *input_l, self.a, self.b);

            self.input_l_prev = *input_l;
            self.output_l_prev = *output_l;

            *output_r = update(self.output_r_prev, self.input_r_prev, *input_r, self.a, self.b);

            self.input_r_prev = *input_r;
            self.output_r_prev = *output_r;
        }
    }

    fn get_editor(&mut self) -> Option<&mut Editor> {
        Some(&mut self.editor)
    }
}

struct TapeSaturationEditor {
    is_open: bool,
}

impl Default for TapeSaturationEditor {
    fn default() -> TapeSaturationEditor {
        TapeSaturationEditor {
            is_open: false,
        }
    }
}

impl Editor for TapeSaturationEditor {
    fn size(&self) -> (i32, i32) {
        (320, 240)
    }

    fn position(&self) -> (i32, i32) {
        (100, 100)
    }

    fn open(&mut self, window: *mut c_void) {

        self.is_open = true;
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }
}

plugin_main!(TapeSaturation);
