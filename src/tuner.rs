const WINDOW: usize = 4096;
const NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

#[derive(Clone, Copy)]
pub struct Reading {
    pub note: &'static str,
    pub octave: i32,
    pub cents: f32,
    pub frequency: f32,
}

pub struct Tuner {
    buffer: Vec<f32>,
    diff: Vec<f32>,
    midi: Option<i32>,
    cents: f32,
    frequency: f32,
    silence: u32,
}

impl Tuner {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(WINDOW * 2),
            diff: Vec::new(),
            midi: None,
            cents: 0.0,
            frequency: 0.0,
            silence: 0,
        }
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.midi = None;
        self.silence = 0;
    }

    pub fn push(&mut self, samples: impl Iterator<Item = f32>) {
        self.buffer.extend(samples);
        if self.buffer.len() > WINDOW {
            let extra = self.buffer.len() - WINDOW;
            self.buffer.drain(..extra);
        }
    }

    pub fn update(&mut self, sample_rate: f32) -> Option<Reading> {
        match self.detect(sample_rate) {
            Some(frequency) => {
                self.silence = 0;
                let exact = 69.0 + 12.0 * (frequency / 440.0).log2();
                let midi = exact.round() as i32;
                let cents = (exact - midi as f32) * 100.0;
                if self.midi == Some(midi) {
                    self.cents += (cents - self.cents) * 0.3;
                    self.frequency += (frequency - self.frequency) * 0.3;
                } else {
                    self.midi = Some(midi);
                    self.cents = cents;
                    self.frequency = frequency;
                }
            }
            None => {
                self.silence += 1;
                if self.silence > 20 {
                    self.midi = None;
                }
            }
        }
        self.midi.map(|midi| Reading {
            note: NAMES[midi.rem_euclid(12) as usize],
            octave: midi.div_euclid(12) - 1,
            cents: self.cents,
            frequency: self.frequency,
        })
    }

    fn detect(&mut self, sample_rate: f32) -> Option<f32> {
        if self.buffer.len() < WINDOW {
            return None;
        }
        let x = &self.buffer;
        let rms = (x.iter().map(|s| s * s).sum::<f32>() / x.len() as f32).sqrt();
        if rms < 0.003 {
            return None;
        }

        let min_tau = (sample_rate / 1200.0) as usize;
        let max_tau = ((sample_rate / 60.0) as usize).min(WINDOW / 2);
        let span = WINDOW - max_tau;

        self.diff.clear();
        self.diff.push(1.0);
        let mut running = 0.0;
        for tau in 1..=max_tau {
            let mut sum = 0.0;
            for j in 0..span {
                let d = x[j] - x[j + tau];
                sum += d * d;
            }
            running += sum;
            self.diff.push(if running > 0.0 {
                sum * tau as f32 / running
            } else {
                1.0
            });
        }

        let d = &self.diff;
        let mut tau = min_tau.max(2);
        let mut found = None;
        while tau < max_tau {
            if d[tau] < 0.12 {
                while tau + 1 < max_tau && d[tau + 1] < d[tau] {
                    tau += 1;
                }
                found = Some(tau);
                break;
            }
            tau += 1;
        }
        let tau = found?;

        let (a, b, c) = (d[tau - 1], d[tau], d[tau + 1]);
        let denom = a - 2.0 * b + c;
        let shift = if denom.abs() > 1e-9 {
            0.5 * (a - c) / denom
        } else {
            0.0
        };
        Some(sample_rate / (tau as f32 + shift))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::TAU;

    fn read(frequency: f32, sample_rate: f32) -> Reading {
        let mut tuner = Tuner::new();
        tuner.push((0..WINDOW * 2).map(|i| {
            let t = i as f32 / sample_rate;
            0.3 * (TAU * frequency * t).sin() + 0.2 * (TAU * frequency * 2.0 * t).sin()
        }));
        tuner.update(sample_rate).unwrap()
    }

    #[test]
    fn open_strings() {
        let strings = [
            (82.41, "E", 2),
            (110.0, "A", 2),
            (146.83, "D", 3),
            (196.0, "G", 3),
            (246.94, "B", 3),
            (329.63, "E", 4),
        ];
        for rate in [44100.0, 48000.0] {
            for (frequency, note, octave) in strings {
                let r = read(frequency, rate);
                assert_eq!((r.note, r.octave), (note, octave));
                assert!(r.cents.abs() < 2.0, "{frequency} {}", r.cents);
            }
        }
    }

    #[test]
    fn flat_string() {
        let r = read(108.0, 48000.0);
        assert_eq!(r.note, "A");
        assert!((r.cents + 31.8).abs() < 2.0, "{}", r.cents);
    }

    #[test]
    fn silence() {
        let mut tuner = Tuner::new();
        tuner.push((0..WINDOW).map(|_| 0.0));
        assert!(tuner.update(48000.0).is_none());
    }
}
