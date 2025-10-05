current_target := "x86_64-unknown-linux-gnu" # TODO: get target

obj file:
    mkdir -p target
    python3 obj/main.py {{file}}

build obj="":
    cargo build --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf {{ if obj == "" { "" } else { "--features obj" } }}

dev file="obj/empty.obj":
    just obj {{file}}
    cargo build --bin nw_3d_grapher --target=thumbv7em-none-eabihf --features obj

load obj="":
    cargo run --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf {{ if obj == "" { "" } else { "--features obj -- -d target/mesh.pbj" } }}

dev-load file="obj/empty.obj":
    just obj {{file}}
    cargo run --bin nw_3d_grapher --target=thumbv7em-none-eabihf --features obj -- -d target/mesh.pbj

# forget about sim for now
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
