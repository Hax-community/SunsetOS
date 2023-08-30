#![no_std]
#![no_main]

mod kernel;

use kernel::print::printk::output;

pub fn startup() {
   output();
}
