use std::num::NonZeroU32;
use std::sync::Arc;
use apollo::apollo_export_vst3;
use apollo::audio_setup::{AudioIOLayout, AuxiliaryBuffers};
use apollo::buffer::Buffer;
use apollo::context::ProcessContext;
use apollo::midi::MidiConfig;
use apollo::params::Params;
use apollo::plugin::{Plugin, ProcessStatus};
use apollo::plugin::vst3::{Vst3Plugin, Vst3SubCategory};

pub struct SpectrumAnalyzer {
    params: Arc<SpectrumAnalyzerParams>
}

struct SpectrumAnalyzerParams {

}

impl Default for SpectrumAnalyzerParams {
    fn default() -> Self {
        Self {
        }
    }
}

impl Default for SpectrumAnalyzer {
    fn default() -> Self {
        Self {
            params: Arc::new(SpectrumAnalyzerParams::default()),
        }
    }
}

impl Plugin for SpectrumAnalyzer {
    const NAME: &'static str = "Spectrum Analyzer";
    const VENDOR: &'static str = "Apollo";
    const URL: &'static str = "https://github.com/apollo-daw";
    const EMAIL: &'static str = "jhoeflaken@live.nl";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: Default::default(),
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;
    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self, buffer:
        &mut Buffer, aux:
        &mut AuxiliaryBuffers, context:
        &mut impl ProcessContext<Self>
    ) -> ProcessStatus {
        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}

}

impl Vst3Plugin for SpectrumAnalyzer {
    const VST3_CLASS_ID: [u8; 16] = *"0119bb1cd8414052899facf87581175d".bytes();
    const VST3_SUB_CATEGORY: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Analyzer,
    ];

}

apollo_export_vst3!(SpectrumAnalyzer);