#[cfg(test)]
mod tests {
    use nih_plug::buffer::Buffer;
    use spectrum_analyzer::analyzer::Analyzer;

    #[test]
    fn analyzer_creates_with_default_sample_rate() {
        let analyzer = Analyzer::new(44100.0);
        assert_eq!(analyzer.sample_rate(), 44100.0);
    }

    #[test]
    fn sample_rate_can_be_changed() {
        let mut analyzer = Analyzer::new(44100.0);
        analyzer.set_sample_rate(48000.0);
        assert_eq!(analyzer.sample_rate(), 48000.0);
    }

    #[test]
    fn process_returns_empty_for_empty_buffer() {
        let mut analyzer = Analyzer::new(44100.0);
        let mut buffer = Buffer::default();
        let results = analyzer.process(&mut buffer);
        assert!(results.is_empty());
    }

    #[test]
    fn process_returns_results_for_single_channel() {
        // Arrange
        let mut analyzer = Analyzer::new(44100.0);
        let mut channel1_data = vec![0.0; 1024];
        let mut buffer = Buffer::default();

        unsafe {
            buffer.set_slices(1024, |output_slices| {
                *output_slices = vec![&mut channel1_data]
            });
        }

        // Act
        let results = analyzer.process(&mut buffer);

        // Assert
        assert_eq!(results.len(), 1);
        assert!(!results[0].magnitudes.is_empty());
        assert!(!results[0].frequencies.is_empty());
    }

    #[test]
    fn process_returns_correct_number_of_results_for_multiple_channels() {
        // Arrange
        let mut analyzer = Analyzer::new(44100.0);
        let mut channel1_data = vec![0.0; 1024];
        let mut channel2_data = vec![0.0; 1024];
        let mut buffer = Buffer::default();
        unsafe {
            buffer.set_slices(1024, |output_slices| {
                *output_slices = vec![&mut channel1_data, &mut channel2_data]
            });
        }

        // Act
        let results = analyzer.process(&mut buffer);

        // Assert
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn magnitudes_and_frequencies_have_correct_length() {
        // Arrange
        let mut analyzer = Analyzer::new(44100.0);
        let mut channel1_data = vec![0.0; 1024];
        let mut buffer = Buffer::default();
        unsafe {
            buffer.set_slices(1024, |output_slices| {
                *output_slices = vec![&mut channel1_data]
            });
        }

        // Act
        let results = analyzer.process(&mut buffer);

        // Assert
        let result = &results[0];
        assert_eq!(result.magnitudes.len(), 512); // FFT size / 2
        assert_eq!(result.frequencies.len(), 512); // FFT size / 2
    }

    #[test]
    fn frequencies_are_calculated_correctly() {
        // Arrange
        let mut analyzer = Analyzer::new(44100.0);
        let mut channel1_data = vec![1.0; 1024];
        let mut buffer = Buffer::default();
        unsafe {
            buffer.set_slices(1024, |output_slices| {
                *output_slices = vec![&mut channel1_data]
            });
        }

        // Act
        let results = analyzer.process(&mut buffer);

        // Assert
        let result = &results[0];
        let expected_frequency_step = 44100.0 / 1024.0;
        assert_eq!(result.frequencies[1] - result.frequencies[0], expected_frequency_step);
    }
}