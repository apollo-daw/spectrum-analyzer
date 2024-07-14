use std::marker::PhantomData;

pub struct SamplesIterator<'slice, 'sample: 'slice> {
    /// The raw output buffers.
    pub(super) buffers: *mut [&'sample mut [f32]],

    pub(super) current_sample: usize,

    /// The index of the last sample to iterate over plus one. Would be the same as the length of
    /// the buffers (`buffers.len()`) when iterating an entire buffer.
    pub(super) samples_end: usize,

    pub(super) _marker: PhantomData<&'slice mut [&'sample mut [f32]]>,
}

pub struct ChannelSamples<'slice, 'sample: 'slice> {
    pub(self) buffers: *mut [&'sample mut [f32]],
    pub(self) current_sample: usize,
    pub(self) _marker: PhantomData<&'slice mut [&'sample mut [f32]]>,
}

pub struct ChannelSamplerIterator<'slice, 'sample: 'slice> {
    pub(self) buffers: *mut [&'sample mut [f32]],
    pub(self) current_sample: usize,
    pub(self) current_channel: usize,
    pub(self) _marker: PhantomData<&'slice mut [&'sample mut [f32]]>,
}

impl<'slice, 'sample> Iterator for SamplesIterator<'slice, 'sample> {
    type Item = ChannelSamples<'slice, 'sample>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sample < self.samples_end {
            let channel_samples = ChannelSamples {
                buffers: self.buffers,
                current_sample: self.current_sample,
                _marker: PhantomData,
            };
            self.current_sample += 1;
            Some(channel_samples)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.samples_end - self.current_sample;
        (remaining, Some(remaining))
    }
}

impl<'slice, 'sample> IntoIterator for ChannelSamples<'slice, 'sample> {
    type Item = &'sample mut f32;
    type IntoIter = ChannelSamplerIterator<'slice, 'sample>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ChannelSamplerIterator {
            buffers: self.buffers,
            current_sample: self.current_sample,
            current_channel: 0,
            _marker: PhantomData,
        }
    }
}
