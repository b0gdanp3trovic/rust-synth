use std::sync::Arc;

use nih_plug::{params::{FloatParam, Params}, prelude::FloatRange};

#[derive(Params)]
struct SynthParams {
    #[id = "gain"]
    gain: FloatParam,

    #[id = "attack"]
    attack: FloatParam,

    #[id = "release"]
    release: FloatParam,
}

struct MonoSynth {
    params: Arc<SynthParams>,
    sample_rate: f32,
    gate: bool,
    freq: f32,
    phase: f32,
    env: f32
}

impl Default for MonoSynth {
    fn default() -> Self {
        let params = Arc::new(SynthParams {
            gain: FloatParam::new("Gain", 0.2, FloatRange::Linear {min: 0.0, max: 1.0}),
            attack: FloatParam::new("Attack", 0.01, FloatRange::Linear {min: 0.001, max: 1.0}),
            release: FloatParam::new("Release", 0.2, FloatRange::Linear {min: 0.001, max: 1.0}),
        });

        Self {
            params,
            sample_rate: 44100.0,
            gate: false,
            freq: 440.0,
            phase: 0.0,
            env: 0.0
        }
    }
}

impl MonoSynth {
    fn next_sample(&mut self) -> f32 {
        let phase_inc = self.freq / self.sample_rate;
        self.phase += phase_inc;

        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        let osc = (2.0 * std::f32::consts::PI * self.phase).sin();
        osc * self.env * self.params.gain.value()
    }
}
