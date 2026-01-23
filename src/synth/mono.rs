use std::sync::Arc;

use crate::synth::params::SynthParams;

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
            let attack = self.params.attack.value();
            self.env += 1.0 / (attack * self.sample_rate);
            if self.env > 1.0 { self.env = 1.0; }
        } else {
            let release = self.params.release.value();
            self.env -= 1.0 / (release * self.sample_rate);
            if self.env < 0.0 { self.env = 0.0; }
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        self.update_env();

        let phase_inc = self.freq / self.sample_rate;
        self.phase += phase_inc;
        if self.phase >= 1.0 { self.phase -= 1.0; }

        let osc = (2.0 * std::f32::consts::PI * self.phase).sin();
        osc * self.env * self.params.gain.value()
    }
}
