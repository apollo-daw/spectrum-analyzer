use nih_plug::prelude::*;
use spectrum_analyzer::plugin::SpectrumAnalyzer;

/// The main function for the plugin. This makes it possible to build the plugin as a standalone
/// executable.
fn main() {
    nih_export_standalone::<SpectrumAnalyzer>();
}