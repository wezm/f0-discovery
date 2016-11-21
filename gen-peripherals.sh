#!/bin/bash

set -e

main() {
    svd2rust -i STM32F0x0.svd dbgmcu > src/peripheral/dbgmcu.rs
    svd2rust -i STM32F0x0.svd gpioa > src/peripheral/gpio.rs
    gsed -i 's/\(pub struct Gpio\)a/\1/' src/peripheral/gpio.rs
    svd2rust -i STM32F0x0.svd i2c1 > src/peripheral/i2c.rs
    gsed -i 's/\(pub struct I2c\)1/\1/' src/peripheral/i2c.rs
    svd2rust -i STM32F0x0.svd rcc > src/peripheral/rcc.rs
    svd2rust -i STM32F0x0.svd spi1 > src/peripheral/spi.rs
    gsed -i 's/\(pub struct Spi\)1/\1/' src/peripheral/spi.rs
    svd2rust -i STM32F0x0.svd tim6 > src/peripheral/tim.rs
    gsed -i 's/\(pub struct Tim\)6/\1/' src/peripheral/tim.rs
    svd2rust -i STM32F0x0.svd usart1 > src/peripheral/usart.rs
    gsed -i 's/\(pub struct Usart\)1/\1/' src/peripheral/usart.rs

    set +e
    rustfmt src/peripheral/*.rs
    set -e

    xargo build --target thumbv7em-none-eabi
}

main

