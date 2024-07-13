
pub use self::sysex::*;

mod sysex;

pub enum MidiConfig {
    /// The plugin does not support MIDI.
    None,

    /// The plugin supports MIDI on/off/choke messages, pressure, and potentially a couple of
    /// other standard MIDI messages depending on the plugin standard and host.
    Basic,

    /// The plugin supports full MIDI continuous controller messages as well as pitch bend
    /// information. For VST3 plugins, this involves adding 130*16 parameters to the plugin.
    /// to bind to the 128 MIDI CCs, pitch bend, and channel pressure.
    MidiCCs
}