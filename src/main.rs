//!
//! randomly generated pixels drop all the way to the "floor"
//! 


#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f1xx_hal as hal;

use cortex_m_rt::entry;

use embedded_graphics::{
    fonts::{Font6x8, Font12x16, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::{PrimitiveStyleBuilder,TextStyleBuilder},
    };

use hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
    delay::Delay,
};

use rand::prelude::*;
use ssd1306::{prelude::*, Builder as SSD1306Builder};

use core::fmt;
use core::fmt::Write;
use arrayvec::ArrayString;


#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();


    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    
    // delay provider
    let mut delay = Delay::new(cp.SYST, clocks);

    // display initiated in GraphicsMode
    let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x32).connect_i2c(i2c).into();
        
    disp.init().unwrap();



    // let text_style = TextStyleBuilder::new(Font12x16).text_color(BinaryColor::On).build();
    // let mut format_buf = ArrayString::<[u8; 9]>::new();
    // format_gen(&mut format_buf);
    // Text::new(format_buf.as_str(), Point::new(10, 0)).into_styled(text_style).draw(&mut disp);


    // generate the random pixels

    let mut rng = SmallRng::seed_from_u64(0x1337_0808_0909_0303);
        
    let mut matrix: [[i8; 128]; 32] = [[0i8;128];32];

    for c in 0..128 {
        for r in 0..32 {
            let mut random = rng.next_u32();
            random = random%2;
            matrix[r as usize][c as usize] = random as i8;
        }
    }

    // display them
    
    for col in 0..128 {
        for row in 0..32 {
            let pixelval = matrix[row as usize][col as usize];
            disp.set_pixel(col, row, pixelval as u8);
                }           
            }

    disp.flush().unwrap();

    
    delay.delay_ms(500_u16);


    let mut new_matrix: [[i8; 128]; 32] = [[0i8;128];32];

    new_matrix = gravity(matrix);
      

    for col in 0..128 {
        for row in 0..32 {
            let pixelval = new_matrix[row as usize][col as usize];
                disp.set_pixel(col, row, pixelval as u8);
                }           
            }


    disp.flush().unwrap();


    for _ in 0..32 {

        new_matrix = gravity(new_matrix);
        

        for col in 0..128 {
            for row in 0..32 {
                let pixelval = new_matrix[row as usize][col as usize];
                disp.set_pixel(col, row, pixelval as u8);
                }           
            }
    


        disp.flush().unwrap();

        }

        loop {}


    }




fn gravity(array: [[i8;128];32]) -> [[i8;128];32] {
    
    let mut new_array: [[i8;128];32] = [[0i8;128];32];
     
    for row in (0..32).rev() {
        for col in 0..128 {
                
            if row + 1 > 31 { // line beyond the vertical bounds of the display
                new_array[row][col] = array[row][col];
            }
                
            else if array[row][col] == 0 { // empty pixels don't drop
                continue
            }
                
            else {
                if array[row+1][col] == 0 {
                    new_array[row][col] = 0;
                    new_array[row+1][col] = array[row][col];
                }
                   

                /*
                else if (col > 0) && (array[row+1][col-1] == 0) {
                    new_array[row][col] = 0;
                    new_array[row+1][col-1] = array[row][col];
                    }
    
                else if ((col+1) < 128) && (array[row+1][col+1] == 0) {
                    new_array[row][col] = 0;
                    new_array[row+1][col+1] = array[row][col];
                     
                    }
                
                */

                else {
                    new_array[row][col] = array[row][col];
                    
                    }                  
                }
            }
        }
           
    return new_array;
           
}