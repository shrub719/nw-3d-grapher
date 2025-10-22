import sys, re

def info(string):
    print("    " + string)


INPUTS = """
constexpr static KeySDLKeyPair sKeyPairs[] = {
  KeySDLKeyPair(Key::Down,      SDL_SCANCODE_DOWN),
  KeySDLKeyPair(Key::Down,      SDL_SCANCODE_S),
  KeySDLKeyPair(Key::Up,        SDL_SCANCODE_UP),
  KeySDLKeyPair(Key::Up,        SDL_SCANCODE_W),
  KeySDLKeyPair(Key::Left,      SDL_SCANCODE_LEFT),
  KeySDLKeyPair(Key::Left,      SDL_SCANCODE_A),
  KeySDLKeyPair(Key::Right,     SDL_SCANCODE_RIGHT),
  KeySDLKeyPair(Key::Right,     SDL_SCANCODE_D),

  KeySDLKeyPair(Key::Shift,     SDL_SCANCODE_LSHIFT),
  KeySDLKeyPair(Key::Shift,     SDL_SCANCODE_RSHIFT),
  KeySDLKeyPair(Key::Shift,     SDL_SCANCODE_Q),
  KeySDLKeyPair(Key::Alpha,     SDL_SCANCODE_LCTRL),
  KeySDLKeyPair(Key::Alpha,     SDL_SCANCODE_RCTRL),
  KeySDLKeyPair(Key::Alpha,     SDL_SCANCODE_E),

  KeySDLKeyPair(Key::Plus,      SDL_SCANCODE_KP_PLUS),
  KeySDLKeyPair(Key::Plus,      SDL_SCANCODE_EQUALS),
  KeySDLKeyPair(Key::Plus,      SDL_SCANCODE_R),
  KeySDLKeyPair(Key::Minus,     SDL_SCANCODE_KP_MINUS),
  KeySDLKeyPair(Key::Minus,     SDL_SCANCODE_MINUS),
  KeySDLKeyPair(Key::Minus,     SDL_SCANCODE_F),
  KeySDLKeyPair(Key::Multiplication, SDL_SCANCODE_0),
  KeySDLKeyPair(Key::Multiplication, SDL_SCANCODE_X),

  KeySDLKeyPair(Key::OK,        SDL_SCANCODE_RETURN),
  KeySDLKeyPair(Key::Back,      SDL_SCANCODE_ESCAPE),

  KeySDLKeyPair(Key::Toolbox,   SDL_SCANCODE_TAB),
  KeySDLKeyPair(Key::Backspace, SDL_SCANCODE_BACKSPACE),

  KeySDLKeyPair(Key::Seven,     SDL_SCANCODE_7),
  KeySDLKeyPair(Key::One,     SDL_SCANCODE_1),
  KeySDLKeyPair(Key::Eight,     SDL_SCANCODE_8),
  KeySDLKeyPair(Key::Two,     SDL_SCANCODE_2),
  KeySDLKeyPair(Key::Nine,      SDL_SCANCODE_9),
  KeySDLKeyPair(Key::Three,      SDL_SCANCODE_3),
};
"""

EXP = "constexpr static KeySDLKeyPair sKeyPairs\\[] ?= ?\\{[\\S\\s]*?};"

sim_dir = sys.argv[1]
input_file = sim_dir + "/ion/src/simulator/shared/keyboard.cpp"
content = ""
remapped_content = ""

with open(input_file, "r") as f:
    content = f.read()

if INPUTS not in content:
    remapped_content = re.sub(EXP, INPUTS, content)

    with open(input_file, "w") as f:
        f.write(remapped_content)

    info("remapped inputs of " + sim_dir)
else:
    info(sim_dir + " inputs already remapped")
