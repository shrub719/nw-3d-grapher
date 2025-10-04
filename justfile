current_target := "x86_64-unknown-linux-gnu" # TODO: get target

obj:
    cd obj && python3 main.py mesh.obj

build:
    just obj
    cargo build --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf

dev:
    just obj
    cargo build --bin nw_3d_grapher --target=thumbv7em-none-eabihf

load:
    just obj
    cargo run --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf -- -d obj/mesh.pbj

sim:
    cargo build --release --lib --target={{current_target}}

[macos]
run_nwb:
    ./epsilon_simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/release/lib_nw_3d_grapher_sim.dylib

[linux]
run_nwb:
    ./epsilon_simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/{{current_target}}/release/libnw_3d_grapher_sim.so

clean:
    cargo clean
