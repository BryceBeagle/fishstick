default:
    just --list

build PROFILE:
    cd software/fishstick-stm32f4 && cargo build --features {{PROFILE}}

run PROFILE:
    cd software/fishstick-stm32f4 && cargo run --features {{PROFILE}}

embed PROFILE:
    cd software/fishstick-stm32f4 && cargo embed {{PROFILE}} --features {{PROFILE}}
