current_target := "x86_64-unknown-linux-gnu" # TODO: get target

# input_file contains .obj file location (e.g. obj/meshes/dog.obj)
# obj_name contains .pbj file name (e.g. dog)
obj input_file obj_name:
    mkdir -p target/obj
    python3 obj/main.py {{input_file}} {{obj_name}}

dev-obj obj_name="":
    just obj obj/meshes/{{obj_name}}.obj {{obj_name}}

# obj_toggle toggles whether it needs external data
build obj_toggle="":
    cargo build --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf {{ if obj_toggle == "" { "" } else { "--features obj" } }}

# obj_toggle toggles whether it needs external data
dev obj_toggle="":
    cargo build --bin nw_3d_grapher --target=thumbv7em-none-eabihf {{ if obj_toggle == "" { "" } else { "--features obj" } }}

# obj contains object name (e.g. dog)
load obj_name="":
    cargo run --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf {{ if obj_name == "" { "" } else { "--features obj -- -d target/obj/" + obj_name + ".pbj" } }}

dev-load obj_name="":
    {{ if obj_name == "" { "" } else { "just dev-obj " + obj_name } }}
    cargo run --bin nw_3d_grapher --target=thumbv7em-none-eabihf {{ if obj_name == "" { "" } else { "--features obj -- -d target/obj/" + obj_name + ".pbj" } }}


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
