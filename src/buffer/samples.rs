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

pub struct ChannelSamplesIterator<'slice, 'sample: 'slice> {
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
    type IntoIter = ChannelSamplesIterator<'slice, 'sample>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ChannelSamplesIterator {
            buffers: self.buffers,
            current_sample: self.current_sample,
            current_channel: 0,
            _marker: PhantomData,
        }
    }
}

impl<'slice, 'sample> Iterator for ChannelSamplesIterator<'slice, 'sample> {
    type Item = &'sample mut f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_channel < unsafe { &*self.buffers }.len() {
            let sample = unsafe {
                (*self.buffers)
                    .get_unchecked_mut(self.current_channel)
                    .get_unchecked_mut(self.current_sample)
            };

            self.current_channel += 1;
            Some(sample)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = unsafe { &*self.buffers }.len() - self.current_channel;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for SamplesIterator<'_, '_> {}
impl ExactSizeIterator for ChannelSamplesIterator<'_, '_> {}

impl<'slice, 'sample> ChannelSamples<'slice, 'sample> {

}