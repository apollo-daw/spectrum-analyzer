use crate::audio_setup::AudioIOLayout;

mod vst3;

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
    /// # use nih_plug::prelude::*;
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
}