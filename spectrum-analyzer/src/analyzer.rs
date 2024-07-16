use nih_plug::buffer::Buffer;
use rustfft::FftPlanner;

/// Implements a Spectrum Analyzer.
pub struct Analyzer {
    fft_planner: FftPlanner<f32>,
    sample_rate: f32,
}

pub struct AnalyzerResult {
    pub frequencies: Vec<f32>,
    pub magnitudes: Vec<f32>,
}

impl Analyzer {
    /// Create a new instance of [`Analyzer`] with defaults.
    pub fn new(sample_rate: f32) -> Self {
        Analyzer {
            fft_planner: FftPlanner::new(),
            sample_rate,
        }
    }

    /// Get the sample rate for the analyzer to use.
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    /// Set the sample rate for the analyzer to use.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    /// Process the buffer and analyze the spectrum.
    pub fn process(&mut self, buffer: &mut Buffer) -> Vec<AnalyzerResult> {
        let sample_count = buffer.samples();
        let fft = self.fft_planner.plan_fft_forward(sample_count);
        let mut results = Vec::new();

        for channel_samples in buffer.as_slice() {
            // We don't want to change the original samples, so we make a copy of them, because we
            // need to convert them to complex numbers and [`fft.process()`] will modify the samples
            // in place.
            let mut complex_samples = channel_samples.into_iter()
                .map(|&mut sample| rustfft::num_complex::Complex::new(sample, 0.0))
                .collect::<Vec<_>>();

            fft.process(&mut complex_samples[..]);
            let fft_size = complex_samples.len();

            let mut magnitudes = Vec::with_capacity(fft_size / 2);
            for i in 0..fft_size / 2 {
                let bin = complex_samples[i];
                let magnitude = (bin.re.powi(2) + bin.im.powi(2)).sqrt();
                magnitudes.push(magnitude);
            }

            let frequencies = (0..fft_size / 2)
                .map(|i| i as f32 * self.sample_rate / fft_size as f32)
                .collect::<Vec<_>>();

            results.push(AnalyzerResult { magnitudes, frequencies });
        }

        results
    }
}