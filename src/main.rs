use anyhow::Result;
use embedded_hal::blocking::delay::DelayMs;
use esp_idf_hal::delay;
use esp_idf_sys as _;

mod led_strip;

const NUM_LEDS: usize = 8;
fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    let mut leds = led_strip::LedStrip::<NUM_LEDS>::new(
        esp_idf_sys::rmt_channel_t_RMT_CHANNEL_0,
        esp_idf_sys::gpio_num_t_GPIO_NUM_21,
    )
    .unwrap();

    let mut ets = delay::Ets;

    leds.brightness = 10.0;
    let on_color = led_strip::Color::new(255, 255, 255);
    let off_color = led_strip::Color::new(0, 0, 0);

    let mut cur_led = 0;
    loop {
        leds.colors[cur_led] = on_color;

        if cur_led == 0 {
            leds.colors[NUM_LEDS - 1] = off_color;
        } else {
            leds.colors[cur_led - 1] = off_color;
        }

        cur_led = (cur_led + 1) % NUM_LEDS;

        leds.update()?;
        ets.delay_ms(100u32);
    }
}
