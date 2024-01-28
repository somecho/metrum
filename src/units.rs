use crate::{error::ConversionError, scanner::Token, score::Tempo};

impl Tempo {
    /// Calculates the length of a whole note in this tempo in ms
    pub fn duration_of_whole(&self) -> f32 {
        let tempo_ratio = self.beat.0 as f32 / self.beat.1 as f32;
        let wholes_per_min = tempo_ratio * self.num_beats as f32;
        60.0 * 1000.0 / wholes_per_min
    }
}

impl Token {
    /// Converts a ratio to a duration. If token is not a ratio, returns a conversion error.
    pub fn as_duration_ms(&self, tempo: &Tempo, num_dots: u16) -> Result<f32, ConversionError> {
        match self {
            Token::Ratio(top, bottom) => {
                let duration_of_whole = tempo.duration_of_whole();
                let duration = duration_of_whole * (*top as f32 / *bottom as f32);
                let mut multiplier = 1.0;
                for i in 0..num_dots {
                    multiplier += 0.5 / (i as f32 + 1.0);
                }
                Ok(duration * multiplier)
            }
            _ => Err(ConversionError::NonRatioToDuration),
        }
    }
}

#[cfg(test)]
mod tests {
    mod tempo {
        use crate::score::Tempo;

        #[test]
        fn whole_duration() {
            let data = vec![
                (Tempo::new((1, 4), 120), 2000.0),
                (Tempo::new((1, 1), 60), 1000.0),
                (Tempo::new((1, 2), 60), 2000.0),
                (Tempo::new((1, 4), 60), 4000.0),
                (Tempo::new((1, 8), 60), 8000.0),
            ];
            for (tempo, duration) in data.iter() {
                assert_eq!(tempo.duration_of_whole(), *duration);
            }
        }
    }

    mod ratio {
        use crate::{scanner::Token, score::Tempo};

        #[test]
        fn duration() {
            let data = vec![
                (Tempo::new((1, 4), 120), Token::Ratio(1, 4), 0, 500.0),
                (Tempo::new((1, 4), 120), Token::Ratio(1, 4), 1, 750.0),
                (Tempo::new((1, 4), 60), Token::Ratio(1, 4), 0, 1000.0),
                (Tempo::new((1, 4), 60), Token::Ratio(1, 4), 2, 1750.0),
                (Tempo::new((1, 4), 240), Token::Ratio(1, 4), 0, 250.0),
                (Tempo::new((1, 4), 120), Token::Ratio(5, 4), 0, 2500.0),
                (Tempo::new((1, 4), 120), Token::Ratio(10, 8), 0, 2500.0),
            ];
            for (tempo, ratio, num_dots, duration) in data.iter() {
                assert_eq!(ratio.as_duration_ms(tempo, *num_dots).unwrap(), *duration);
            }
        }
    }
}
