use std::sync::Arc;

use crate::synth::{osc::OscType, params::SynthParams};

pub struct MonoSynth {
    pub params: Arc<SynthParams>,
    pub sample_rate: f32,
    pub gate: bool,
    pub freq: f32,
    pub phase: f32,
    pub env: f32,
}

impl Default for MonoSynth {
    fn default() -> Self {
        Self {
            params: Arc::new(SynthParams::default()),
            sample_rate: 44100.0,
            gate: false,
            freq: 440.0,
            phase: 0.0,
            env: 0.0,
        }
    }
}

impl MonoSynth {
    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
    }

    pub fn note_on(&mut self, freq: f32) {
        self.freq = freq;
        self.gate = true;
    }

    pub fn note_off(&mut self) {
        self.gate = false;
    }

    fn update_env(&mut self) {
        if self.gate {
            let attack = self.params.attack.value().max(0.001);
            let a = 1.0 - (-1.0 / (attack * self.sample_rate)).exp();
            self.env += (1.0 - self.env) * a;
        } else {
            let release = self.params.release.value().max(0.001);
            let a = 1.0 - (-1.0 / (release * self.sample_rate)).exp();
            self.env -= self.env * a;
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        self.update_env();

        let phase_inc = self.freq / self.sample_rate;
        self.phase += phase_inc;
        if self.phase >= 1.0 { self.phase -= 1.0; }

        let osc_type = self.params.osc.value();
        let osc = Self::osc_sample(self.phase, osc_type);

        osc * self.env * self.params.gain.value()
    }

    fn osc_sample(phase: f32, osc: OscType) -> f32 {
        match osc {
            OscType::Sine => (2.0 * std::f32::consts::PI * phase).sin(),
            OscType::Saw => 2.0 * (phase - 0.5),
            OscType::Square => if phase < 0.5 { 1.0 } else { -1.0 },
            OscType::Triangle => 4.0 * (phase - 0.5).abs() - 1.0,
        }
    }
}
