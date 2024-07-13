use std::num::NonZeroU32;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortNames {
    /// The name of the audio IO layout as a whole. Useful when a plugin has multiple layouts.
    /// Will be generated automatically if not provided.
    pub layout: Option<&'static str>,

    /// The name of the main input port. Will be generated automatically if not provided.
    pub main_input: Option<&'static str>,

    /// The name of the main output port. Will be generated automatically if not provided.
    pub main_output: Option<&'static str>,

    /// The names of the auxiliary side-chain input ports. Will be generated automatically if
    /// not provided or does not match the number of ports.
    pub aux_inputs: &'static [&'static str],

    /// The names of the auxiliary side-chain output ports. Will be generated automatically if
    /// not provided or does not match the number of ports.
    pub aux_outputs: &'static [&'static str]
}

/// Create a new non-zero u32 value. If the value is zero, then panic. This is useful for creating
/// non-zero values in a const context, because of the current limitations of using [`Option`] in
/// const functions. See <https://github.com/rust-lang/rust/issues/67441>,
pub const fn new_nonzero_u32(value: u32) -> NonZeroU32 {
    match NonZeroU32::new(value) {
        Some(nonzero) => nonzero,
        None => panic!("Expected a non-zero value, but got 0.")
    }
}

pub struct AuxiliaryBuffers<'a> {
    /// Buffers for all auxiliary side-chain inputs defined for this plugin. The data in these
    /// buffers can be safely overwritten. Auxiliary side-chain inputs can be defined using the
    /// [AudioIOLayout::aux_input_ports] field.
    pub inputs: &'a mut [Buffer<'a>],

    /// Buffers for all auxiliary side-chain inputs defined for this plugin. The data in these
    /// buffers can be safely overwritten. Auxiliary side-chain inputs can be defined using the
    /// [AudioIOLayout::aux_output_ports] field.
    pub outputs: &'a mut [Buffer<'a>],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessMode {
    /// Real-time processing. Audio is processed in real-time at a fixed rate.
    RealTime,

    /// Buffered processing. Audio is processed in real-time pace, but at irregular intervals. The
    /// host may do this to process audio ahead of time to loosen the real-time constraints and to
    /// reduce the risk of dropouts.
    Buffered,

    /// Offline processing. Audio is processed as fast as possible, without any real-time
    /// constraints. The host will continuously call the process method until all audio has been
    /// processed.
    Offline
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BufferConfig {
    /// The current sample rate of the audio. This is the number of samples per second.
    pub sample_rate: f32,

    /// The minimum buffer size the host will use. This may be set to `None` if the host does not
    /// have a minimum buffer size.
    pub min_buffer_size: Option<u32>,

    /// The maximum buffer size the host will use. The plugin should be able to accept variable
    /// buffer sizes within the range of the minimum and maximum buffer sizes.
    pub max_buffer_size: u32,

    /// The current processing mode of the plugin. This will determine how the plugin should
    /// process audio.
    pub process_mode: ProcessMode
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudioIOLayout {
    /// The number of main input channels for the plugin. This is the number of channels that the
    /// plugin will receive audio on. It can be set to `None` if the plugin does not have any input
    /// channels.
    pub main_input_channels: Option<NonZeroU32>,

    /// The number of main output channels for the plugin. This is the number of channels that the
    /// plugin will send audio to. It can be set to `None` if the plugin does not have any output
    /// channels.
    pub main_output_channels: Option<NonZeroU32>,

    /// The plugin's auxiliary side-chain inputs.
    pub aux_input_ports: &'static [NonZeroU32],

    /// The plugin's auxiliary side-chain outputs.
    pub aux_output_ports: &'static [NonZeroU32],

    /// Optional names for the audio input and/or output ports. If the names are not provided,
    /// then the default names will be used. The default names are based on the number of channels
    pub names: PortNames
}

impl AudioIOLayout {
    /// [`AudioIOLayout::default()`], but as a const function. Used when initializing
    /// `Plugin::AUDIO_IO_LAYOUTS`. This is because of issue
    /// <https://github.com/rust-lang/rust/issues/67792>
    pub const fn const_default() -> Self {
        Self {
            main_input_channels: None,
            main_output_channels: None,
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        }
    }

    /// Get the name of the layout. If the layout name is provided, then use that. If the layout
    /// name is not provided, then generate a name based on the number of channels and auxiliary
    /// side-chain ports.
    pub fn name(&self) -> String {
        // If the layout name is provided, then use that.
        if let Some(name) = self.names.layout {
            return name.to_owned();
        }

        // If the layout name is not provided, then generate a name based on the number of
        // channels and auxiliary side-chain ports.
        match (
            self.main_input_channels.map(NonZeroU32::get).unwrap_or_default(),
            self.main_output_channels.map(NonZeroU32::get).unwrap_or_default(),
            self.aux_input_ports.len(),
            self.aux_output_ports.len()
        ) {
            (0, 0, 0, 0) => String::from("Empty"),
            (_, 1, 0, _) | (1, 0, _, _) => String::from("Mono"),
            (_, 2, 0, _) | (2, 0, _, _) => String::from(" Stereo"),
            (_, 1, _, _) => String::from("Mono with Side-chain"),
            (_, 2, _, _) => String::from("Stereo with Side-chain"),
            (i, o, 0, 0) => format!("{i} inputs, {o} outputs"),
            (i, o, _, 0) => format!("{i} inputs, {o} outputs with Side-chain"),
            (i, o, 0, aux_out) => format!("{i} inputs, {o}*{} outputs", aux_out + 1),
            (i, o, aux_in, aux_out) => format!("{i}*{} inputs, {o}*{} outputs",
                                              aux_in + 1, aux_out + 1),
        }
    }

    /// Get the name of the main input port. If the main input port name is provided, then use
    /// that. Otherwise, return `Input` as the default name.
    pub fn main_input_name(&self) -> String {
        self.names.main_input.unwrap_or("Input").to_owned()
    }

    /// Get the name of the main output port. If the main output port name is provided, then use
    /// that. Otherwise, return `Output` as the default name.
    pub fn main_output_name(&self) -> String {
        self.names.main_output.unwrap_or("Output").to_owned()
    }

    /// Get the name of the auxiliary side-chain input port at the given index. If the name is
    /// provided, then use that. If the name is not provided, then generate a name based on the
    /// index.
    pub fn aux_input_name(&self, idx: usize) -> Option<String> {
        if idx >= self.aux_input_ports.len() {
            None
        } else {
            match self.names.aux_inputs.get(idx) {
                Some(name) => Some(String::from(*name)),
                None if self.aux_input_ports.len() == 1 => Some(String::from("Side-chain input")),
                None => Some(format!("Side-chain input {}", idx + 1))
            }
        }
    }

    /// Get the name of the auxiliary side-chain output port at the given index. If the name is
    /// provided, then use that. If the name is not provided, then generate a name based on the
    /// index.
    pub fn aux_output_name(&self, idx: usize) -> Option<String> {
        if idx >= self.aux_output_ports.len() {
            None
        } else {
            match self.names.aux_outputs.get(idx) {
                Some(name) => Some(String::from(*name)),
                None if self.aux_output_ports.len() == 1 => Some(String::from("Auxiliary output")),
                None => Some(format!("Auxiliary output {}", idx + 1))
            }
        }
    }

}

impl PortNames {
    /// [`PortNames::default()`], but as a const function. Used when initializing
    /// `Plugin::AUDIO_IO_LAYOUTS`. This is because of issue
    /// <https://github.com/rust-lang/rust/issues/67792>
    pub const fn const_default() -> Self {
        Self {
            layout: None,
            main_input: None,
            main_output: None,
            aux_inputs: &[],
            aux_outputs: &[],
        }
    }
}