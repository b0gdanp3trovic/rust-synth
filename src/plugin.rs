use std::{num::NonZeroU32, sync::Arc};

use nih_plug::{buffer::Buffer, midi::{MidiConfig, NoteEvent}, params::Params, plugin::{Plugin, ProcessStatus}, prelude::{AudioIOLayout, AuxiliaryBuffers, BufferConfig, InitContext, ProcessContext, Vst3Plugin, Vst3SubCategory}};

use crate::synth::MonoSynth;

pub struct MonoSynthPlugin {
    pub synth: MonoSynth,
}

impl Default for MonoSynthPlugin {
    fn default() -> Self {
        Self {
            synth: MonoSynth::default(),
        }
    }
}

impl Plugin for MonoSynthPlugin {
    const NAME: &'static str = "BODA";
    const VENDOR: &'static str = "BODACORP";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = "0.1.0";

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: Some(NonZeroU32::new(2).unwrap()),
            ..AudioIOLayout::const_default()
        },
    ];

    type SysExMessage = ();
    type BackgroundTask = ();


    fn params(&self) -> Arc<dyn Params> {
        self.synth.params.clone()
    }

    fn initialize(
        &mut self,
        _layout: &AudioIOLayout,
        cfg: &BufferConfig,
        _ctx: &mut impl InitContext<Self>,
    ) -> bool {
        self.synth.set_sample_rate(cfg.sample_rate);
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Read events (MIDI / note events)
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { note, .. } => {
                    let freq = 440.0 * 2f32.powf((note as f32 - 69.0) / 12.0);
                    self.synth.note_on(freq);
                }
                NoteEvent::NoteOff { .. } => self.synth.note_off(),
                _ => {}
            }
        }

        // Render audio
        for mut channel_samples in buffer.iter_samples() {
            let s = self.synth.next_sample();
            for out in channel_samples.iter_mut() {
                *out = s;
            }
        }

        ProcessStatus::Normal
    } 
}

impl Vst3Plugin for MonoSynthPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"BODABODMonoSynth";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}