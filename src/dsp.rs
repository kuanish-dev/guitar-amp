use std::f32::consts::{PI, TAU};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

pub struct AtomicF32(AtomicU32);

impl AtomicF32 {
    pub fn new(value: f32) -> Self {
        Self(AtomicU32::new(value.to_bits()))
    }

    pub fn get(&self) -> f32 {
        f32::from_bits(self.0.load(Ordering::Relaxed))
    }

    pub fn set(&self, value: f32) {
        self.0.store(value.to_bits(), Ordering::Relaxed);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Settings {
    pub gate_on: bool,
    pub gate: f32,
    pub comp_on: bool,
    pub comp: f32,
    pub wah_on: bool,
    pub wah: f32,
    pub gain: f32,
    pub drive_on: bool,
    pub drive: f32,
    pub bass: f32,
    pub mid: f32,
    pub tone: f32,
    pub phaser_on: bool,
    pub phaser_rate: f32,
    pub phaser_depth: f32,
    pub chorus_on: bool,
    pub chorus_rate: f32,
    pub chorus_depth: f32,
    pub tremolo_on: bool,
    pub tremolo_rate: f32,
    pub tremolo_depth: f32,
    pub delay_on: bool,
    pub delay_time: f32,
    pub delay_feedback: f32,
    pub delay_mix: f32,
    pub reverb_on: bool,
    pub reverb: f32,
    pub limiter_on: bool,
    pub limiter: f32,
    pub volume: f32,
}

impl Settings {
    pub fn clean() -> Self {
        Self {
            gate_on: true,
            gate: 0.1,
            comp_on: false,
            comp: 0.4,
            wah_on: false,
            wah: 0.5,
            gain: 0.3,
            drive_on: false,
            drive: 0.3,
            bass: 0.5,
            mid: 0.5,
            tone: 0.75,
            phaser_on: false,
            phaser_rate: 0.3,
            phaser_depth: 0.6,
            chorus_on: false,
            chorus_rate: 0.3,
            chorus_depth: 0.5,
            tremolo_on: false,
            tremolo_rate: 0.4,
            tremolo_depth: 0.6,
            delay_on: false,
            delay_time: 0.35,
            delay_feedback: 0.3,
            delay_mix: 0.25,
            reverb_on: true,
            reverb: 0.2,
            limiter_on: true,
            limiter: 0.8,
            volume: 0.5,
        }
    }

    pub fn presets() -> Vec<(&'static str, Settings)> {
        let clean = Self::clean();
        vec![
            ("Clean", clean),
            (
                "Jazz",
                Self {
                    comp_on: true,
                    comp: 0.3,
                    bass: 0.6,
                    mid: 0.55,
                    tone: 0.4,
                    reverb: 0.25,
                    ..clean
                },
            ),
            (
                "Funk",
                Self {
                    comp_on: true,
                    comp: 0.6,
                    wah_on: true,
                    wah: 0.6,
                    bass: 0.45,
                    tone: 0.85,
                    reverb: 0.1,
                    ..clean
                },
            ),
            (
                "Country",
                Self {
                    comp_on: true,
                    comp: 0.7,
                    bass: 0.4,
                    mid: 0.45,
                    tone: 0.9,
                    delay_on: true,
                    delay_time: 0.08,
                    delay_feedback: 0.0,
                    delay_mix: 0.5,
                    reverb: 0.15,
                    ..clean
                },
            ),
            (
                "80s Chorus",
                Self {
                    comp_on: true,
                    comp: 0.5,
                    tone: 0.8,
                    chorus_on: true,
                    chorus_rate: 0.3,
                    chorus_depth: 0.7,
                    delay_on: true,
                    delay_time: 0.4,
                    delay_feedback: 0.3,
                    delay_mix: 0.2,
                    reverb: 0.35,
                    ..clean
                },
            ),
            (
                "Surf",
                Self {
                    tone: 0.85,
                    tremolo_on: true,
                    tremolo_rate: 0.45,
                    tremolo_depth: 0.7,
                    reverb: 0.65,
                    ..clean
                },
            ),
            (
                "Ambient",
                Self {
                    bass: 0.45,
                    tone: 0.7,
                    chorus_on: true,
                    chorus_rate: 0.15,
                    chorus_depth: 0.5,
                    delay_on: true,
                    delay_time: 0.5,
                    delay_feedback: 0.55,
                    delay_mix: 0.45,
                    reverb: 0.6,
                    ..clean
                },
            ),
            (
                "Blues",
                Self {
                    gate: 0.15,
                    gain: 0.35,
                    drive_on: true,
                    drive: 0.22,
                    mid: 0.6,
                    tone: 0.65,
                    reverb: 0.25,
                    ..clean
                },
            ),
            (
                "Crunch",
                Self {
                    gate: 0.2,
                    gain: 0.4,
                    drive_on: true,
                    drive: 0.35,
                    tone: 0.65,
                    reverb: 0.15,
                    ..clean
                },
            ),
            (
                "Phaser Rock",
                Self {
                    gate: 0.25,
                    gain: 0.4,
                    drive_on: true,
                    drive: 0.45,
                    tone: 0.65,
                    phaser_on: true,
                    phaser_rate: 0.25,
                    phaser_depth: 0.7,
                    reverb: 0.15,
                    ..clean
                },
            ),
            (
                "Lead",
                Self {
                    gate: 0.35,
                    gain: 0.5,
                    drive_on: true,
                    drive: 0.8,
                    bass: 0.55,
                    mid: 0.65,
                    tone: 0.6,
                    delay_on: true,
                    delay_time: 0.4,
                    delay_feedback: 0.35,
                    delay_mix: 0.25,
                    volume: 0.45,
                    ..clean
                },
            ),
            (
                "Metal",
                Self {
                    gate: 0.5,
                    gain: 0.6,
                    drive_on: true,
                    drive: 1.0,
                    bass: 0.65,
                    mid: 0.3,
                    tone: 0.7,
                    reverb: 0.08,
                    volume: 0.4,
                    ..clean
                },
            ),
            (
                "Shoegaze",
                Self {
                    gate: 0.2,
                    gain: 0.5,
                    drive_on: true,
                    drive: 0.7,
                    tone: 0.55,
                    chorus_on: true,
                    chorus_rate: 0.2,
                    chorus_depth: 0.8,
                    tremolo_on: true,
                    tremolo_rate: 0.3,
                    tremolo_depth: 0.3,
                    delay_on: true,
                    delay_time: 0.45,
                    delay_feedback: 0.5,
                    delay_mix: 0.4,
                    reverb: 0.8,
                    volume: 0.4,
                    ..clean
                },
            ),
            (
                "Wah Solo",
                Self {
                    gate: 0.35,
                    wah_on: true,
                    wah: 0.7,
                    gain: 0.5,
                    drive_on: true,
                    drive: 0.65,
                    mid: 0.6,
                    tone: 0.65,
                    delay_on: true,
                    delay_time: 0.35,
                    delay_feedback: 0.3,
                    delay_mix: 0.2,
                    volume: 0.45,
                    ..clean
                },
            ),
        ]
    }
}

pub struct Params {
    settings: Mutex<Settings>,
    pub input_channel: AtomicUsize,
    pub input_channels: AtomicUsize,
    pub in_level: AtomicF32,
    pub out_level: AtomicF32,
    pub reduction: AtomicF32,
}

impl Params {
    pub fn new(s: &Settings) -> Self {
        Self {
            settings: Mutex::new(*s),
            input_channel: AtomicUsize::new(0),
            input_channels: AtomicUsize::new(1),
            in_level: AtomicF32::new(0.0),
            out_level: AtomicF32::new(0.0),
            reduction: AtomicF32::new(1.0),
        }
    }

    pub fn store(&self, s: &Settings) {
        if let Ok(mut guard) = self.settings.lock() {
            *guard = *s;
        }
    }

    pub fn try_load(&self) -> Option<Settings> {
        self.settings.try_lock().ok().map(|guard| *guard)
    }
}

#[derive(Default)]
struct OnePole {
    z: f32,
}

impl OnePole {
    fn low(&mut self, x: f32, a: f32) -> f32 {
        self.z += a * (x - self.z);
        self.z
    }

    fn high(&mut self, x: f32, a: f32) -> f32 {
        x - self.low(x, a)
    }
}

#[derive(Default, Clone, Copy)]
struct PhaseStage {
    x1: f32,
    y1: f32,
}

impl PhaseStage {
    fn process(&mut self, x: f32, a: f32) -> f32 {
        let y = a * x + self.x1 - a * self.y1;
        self.x1 = x;
        self.y1 = y;
        y
    }
}

struct Comb {
    buf: Vec<f32>,
    pos: usize,
    damp: f32,
}

impl Comb {
    fn new(len: usize) -> Self {
        Self {
            buf: vec![0.0; len.max(1)],
            pos: 0,
            damp: 0.0,
        }
    }

    fn process(&mut self, x: f32) -> f32 {
        let y = self.buf[self.pos];
        self.damp = y * 0.75 + self.damp * 0.25;
        self.buf[self.pos] = x + self.damp * 0.86;
        self.pos = (self.pos + 1) % self.buf.len();
        y
    }
}

struct Allpass {
    buf: Vec<f32>,
    pos: usize,
}

impl Allpass {
    fn new(len: usize) -> Self {
        Self {
            buf: vec![0.0; len.max(1)],
            pos: 0,
        }
    }

    fn process(&mut self, x: f32) -> f32 {
        let b = self.buf[self.pos];
        self.buf[self.pos] = x + b * 0.5;
        self.pos = (self.pos + 1) % self.buf.len();
        b - x
    }
}

pub struct Chain {
    sr: f32,
    gate_env: f32,
    gate_gain: f32,
    comp_env: f32,
    wah_env: f32,
    wah_low: f32,
    wah_band: f32,
    limit_env: f32,
    pub reduction: f32,
    pre_hp: OnePole,
    bass_lp: OnePole,
    mid_lp1: OnePole,
    mid_lp2: OnePole,
    tone_lp: OnePole,
    cab_lp1: OnePole,
    cab_lp2: OnePole,
    cab_hp: OnePole,
    phaser: [PhaseStage; 4],
    phaser_phase: f32,
    phaser_last: f32,
    chorus: Vec<f32>,
    chorus_pos: usize,
    chorus_phase: f32,
    tremolo_phase: f32,
    delay: Vec<f32>,
    delay_pos: usize,
    combs: Vec<Comb>,
    allpasses: Vec<Allpass>,
}

impl Chain {
    pub fn new(sample_rate: u32) -> Self {
        let sr = sample_rate as f32;
        let scale = sr / 44100.0;
        let combs = [1116, 1188, 1277, 1356, 1422, 1491, 1557, 1617]
            .iter()
            .map(|n| Comb::new((*n as f32 * scale) as usize))
            .collect();
        let allpasses = [556, 441, 341, 225]
            .iter()
            .map(|n| Allpass::new((*n as f32 * scale) as usize))
            .collect();
        Self {
            sr,
            gate_env: 0.0,
            gate_gain: 0.0,
            comp_env: 0.0,
            wah_env: 0.0,
            wah_low: 0.0,
            wah_band: 0.0,
            limit_env: 0.0,
            reduction: 1.0,
            pre_hp: OnePole::default(),
            bass_lp: OnePole::default(),
            mid_lp1: OnePole::default(),
            mid_lp2: OnePole::default(),
            tone_lp: OnePole::default(),
            cab_lp1: OnePole::default(),
            cab_lp2: OnePole::default(),
            cab_hp: OnePole::default(),
            phaser: [PhaseStage::default(); 4],
            phaser_phase: 0.0,
            phaser_last: 0.0,
            chorus: vec![0.0; (sr * 0.05) as usize + 4],
            chorus_pos: 0,
            chorus_phase: 0.0,
            tremolo_phase: 0.0,
            delay: vec![0.0; (sr * 1.2) as usize + 1],
            delay_pos: 0,
            combs,
            allpasses,
        }
    }

    fn coef(&self, hz: f32) -> f32 {
        1.0 - (-2.0 * PI * hz / self.sr).exp()
    }

    fn time_coef(&self, seconds: f32) -> f32 {
        1.0 - (-1.0 / (seconds * self.sr)).exp()
    }

    fn follow(&self, env: f32, level: f32, attack: f32, release: f32) -> f32 {
        let speed = if level > env {
            self.time_coef(attack)
        } else {
            self.time_coef(release)
        };
        env + (level - env) * speed
    }

    fn advance(&self, phase: f32, hz: f32) -> f32 {
        let next = phase + hz / self.sr;
        next - next.floor()
    }

    pub fn process(&mut self, input: f32, s: &Settings) -> f32 {
        let mut x = input;

        if s.gate_on {
            let threshold = 10f32.powf((-80.0 + s.gate * 50.0) / 20.0);
            let release = 1.0 - self.time_coef(0.05);
            self.gate_env = x.abs().max(self.gate_env * release);
            let (target, speed) = if self.gate_env > threshold {
                (1.0, self.time_coef(0.002))
            } else {
                (0.0, self.time_coef(0.08))
            };
            self.gate_gain += (target - self.gate_gain) * speed;
            x *= self.gate_gain;
        }

        if s.comp_on {
            self.comp_env = self.follow(self.comp_env, x.abs(), 0.005, 0.15);
            let threshold = -20.0 - s.comp * 25.0;
            let level = 20.0 * self.comp_env.max(1e-6).log10();
            let over = (level - threshold).max(0.0);
            let makeup = -threshold * 0.3;
            x *= 10f32.powf((makeup - over * 0.75) / 20.0);
        }

        if s.wah_on {
            self.wah_env = self.follow(self.wah_env, x.abs(), 0.01, 0.15);
            let sweep = (self.wah_env * (4.0 + s.wah * 40.0)).min(1.0);
            let f = 2.0 * (PI * (250.0 + sweep * 2300.0) / self.sr).sin();
            self.wah_low += f * self.wah_band;
            let high = x - self.wah_low - 0.25 * self.wah_band;
            self.wah_band += f * high;
            x = self.wah_band * 0.8 + x * 0.3;
        }

        x *= 10f32.powf(s.gain * 30.0 / 20.0);

        if s.drive_on {
            let a = self.coef(110.0);
            x = self.pre_hp.high(x, a);
            let k = 1.0 + s.drive * s.drive * 80.0;
            x = (k * x + 0.15 * (k * x).abs().min(1.0)).tanh() * 0.35;
            let lp = self.coef(4800.0);
            let hp = self.coef(75.0);
            x = self.cab_lp1.low(x, lp);
            x = self.cab_lp2.low(x, lp);
            x = self.cab_hp.high(x, hp);
        }

        let bass_a = self.coef(160.0);
        let lows = self.bass_lp.low(x, bass_a);
        let mid_a1 = self.coef(1400.0);
        let mid_a2 = self.coef(350.0);
        let mids = self.mid_lp1.low(x, mid_a1) - self.mid_lp2.low(x, mid_a2);
        x += lows * (s.bass * 2.0 - 1.0) * 1.5 + mids * (s.mid * 2.0 - 1.0) * 1.5;

        let tone_a = self.coef(500.0 * 2f32.powf(s.tone * 5.0)).min(1.0);
        x = self.tone_lp.low(x, tone_a);

        if s.phaser_on {
            self.phaser_phase = self.advance(self.phaser_phase, 0.1 + s.phaser_rate * 3.9);
            let lfo = 0.5 + 0.5 * (TAU * self.phaser_phase).sin();
            let t = (PI * 300.0 * 8f32.powf(lfo) / self.sr).tan();
            let a = (t - 1.0) / (t + 1.0);
            let mut y = x + self.phaser_last * 0.5 * s.phaser_depth;
            for stage in &mut self.phaser {
                y = stage.process(y, a);
            }
            self.phaser_last = y;
            x = x * (1.0 - 0.5 * s.phaser_depth) + y * 0.5 * s.phaser_depth;
        }

        if s.chorus_on {
            let len = self.chorus.len();
            self.chorus_phase = self.advance(self.chorus_phase, 0.2 + s.chorus_rate * 2.8);
            self.chorus[self.chorus_pos] = x;
            let lfo = (TAU * self.chorus_phase).sin();
            let delay = (0.014 + 0.006 * s.chorus_depth * lfo) * self.sr;
            let read = self.chorus_pos as f32 + len as f32 - delay;
            let i = read as usize;
            let frac = read - i as f32;
            let wet = self.chorus[i % len] * (1.0 - frac) + self.chorus[(i + 1) % len] * frac;
            self.chorus_pos = (self.chorus_pos + 1) % len;
            x = x * 0.75 + wet * 0.65;
        }

        if s.tremolo_on {
            self.tremolo_phase = self.advance(self.tremolo_phase, 1.0 + s.tremolo_rate * 11.0);
            let lfo = 0.5 + 0.5 * (TAU * self.tremolo_phase).sin();
            x *= 1.0 - s.tremolo_depth * lfo;
        }

        if s.delay_on {
            let len = self.delay.len();
            let samples = ((0.05 + s.delay_time * 0.95) * self.sr) as usize;
            let read = (self.delay_pos + len - samples.clamp(1, len - 1)) % len;
            let echo = self.delay[read];
            self.delay[self.delay_pos] = x + echo * s.delay_feedback * 0.85;
            self.delay_pos = (self.delay_pos + 1) % len;
            x += echo * s.delay_mix;
        }

        if s.reverb_on {
            let feed = x * 0.03;
            let mut wet = 0.0;
            for comb in &mut self.combs {
                wet += comb.process(feed);
            }
            for allpass in &mut self.allpasses {
                wet = allpass.process(wet);
            }
            x += wet * s.reverb * 1.5;
        }

        x *= s.volume * s.volume * 4.0;

        if s.limiter_on {
            let ceiling = 10f32.powf((s.limiter - 1.0) * 24.0 / 20.0);
            let release = 1.0 - self.time_coef(0.12);
            self.limit_env = x.abs().max(self.limit_env * release);
            let gain = if self.limit_env > ceiling {
                ceiling / self.limit_env
            } else {
                1.0
            };
            self.reduction = self.reduction.min(gain);
            x * gain
        } else {
            x.tanh()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &Settings, amplitude: f32) -> f32 {
        let mut chain = Chain::new(48000);
        let mut peak = 0.0f32;
        for i in 0..96000 {
            let x = amplitude * (2.0 * PI * 220.0 * i as f32 / 48000.0).sin();
            let y = chain.process(x, s);
            assert!(y.is_finite());
            if i > 24000 {
                peak = peak.max(y.abs());
            }
        }
        peak
    }

    fn average(s: &Settings, amplitude: f32) -> f32 {
        let mut chain = Chain::new(48000);
        let mut sum = 0.0f32;
        for i in 0..96000 {
            let x = amplitude * (2.0 * PI * 220.0 * i as f32 / 48000.0).sin();
            sum += chain.process(x, s).abs();
        }
        sum / 96000.0
    }

    #[test]
    fn limiter_holds_ceiling() {
        let (_, mut s) = Settings::presets()
            .into_iter()
            .find(|(name, _)| *name == "Lead")
            .unwrap();
        s.volume = 1.0;
        s.limiter = 0.5;
        let ceiling = 10f32.powf(-12.0 / 20.0);
        assert!(run(&s, 0.5) <= ceiling + 1e-4);
        s.limiter_on = false;
        assert!(run(&s, 0.5) > ceiling);
    }

    #[test]
    fn gate_mutes_noise() {
        let mut s = Settings::clean();
        s.reverb_on = false;
        s.gate = 0.5;
        assert!(run(&s, 0.0005) < 1e-6);
        assert!(run(&s, 0.1) > 0.01);
        s.gate_on = false;
        assert!(run(&s, 0.0005) > 1e-4);
    }

    #[test]
    fn presets_are_audible_and_bounded() {
        for (name, s) in Settings::presets() {
            for amplitude in [0.05, 0.2, 0.9] {
                let peak = run(&s, amplitude);
                assert!(peak > 0.02, "{name} {amplitude} {peak}");
                assert!(peak <= 1.0, "{name} {amplitude} {peak}");
            }
        }
    }

    #[test]
    fn each_effect_changes_the_sound() {
        let mut base = Settings::clean();
        base.reverb_on = false;
        base.limiter_on = false;
        let dry = average(&base, 0.1);
        let variants: [(&str, fn(&mut Settings)); 5] = [
            ("comp", |s| s.comp_on = true),
            ("wah", |s| s.wah_on = true),
            ("phaser", |s| s.phaser_on = true),
            ("chorus", |s| s.chorus_on = true),
            ("tremolo", |s| s.tremolo_on = true),
        ];
        for (name, apply) in variants {
            let mut s = base;
            apply(&mut s);
            let level = average(&s, 0.1);
            assert!((level - dry).abs() > dry * 0.02, "{name} {level} {dry}");
            assert!(level > dry * 0.2, "{name} {level} {dry}");
        }
    }
}
