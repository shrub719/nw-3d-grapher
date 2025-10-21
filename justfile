current_target := "x86_64-unknown-linux-gnu" # TODO: get target

# ===== DEVICE =====

# creates .pbj in target/obj from .obj source file
# input_file contains .obj file location (e.g. obj/meshes/dog.obj)
# obj_name contains .pbj file name (e.g. dog)
obj input_file obj_name:
    mkdir -p target/obj
    python3 obj/main.py {{input_file}} {{obj_name}}

# automatically creates .pbj from obj/meshes
# obj_name contains .obj and .obj file name (e.g. dog)
dev-obj obj_name="":
    just obj obj/meshes/{{obj_name}}.obj {{obj_name}}
    cp target/obj/{{obj_name}}.pbj target/thumbv7em-none-eabihf/debug

# builds release profile
# obj_toggle toggles whether it will need external data
build obj_toggle="":
    cargo build --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf {{ if obj_toggle == "" { "" } else { "--features obj" } }}

# builds dev profile
# obj_toggle toggles whether it will need external data
dev obj_toggle="":
    cargo build --bin nw_3d_grapher --target=thumbv7em-none-eabihf {{ if obj_toggle == "" { "" } else { "--features obj" } }}

# loads app to calculator
# obj toggles whether it is loaded with external data, containing object name (e.g. dog) if it is
load obj_name="":
    cargo run --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf {{ if obj_name == "" { "" } else { "--features obj -- -d target/obj/" + obj_name + ".pbj" } }}

# automatically creates .pbj from obj/meshes before loading to calculator
# obj_name toggles whether it is loaded with external data, containing object name (e.g. dog) if it is
dev-load obj_name="":
    {{ if obj_name == "" { "" } else { "just dev-obj " + obj_name } }}
    cargo run --bin nw_3d_grapher --target=thumbv7em-none-eabihf {{ if obj_name == "" { "" } else { "--features obj -- -d target/obj/" + obj_name + ".pbj" } }}


# ===== SIMULATOR =====

# builds release profile for simulator
nwb-build:
    cargo build --release --lib --target={{current_target}}

# builds dev profile for simulator
nwb-dev:
    cargo build --lib --target={{current_target}}

# builds epsilon simulator
build-sim jobs="1":
    -git clone https://github.com/numworks/epsilon epsilon_simulator -b version-20
    cd epsilon_simulator && make PLATFORM=simulator -j {{jobs}}

# run app on simulator
[macos]
nwb-run:
    ./epsilon_simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/release/libnw_3d_grapher_sim.dylib
[linux]
nwb-run:
    ./epsilon_simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/{{current_target}}/release/libnw_3d_grapher_sim.so

# run dev profile on simulator
[macos]
nwb-dev-run:
    ./epsilon_simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/debug/libnw_3d_grapher_sim.dylib
[linux]
nwb-dev-run:
    ./epsilon_simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/{{current_target}}/debug/libnw_3d_grapher_sim.so


# ===== UTILS =====

ndev: nwb-dev nwb-dev-run

clean:
    cargo clean

clean-sim:
    cd ./epsilon_simulator && make clean

clean-all: clean clean-sim
