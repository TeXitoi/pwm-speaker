#![no_std]

extern crate stm32f103xx_hal as hal;
extern crate cast;

pub mod pitch;
pub mod songs;

use hal::prelude::*;
use hal::delay::Delay;

type Pwm = hal::pwm::Pwm<hal::stm32f103xx::TIM2, hal::pwm::C1>;
pub struct Speaker {
    pwm: Pwm,
    clk: u32,
}

impl Speaker {
    pub fn new(pwm: Pwm, clocks: hal::rcc::Clocks) -> Speaker {
        Speaker {
            pwm,
            clk: clocks.pclk1().0,
        }
    }
    pub fn play(&mut self, freq: impl Into<hal::time::Hertz>) {
        use cast::{u16, u32};

        let tim: hal::stm32f103xx::TIM2 = unsafe { core::mem::uninitialized() };
        let freq = freq.into().0;
        let ticks = self.clk / freq;
        let psc = u16(ticks / (1 << 16)).unwrap();
        tim.psc.write(|w| w.psc().bits(psc));
        let arr = u16(ticks / u32(psc + 1)).unwrap();
        tim.arr.write(|w| w.arr().bits(arr));

        let max = self.pwm.get_max_duty();
        self.pwm.set_duty(max / 2);
    }
    pub fn mute(&mut self) {
        self.pwm.set_duty(0);
    }
    pub fn play_score(&mut self, score: &[(u16, u8, u8, u8)], tempo: u16, delay: &mut Delay) {
        use cast::u32;
        let whole_dur = 60 * 1000 / u32(tempo);// in ms
        for &(pitch, n, d, pct) in score {
            let note_dur = whole_dur * u32(n) / u32(d);
            self.play(u32(pitch).hz());
            delay.delay_ms(note_dur * u32(pct) / 100);
            self.mute();
            delay.delay_ms(note_dur * (100 - u32(pct)) / 100);
        }
    }
}
