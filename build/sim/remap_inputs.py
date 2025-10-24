import sys, re

REMAP_EXP = r'constexpr static KeySDLKeyPair sKeyPairs\[] ?= ?\{[\S\s]*?};'
IDENTIFIER_EXP = r"pub const (\w+): Key ="
KEY_EXP = r": Key =\s+(Key::\w+);"

inputs = [
    ["D_DOWN", "DOWN"],
    ["D_DOWN", "S"],
    ["D_UP", "UP"],
    ["D_UP", "W"],
    ["D_LEFT", "LEFT"],
    ["D_LEFT", "A"],
    ["D_RIGHT", "RIGHT"],
    ["D_RIGHT", "D"],
    ["D_SP_1", "LCTRL"],
    ["D_SP_1", "RCTRL"],
    ["D_SP_1", "E"],
    ["D_SP_2", "LSHIFT"],
    ["D_SP_2", "RSHIFT"],
    ["D_SP_2", "Q"],

    ["INCREASE", "KP_PLUS"],
    ["INCREASE", "EQUALS"],
    ["INCREASE", "R"],
    ["DECREASE", "KP_MINUS"],
    ["DECREASE", "MINUS"],
    ["DECREASE", "F"],
    ["MODIFIER", "0"],
    ["DECREASE", "X"],

    ["CONFIRM", "RETURN"],
    ["BACK", "ESCAPE"],

    ["HELP", "TAB"],
    ["RESET", "BACKSPACE"],

    ["MODE_1", "7"],
    ["MODE_1B", "1"],
    ["MODE_2", "8"],
    ["MODE_2B", "2"],
    ["MODE_3", "9"],
    ["MODE_3B", "3"]
]


def info(string):
    print("    " + string)


def remap_key_pair(identifier, key, inputs):
    for i in range(len(inputs)):
        if inputs[i][0] == identifier:
            inputs[i][0] = key


def remap_controls(app_controls_file, inputs):
    controls = ""

    with open(app_controls_file, "r") as f:
        controls = f.read()
    
    for control in controls.split("\n"):
        identifier_match = re.findall(IDENTIFIER_EXP, control)
        key_match = re.findall(KEY_EXP, control)
        if identifier_match and key_match:
            remap_key_pair(identifier_match[0], key_match[0], inputs)

    return inputs


def convert_inputs(remapped_inputs):
    key_pairs = "constexpr static KeySDLKeyPair sKeyPairs[] = {\n"

    for ion_code, scancode in inputs:
        key_pair = f"  KeySDLKeyPair({ion_code},    SDL_SCANCODE_{scancode}),\n"
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

remapped_inputs = remap_controls(app_controls_file, inputs)

key_pairs = convert_inputs(remapped_inputs)

remap_file(sim_input_file, key_pairs)
