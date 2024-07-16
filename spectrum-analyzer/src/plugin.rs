use std::sync::Arc;
use nih_plug::prelude::*;

/// The parameters of the plugin. This struct will be used to store the parameters of the plugin.
#[derive(Params)]
pub struct SpectrumAnalyzerParams {}

/// The plugin itself. This struct will be used to store the state of the plugin.
pub struct SpectrumAnalyzer {
    params: Arc<SpectrumAnalyzerParams>,
}

impl Default for SpectrumAnalyzerParams {
    /// Create a new instance of [`SpectrumAnalyzerParams`] with defaults.
    fn default() -> Self {
        SpectrumAnalyzerParams {}
    }
}

impl Default for SpectrumAnalyzer {
    /// Create a new instance of [`SpectrumAnalyzer`] with defaults.
    fn default() -> Self {
        SpectrumAnalyzer {
            params: Arc::new(SpectrumAnalyzerParams::default())
        }
    }
}

impl Plugin for SpectrumAnalyzer {
    const NAME: &'static str = "Apollo Spectrum Analyzer";
    const VENDOR: &'static str = "Apollo Digital Audio Workbench";
    const URL: &'static str = "https://github.com/apollo-daw";
    const EMAIL: &'static str = "jhoeflaken@live.nl";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    /// Get the parameters of the plugin. This will be a clone of the parameters that the plugin
    /// uses.
    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    /// Get the editor of the plugin. This is called when the host wants to open the editor of the
    /// plugin. If the plugin does not have an editor, it should return `None`.
    fn editor(
        &mut self,
        _async_executor: AsyncExecutor<Self>,
    ) -> Option<Box<dyn Editor>> {
        None
    }

    /// Initialize the plugin. This is called when the plugin is loaded. The plugin should return
    /// `true` if initialization was successful, and `false` otherwise.
    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    /// Process audio. This is called for each block of audio that the plugin processes.
    /// The plugin should return [`ProcessStatus::Normal`] if processing was successful, and
    /// [`ProcessStatus::Error`] if not. See [`ProcessStatus`] for other possible return values.
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        ProcessStatus::Normal
    }
}

// This is the UUID of the plugin. It is used to uniquely identify the plugin in the VST3 format.
// UUID IS f2a58f3c-ed54-47bd-90a6-220c13b9722a.
const PLUGIN_UUID: [u8; 16] = [
    0xf2, 0xa5, 0x8f, 0x3c, 0xed, 0x54, 0x47, 0xbd, 0x90, 0xa6, 0x22, 0x0c, 0x13, 0xb9, 0x72, 0x2a,
];

impl Vst3Plugin for SpectrumAnalyzer {
    const VST3_CLASS_ID: [u8; 16] = PLUGIN_UUID;
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Analyzer,
    ];
}

nih_export_vst3!(SpectrumAnalyzer);

