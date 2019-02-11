#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate pwm_speaker;
extern crate stm32f1xx_hal as hal;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::rt::entry;

#[entry]
fn main() -> ! {
    let dp = hal::stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);
    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let mut pwm = dp
        .TIM2
        .pwm(c1, &mut afio.mapr, 440.hz(), clocks, &mut rcc.apb1);
    pwm.enable();
    let mut speaker = pwm_speaker::Speaker::new(pwm, clocks);
    loop {
        use pwm_speaker::songs::*;
        speaker.play_score(&AU_FEU_LES_POMPIERS, &mut delay);
        delay.delay_ms(1000u32);
        speaker.play_score(&BATEAU_SUR_LEAU, &mut delay);
        delay.delay_ms(1000u32);
        speaker.play_score(&FRERE_JACQUES, &mut delay);
        delay.delay_ms(1000u32);
        speaker.play_score(&IL_ETAIT_UN_PETIT_NAVIRE, &mut delay);
        delay.delay_ms(1000u32);
        speaker.play_score(&LAVENTURIER, &mut delay);
        delay.delay_ms(1000u32);
        speaker.play_score(&MARIO_THEME_INTRO, &mut delay);
        delay.delay_ms(1000u32);
        speaker.play_score(&SO_WHAT, &mut delay);
        delay.delay_ms(1000u32);
        speaker.play_score(&THIRD_KIND, &mut delay);
        delay.delay_ms(1000u32);
    }
}
