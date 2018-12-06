#![no_std]

extern crate cast;
extern crate stm32f103xx_hal as hal;

pub mod pitch;
pub mod songs;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;

type Pwm = hal::pwm::Pwm<hal::device::TIM2, hal::pwm::C1>;
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
    pub fn play(&mut self, pitch: u16) {
        use cast::{u16, u32};

        let tim: hal::device::TIM2 = unsafe { core::mem::uninitialized() };
        let freq = pitch as u32;
        let ticks = self.clk / freq;
        let psc = u16(ticks / (1 << 16)).unwrap();
        tim.psc.write(|w| w.psc().bits(psc));
        let arr = u16(ticks / u32(psc + 1)).unwrap();
        tim.arr.write(|w| w.arr().bits(arr));

        let max = self.pwm.get_max_duty();
        self.pwm.set_duty(max / 2);
    }
    pub fn rest(&mut self) {
        self.pwm.set_duty(0);
    }
    pub fn mute(&mut self) {
        self.pwm.disable();
    }
    pub fn unmute(&mut self) {
        self.pwm.enable();
    }
    pub fn play_score(&mut self, score: &songs::Score, delay: &mut Delay) {
        self.rest();
        self.unmute();
        for event in score.events() {
            match event {
                songs::Event::Note { pitch, ms } => {
                    self.play(pitch);
                    delay.delay_ms(ms);
                }
                songs::Event::Rest { ms } => {
                    self.rest();
                    delay.delay_ms(ms);
                }
            }
        }
        self.rest();
        self.mute();
    }
}
