use nih_plug::prelude::Enum;

#[derive(Enum, Debug, Copy, Clone, PartialEq, Eq)]
pub enum OscType {
    Sine,
    Saw,
    Square,
    Triangle,
}