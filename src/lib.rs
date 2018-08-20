#![no_std]

extern crate cast;
extern crate stm32f103xx_hal as hal;

pub mod pitch;
pub mod songs;

use hal::delay::Delay;
use hal::prelude::*;

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
    pub fn play_score(&mut self, score: &songs::Score, delay: &mut Delay) {
        use songs::Event::*;
        use cast::u32;
        for event in score.events() {
            match event {
                Note { pitch, ms } => {
                    self.play(u32(pitch).hz());
                    delay.delay_ms(ms);
                }
                Rest { ms } => {
                    self.mute();
                    delay.delay_ms(ms);
                }
            }
        }
        self.mute();
    }
}
