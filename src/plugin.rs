use std::sync::Arc;
use crate::audio_setup::{AudioIOLayout, AuxiliaryBuffers, BufferConfig};
use crate::buffer::Buffer;
use crate::context::{AsyncExecutor, InitContext, ProcessContext};
use crate::editor::Editor;
use crate::midi::{MidiConfig, SysExMessage};
use crate::params::Params;
use crate::wrapper::PluginState;

pub mod vst3;

/// A function that can execute a plugin's [`BackgroundTask`][Plugin::BackgroundTask]s. A plugin
/// can dispatch these tasks from the `initialize()` function, the `process()` function, or the
/// GUI, so they can be deferred for later to avoid blocking realtime contexts.
pub type TaskExecutor<P> = Box<dyn Fn(<P as Plugin>::BackgroundTask) + Send>;

pub enum ProcessStatus {
    /// The plugin has encountered an error and should be deactivated.
    Error(&'static str),
    /// The plugin has finished processing the buffer. When the input is silent the host may
    /// suspend the plugin to save resources as it sees fit.
    Normal,
    /// The plugin has a (reverb) tail with a specific length in samples.
    Tail(u32),
    /// This plugin will continue to produce sound regardless of whether the input is silent,
    /// and should thus not be deactivated by the host. This is essentially the same as having an
    /// infinite tail.
    KeepAlive
}

/// A plugin trait that all plugins must implement.
///
/// The [`Default`] trait is implemented so that the plugin can be created with default values.
///
/// The [`Send`] trait is implemented so that the plugin can be sent between threads.
///
/// The `static` lifetime specifier  is used to indicate that the plugin will live for the entire duration of
/// the program.
pub trait Plugin: Default + Send + 'static {
    /// The name of the plugin.
    const NAME: &'static str;
    /// The vendor of the plugin.
    const VENDOR: &'static str;
    /// The URL to the website of the plugin.
    const URL: &'static str;
    /// The email address of the vendor.
    const EMAIL: &'static str;
    /// The version of the plugin. Should be Semver compatible, e.g. "1.2.0".
    const VERSION: &'static str;

    /// The plugin's supported audio IO layouts. The first layout will be used as the default
    /// layout if the host doesn't or can't select an alternative layouts. Because of that it's
    /// recommended to begin this slice with a stereo layout. For maximum compatibility with the
    /// different plugin formats this default layout should also include all the plugin's
    /// auxiliary input and output ports, if the plugin has any. If the slice is empty, then the
    /// plugin will not have any audio IO.
    ///
    /// Both [`AudioIOLayout`] and [`PortNames`][crate::prelude::PortNames] have `.const_default()`
    /// functions for compile-time equivalents to `Default::default()`:
    ///
    /// ```
    /// use std::num::NonZeroU32;
    /// use apollo::audio_setup::{AudioIOLayout, new_nonzero_u32};
    ///
    /// const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    ///     main_input_channels: NonZeroU32::new(2),
    ///     main_output_channels: NonZeroU32::new(2),
    ///
    ///     aux_input_ports: &[new_nonzero_u32(2)],
    ///
    ///     ..AudioIOLayout::const_default()
    /// }];
    /// ```
    ///
    /// ###### Note
    ///
    /// Some plugin hosts, like Ableton Live, don't support MIDI-only plugins and may refuse to load
    /// plugins with no main output or with zero main output channels.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout];

    /// The plugin's supported MIDI input configurations. If the plugin doesn't support MIDI, then
    /// use [`MidiConfig::None`]. If the plugin supports MIDI input, then define its configuration.
    const MIDI_INPUT: MidiConfig = MidiConfig::None;

    /// The plugin's supported MIDI output configurations. If the plugin doesn't support MIDI, then
    /// use [`MidiConfig::None`]. If the plugin supports MIDI output, then define its configuration.
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    /// If enabled, the audio processing cycle may be split up into multiple smaller chunks if
    /// parameter values change occur in the middle of the buffer. Depending on the host these
    /// blocks may be as small as a single sample. Bitwig Studio sends at most one parameter change
    /// every 64 samples for example.
    const SAMPLE_ACCURATE_AUTOMATION: bool = false;

    /// If this is set to true, then the plugin will report itself as having a hard realtime
    /// processing requirement when the host asks for it. Supported hosts will never ask the plugin
    /// to do offline processing.
    const HARD_REALTIME_ONLY: bool = false;

    type SysExMessage: SysExMessage;

    /// A type encoding the different background tasks this plugin wants to run, or `()` if it
    /// doesn't have any background tasks. This is usually set to an enum type. The task type should
    /// not contain any heap allocated data like [`Vec`]s and [`Box`]es. Tasks can be sent using the
    /// methods on the various [`*Context`][crate::context] objects.
    ///
    /// ###### Note
    ///
    /// Sadly it's not yet possible to default this and the `async_executor()` function to `()`.
    /// See https://github.com/rust-lang/rust/issues/29661.
    type BackgroundTask: Send;

    /// A function that executes the plugin's tasks. When implementing this you will likely want to
    /// pattern match on the task type, and then send any resulting data back over a channel or
    /// triple buffer. See [`BackgroundTask`][Self::BackgroundTask].
    ///
    /// Queried only once immediately after the plugin instance is created. This function takes
    /// `&mut self` to make it easier to move data into the closure.
    fn task_executor(&mut self) -> TaskExecutor<Self> {
        // This is a dummy task executor that does nothing. It's used when the plugin doesn't have
        // any background tasks.
        Box::new(|_| {})
    }

    /// The plugin's parameters. The host will update the parameter values before calling
    /// `process()`. These string parameter IDs parameters should never change as they are used to
    /// distinguish between parameters.
    ///
    /// Queried only once immediately after the plugin instance is created.
    fn params(&self) -> Arc<dyn Params>;

    #[allow(unused_variables)]
    fn editor(&mut self, async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        None
    }

    #[allow(unused_variables)]
    fn filter_state(state: &mut PluginState) {}

    #[allow(unused_variables)]
    fn initialize(&mut self,
                  audio_io_layout: AudioIOLayout,
                  buffer_config: &BufferConfig,
                  context: &mut impl InitContext<Self>) -> bool {
        true
    }

    fn reset(&mut self) {}

    fn process(&mut self,
               buffer: &mut Buffer,
               aux: &mut AuxiliaryBuffers,
               context: &mut impl ProcessContext<Self>) -> ProcessStatus;

    fn deactivate(&mut self) {}

}