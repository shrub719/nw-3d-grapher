current_target := `rustc -Vv | grep host | awk '{print $2}'`

# ===== DEVICE =====

# builds release profile
build:
    cargo build --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf

# builds dev profile
[default]
dev:
    cargo build --bin nw_3d_grapher --target=thumbv7em-none-eabihf

# loads app to calculator
load:
    cargo run --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf 

# loads dev profile to calculator
dev-load:
    cargo run --bin nw_3d_grapher --target=thumbv7em-none-eabihf


# ===== SIMULATOR =====

# builds release profile for simulator
nwb-build:
    cargo build --release --lib --target={{current_target}}

# builds dev profile for simulator
nwb-dev:
    cargo build --lib --target={{current_target}}

# runs dev profile on simulator
[macos]
run: nwb-dev
    ./sim/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/debug/libnw_3d_grapher_sim.dylib
[linux]
run: nwb-dev
    ./sim/epsilon.bin --nwb ./target/{{current_target}}/debug/libnw_3d_grapher_sim.so

   
 
# ===== SIMULATOR: LEGACY =====
# only kept these for build/BUILDING.md

# remaps sim inputs
# sim_dir is the directory containing epsilon
remap-sim sim_dir="epsilon_simulator":
    python3 build/sim/remap_inputs.py {{sim_dir}}

# sets up build environment for epsilon simulator
setup-sim:
    -git clone https://github.com/numworks/epsilon epsilon_simulator -b version-20
    cd epsilon_simulator && build/setup.sh --only-simulator
    just remap-sim

# builds epsilon simulator
# jobs is the number of jobs to use while building
build-sim jobs="8": remap-sim
    cd epsilon_simulator && make PLATFORM=simulator -j {{jobs}}

# run app on simulator
# sim_dir is the directory of the epsilon repo
[macos]
nwb-run sim_dir="epsilon_simulator": nwb-build
    ./{{sim_dir}}/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/release/libnw_3d_grapher_sim.dylib
[linux]
nwb-run sim_dir="epsilon_simulator": nwb-build
    ./{{sim_dir}}/output/release/simulator/linux/epsilon.bin --nwb ./target/{{current_target}}/release/libnw_3d_grapher_sim.so

# run dev profile on simulator
[macos]
nwb-dev-run: nwb-dev
    ./epsilon_simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/debug/libnw_3d_grapher_sim.dylib
[linux]
nwb-dev-run: nwb-dev
    ./epsilon_simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/{{current_target}}/debug/libnw_3d_grapher_sim.so

clean-sim:
    cd ./epsilon_simulator && make clean

clean-all: clean clean-sim

# ===== UTILS =====

clean:
    cargo clean

