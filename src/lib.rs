use nih_plug::nih_export_vst3;

mod plugin;
mod synth;

nih_export_vst3!(plugin::MonoSynthPlugin);