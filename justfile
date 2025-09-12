current_target := "x86_64-unknown-linux-gnu" # TODO: get target

build:
    cargo build --release --bin NW3DGrapher --target=thumbv7em-none-eabihf

send:
    cargo run --release --bin NW3DGrapher --target=thumbv7em-none-eabihf

build_sim:
    cargo build --release --lib --target={{current_target}}

sim:
    just build_sim
    just run_nwb

[macos]
run_nwb:
    ./epsilon_simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/release/lib_nw_3d_grapher_sim.dylib

[linux]
run_nwb:
    ./epsilon_simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/{{current_target}}/release/libnw_3d_grapher_sim.so

clean:
    cargo clean
