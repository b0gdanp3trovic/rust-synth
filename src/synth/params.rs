use nih_plug::{params::FloatParam, prelude::FloatRange};
use nih_plug::params::Params;

#[derive(Params)]
pub struct SynthParams {
    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "attack"]
    pub attack: FloatParam,

    #[id = "release"]
    pub release: FloatParam,
}

impl Default for SynthParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new("Gain", 0.2, FloatRange::Linear { min: 0.0, max: 1.0 }),
            attack: FloatParam::new("Attack", 0.01, FloatRange::Linear { min: 0.001, max: 1.0 }),
            release: FloatParam::new("Release", 0.2, FloatRange::Linear { min: 0.001, max: 1.0 }),
        }
    }
}