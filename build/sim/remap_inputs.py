import sys, re, json

REMAP_EXP = r'constexpr static KeySDLKeyPair sKeyPairs\[] ?= ?\{[\S\s]*?};'
IDENTIFIER_EXP = r"pub const (\w+): Key ="
KEY_EXP = r": Key =\s+(Key::\w+);"


def info(string):
    print("    " + string)


def read_inputs(filename):
    with open(filename, "r") as f:
        return json.load(f)


def remap_key_pair(identifier, key, inputs):
    inputs[key] = inputs.pop(identifier)


# converts identifiers (e.g. D_DOWN) to their respective ion keys (e.g. Key::Down) according to constants.rs
def remap_controls(app_controls_file, inputs):
    controls = ""

    with open(app_controls_file, "r") as f:
        controls = f.read()
    
    for control in controls.split("\n"):
        identifier_match = re.findall(IDENTIFIER_EXP, control)
        key_match = re.findall(KEY_EXP, control)
        if identifier_match and key_match:
            remap_key_pair(identifier_match[0], key_match[0], inputs)


# converts inputs dict to C++ code
def convert_inputs(inputs):
    key_pairs = "constexpr static KeySDLKeyPair sKeyPairs[] = {\n"

    for ion_code, scancodes in inputs.items():
        for scancode in scancodes:
            spaces = (30 - len(ion_code)) * " "
            key_pair = f"  KeySDLKeyPair({ion_code},{spaces}SDL_SCANCODE_{scancode}),\n"
            key_pairs = key_pairs + key_pair
    key_pairs = key_pairs + "};"

    return key_pairs


def remap_file(sim_input_file, key_pairs):
    content = ""
    remapped_content = ""

    with open(sim_input_file, "r") as f:
        content = f.read()

    if key_pairs not in content:
        remapped_content = re.sub(REMAP_EXP, key_pairs, content)

        with open(sim_input_file, "w") as f:
            f.write(remapped_content)

        info("remapped inputs of " + sim_dir)
    else:
        info(sim_dir + " inputs already remapped")


sim_dir = sys.argv[1]
sim_input_file = sim_dir + "/ion/src/simulator/shared/keyboard.cpp"
app_controls_file = "src/constants.rs"

inputs = read_inputs("build/sim/inputs.json")

remap_controls(app_controls_file, inputs)

key_pairs = convert_inputs(inputs)

remap_file(sim_input_file, key_pairs)
