#!/bin/sh

#########################################################################################
#                                                                                       #
# This script was auto-generated from a Justfile by just.sh.                            #
#                                                                                       #
# Generated on 2025-10-06 with just.sh version 0.0.2.                                   #
# https://github.com/jstrieb/just.sh                                                    #
#                                                                                       #
# Run `./just.sh --dump` to recover the original Justfile.                              #
#                                                                                       #
#########################################################################################

if sh "set -o pipefail" > /dev/null 2>&1; then
  set -euo pipefail
else
  set -eu
fi


#########################################################################################
# Internal functions                                                                    #
#########################################################################################

os() {
  case "$(uname -s | tr '[:upper:]' '[:lower:]')" in
  *darwin*)
    echo "macos"
    ;;
  *linux*)
    echo "linux"
    ;;
  *windows*|*msys*)
    echo "windows"
    ;;
  *)
    echo "unknown"
    ;;
  esac
}

if_cdcba2184ad3b021() {
  if [ "${VAR_obj_toggle}" = '' ]; then
    THEN_EXPR='' || exit "${?}"
    echo "${THEN_EXPR}"
  else
    ELSE_EXPR='--features obj' || exit "${?}"
    echo "${ELSE_EXPR}"
  fi
}

if_e2afe9da5e0d3dea() {
  if [ "${VAR_obj_name}" = '' ]; then
    THEN_EXPR='' || exit "${?}"
    echo "${THEN_EXPR}"
  else
    ELSE_EXPR='--features obj -- -d target/obj/'"${VAR_obj_name}"'.pbj' || exit "${?}"
    echo "${ELSE_EXPR}"
  fi
}

if_302d2c5cf448a341() {
  if [ "${VAR_obj_name}" = '' ]; then
    THEN_EXPR='' || exit "${?}"
    echo "${THEN_EXPR}"
  else
    ELSE_EXPR='./just.sh dev-obj '"${VAR_obj_name}" || exit "${?}"
    echo "${ELSE_EXPR}"
  fi
}


#########################################################################################
# Variables                                                                             #
#########################################################################################

# User-overwritable variables (via CLI)
INVOCATION_DIRECTORY="$(pwd)"
DEFAULT_SHELL='sh'
DEFAULT_SHELL_ARGS='-cu'
LIST_HEADING='Available recipes:
'
LIST_PREFIX='    '
CHOOSER='fzf'
SORTED='true'

# Display colors
SHOW_COLOR='false'
if [ -t 1 ]; then SHOW_COLOR='true'; fi
NOCOLOR="$(test "${SHOW_COLOR}" = 'true' && printf "\033[m" || echo)"
BOLD="$(test "${SHOW_COLOR}" = 'true' && printf "\033[1m" || echo)"
RED="$(test "${SHOW_COLOR}" = 'true' && printf "\033[1m\033[31m" || echo)"
YELLOW="$(test "${SHOW_COLOR}" = 'true' && printf "\033[33m" || echo)"
CYAN="$(test "${SHOW_COLOR}" = 'true' && printf "\033[36m" || echo)"
GREEN="$(test "${SHOW_COLOR}" = 'true' && printf "\033[32m" || echo)"
PINK="$(test "${SHOW_COLOR}" = 'true' && printf "\033[35m" || echo)"
BLUE="$(test "${SHOW_COLOR}" = 'true' && printf "\033[34m" || echo)"
TICK="$(printf '%s' '`')"
DOLLAR="$(printf '%s' '$')"

assign_variables() {
  test -z "${HAS_RUN_assign_variables:-}" || return 0

  VAR_current_target='x86_64-unknown-linux-gnu' || exit "${?}"

  HAS_RUN_assign_variables="true"
}


#########################################################################################
# Recipes                                                                               #
#########################################################################################

# creates .pbj in target/obj from .obj source file

# input_file contains .obj file location (e.g. obj/meshes/dog.obj)

# obj_name contains .pbj file name (e.g. dog)

FUN_obj() {
  # Recipe setup and pre-recipe dependencies
  test -z "${HAS_RUN_obj:-}" \
    || test "${FORCE_obj:-}" = "true" \
    || return 0

  if [ "${#}" -lt 2 ]; then
    (
      echo_error 'Recipe `obj`'" got ${#} arguments but takes 2"
      echo "${BOLD}usage:${NOCOLOR}"
      echo "    ${0} "'obj '"${CYAN}"'input_file'"${NOCOLOR}"' '"${CYAN}"'obj_name'"${NOCOLOR}"
    ) >&2
    exit 1
  fi
  VAR_input_file="${1:-}"
  VAR_obj_name="${2:-}"

  OLD_WD="$(pwd)"
  cd "${INVOCATION_DIRECTORY}"

  # Recipe body
  echo_recipe_line 'mkdir -p target/obj'
  env "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} \
    'mkdir -p target/obj'  \
    || recipe_error "obj" "${LINENO:-}"
  INTERP_1="${VAR_input_file}" || recipe_error 'obj' "${LINENO:-}"
  INTERP_2="${VAR_obj_name}" || recipe_error 'obj' "${LINENO:-}"
  echo_recipe_line 'python3 obj/main.py '"${INTERP_1}"' '"${INTERP_2}"
  env "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} \
    'python3 obj/main.py '"${INTERP_1}"' '"${INTERP_2}"  \
    || recipe_error "obj" "${LINENO:-}"

  # Post-recipe dependencies and teardown
  cd "${OLD_WD}"
  if [ -z "${FORCE_obj:-}" ]; then
    HAS_RUN_obj="true"
  fi
}

# automatically creates .pbj from obj/meshes

# obj_name contains .obj and .obj file name (e.g. dog)

FUN_dev_obj() {
  # Recipe setup and pre-recipe dependencies
  test -z "${HAS_RUN_dev_obj:-}" \
    || test "${FORCE_dev_obj:-}" = "true" \
    || return 0

  VAR_obj_name="${1:-}"
  if [ "${#}" -lt 1 ]; then
    VAR_obj_name=''
  fi

  OLD_WD="$(pwd)"
  cd "${INVOCATION_DIRECTORY}"

  # Recipe body
  INTERP_1="${VAR_obj_name}" || recipe_error 'dev-obj' "${LINENO:-}"
  INTERP_2="${VAR_obj_name}" || recipe_error 'dev-obj' "${LINENO:-}"
  echo_recipe_line "./$(basename "${0}")"' obj obj/meshes/'"${INTERP_1}"'.obj '"${INTERP_2}"
  env "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} \
    "./$(basename "${0}")"' obj obj/meshes/'"${INTERP_1}"'.obj '"${INTERP_2}"  \
    || recipe_error "dev-obj" "${LINENO:-}"
  INTERP_3="${VAR_obj_name}" || recipe_error 'dev-obj' "${LINENO:-}"
  echo_recipe_line 'cp target/obj/'"${INTERP_3}"'.pbj target/thumbv7em-none-eabihf/debug'
  env "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} \
    'cp target/obj/'"${INTERP_3}"'.pbj target/thumbv7em-none-eabihf/debug'  \
    || recipe_error "dev-obj" "${LINENO:-}"

  # Post-recipe dependencies and teardown
  cd "${OLD_WD}"
  if [ -z "${FORCE_dev_obj:-}" ]; then
    HAS_RUN_dev_obj="true"
  fi
}

# builds release profile

# obj_toggle toggles whether it will need external data

FUN_build() {
  # Recipe setup and pre-recipe dependencies
  test -z "${HAS_RUN_build:-}" \
    || test "${FORCE_build:-}" = "true" \
    || return 0

  VAR_obj_toggle="${1:-}"
  if [ "${#}" -lt 1 ]; then
    VAR_obj_toggle=''
  fi

  OLD_WD="$(pwd)"
  cd "${INVOCATION_DIRECTORY}"

  # Recipe body
  INTERP_1="$(if_cdcba2184ad3b021)" || recipe_error 'build' "${LINENO:-}"
  echo_recipe_line 'cargo build --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf '"${INTERP_1}"
  env "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} \
    'cargo build --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf '"${INTERP_1}"  \
    || recipe_error "build" "${LINENO:-}"

  # Post-recipe dependencies and teardown
  cd "${OLD_WD}"
  if [ -z "${FORCE_build:-}" ]; then
    HAS_RUN_build="true"
  fi
}

# builds dev profile

# obj_toggle toggles whether it will need external data

FUN_dev() {
  # Recipe setup and pre-recipe dependencies
  test -z "${HAS_RUN_dev:-}" \
    || test "${FORCE_dev:-}" = "true" \
    || return 0

  VAR_obj_toggle="${1:-}"
  if [ "${#}" -lt 1 ]; then
    VAR_obj_toggle=''
  fi

  OLD_WD="$(pwd)"
  cd "${INVOCATION_DIRECTORY}"

  # Recipe body
  INTERP_1="$(if_cdcba2184ad3b021)" || recipe_error 'dev' "${LINENO:-}"
  echo_recipe_line 'cargo build --bin nw_3d_grapher --target=thumbv7em-none-eabihf '"${INTERP_1}"
  env "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} \
    'cargo build --bin nw_3d_grapher --target=thumbv7em-none-eabihf '"${INTERP_1}"  \
    || recipe_error "dev" "${LINENO:-}"

  # Post-recipe dependencies and teardown
  cd "${OLD_WD}"
  if [ -z "${FORCE_dev:-}" ]; then
    HAS_RUN_dev="true"
  fi
}

#  loads app to calculator

# obj toggles whether it is loaded with external data, containing object name (e.g. dog) if it is

FUN_load() {
  # Recipe setup and pre-recipe dependencies
  test -z "${HAS_RUN_load:-}" \
    || test "${FORCE_load:-}" = "true" \
    || return 0

  VAR_obj_name="${1:-}"
  if [ "${#}" -lt 1 ]; then
    VAR_obj_name=''
  fi

  OLD_WD="$(pwd)"
  cd "${INVOCATION_DIRECTORY}"

  # Recipe body
  INTERP_1="$(if_e2afe9da5e0d3dea)" || recipe_error 'load' "${LINENO:-}"
  echo_recipe_line 'cargo run --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf '"${INTERP_1}"
  env "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} \
    'cargo run --release --bin nw_3d_grapher --target=thumbv7em-none-eabihf '"${INTERP_1}"  \
    || recipe_error "load" "${LINENO:-}"

  # Post-recipe dependencies and teardown
  cd "${OLD_WD}"
  if [ -z "${FORCE_load:-}" ]; then
    HAS_RUN_load="true"
  fi
}

# automatically creates .pbj from obj/meshes before loading to calculator

# obj_name toggles whether it is loaded with external data, containing object name (e.g. dog) if it is

FUN_dev_load() {
  # Recipe setup and pre-recipe dependencies
  test -z "${HAS_RUN_dev_load:-}" \
    || test "${FORCE_dev_load:-}" = "true" \
    || return 0

  VAR_obj_name="${1:-}"
  if [ "${#}" -lt 1 ]; then
    VAR_obj_name=''
  fi

  OLD_WD="$(pwd)"
  cd "${INVOCATION_DIRECTORY}"

  # Recipe body
  INTERP_1="$(if_302d2c5cf448a341)" || recipe_error 'dev-load' "${LINENO:-}"
  echo_recipe_line "${INTERP_1}"
  env "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} \
    "${INTERP_1}"  \
    || recipe_error "dev-load" "${LINENO:-}"
  INTERP_2="$(if_e2afe9da5e0d3dea)" || recipe_error 'dev-load' "${LINENO:-}"
  echo_recipe_line 'cargo run --bin nw_3d_grapher --target=thumbv7em-none-eabihf '"${INTERP_2}"
  env "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} \
    'cargo run --bin nw_3d_grapher --target=thumbv7em-none-eabihf '"${INTERP_2}"  \
    || recipe_error "dev-load" "${LINENO:-}"

  # Post-recipe dependencies and teardown
  cd "${OLD_WD}"
  if [ -z "${FORCE_dev_load:-}" ]; then
    HAS_RUN_dev_load="true"
  fi
}

# forget about sim for now

FUN_sim() {
  # Recipe setup and pre-recipe dependencies
  test -z "${HAS_RUN_sim:-}" \
    || test "${FORCE_sim:-}" = "true" \
    || return 0

  OLD_WD="$(pwd)"
  cd "${INVOCATION_DIRECTORY}"

  # Recipe body
  INTERP_1="${VAR_current_target}" || recipe_error 'sim' "${LINENO:-}"
  echo_recipe_line 'cargo build --release --lib --target='"${INTERP_1}"
  env "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} \
    'cargo build --release --lib --target='"${INTERP_1}"  \
    || recipe_error "sim" "${LINENO:-}"

  # Post-recipe dependencies and teardown
  cd "${OLD_WD}"
  if [ -z "${FORCE_sim:-}" ]; then
    HAS_RUN_sim="true"
  fi
}

FUN_run_nwb_macos() {
  # Recipe setup and pre-recipe dependencies
  test -z "${HAS_RUN_run_nwb:-}" \
    || test "${FORCE_run_nwb:-}" = "true" \
    || return 0

  OLD_WD="$(pwd)"
  cd "${INVOCATION_DIRECTORY}"

  # Recipe body
  INTERP_1="${VAR_current_target}" || recipe_error 'run_nwb' "${LINENO:-}"
  echo_recipe_line './epsilon_simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/'"${INTERP_1}"'/release/lib_nw_3d_grapher_sim.dylib'
  env "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} \
    './epsilon_simulator/output/release/simulator/macos/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/'"${INTERP_1}"'/release/lib_nw_3d_grapher_sim.dylib'  \
    || recipe_error "run_nwb" "${LINENO:-}"

  # Post-recipe dependencies and teardown
  cd "${OLD_WD}"
  if [ -z "${FORCE_run_nwb:-}" ]; then
    HAS_RUN_run_nwb="true"
  fi
}

FUN_run_nwb_linux() {
  # Recipe setup and pre-recipe dependencies
  test -z "${HAS_RUN_run_nwb:-}" \
    || test "${FORCE_run_nwb:-}" = "true" \
    || return 0

  OLD_WD="$(pwd)"
  cd "${INVOCATION_DIRECTORY}"

  # Recipe body
  INTERP_1="${VAR_current_target}" || recipe_error 'run_nwb' "${LINENO:-}"
  echo_recipe_line './epsilon_simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/'"${INTERP_1}"'/release/libnw_3d_grapher_sim.so'
  env "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} \
    './epsilon_simulator/output/release/simulator/linux/epsilon.bin --nwb ./target/'"${INTERP_1}"'/release/libnw_3d_grapher_sim.so'  \
    || recipe_error "run_nwb" "${LINENO:-}"

  # Post-recipe dependencies and teardown
  cd "${OLD_WD}"
  if [ -z "${FORCE_run_nwb:-}" ]; then
    HAS_RUN_run_nwb="true"
  fi
}

FUN_clean() {
  # Recipe setup and pre-recipe dependencies
  test -z "${HAS_RUN_clean:-}" \
    || test "${FORCE_clean:-}" = "true" \
    || return 0

  OLD_WD="$(pwd)"
  cd "${INVOCATION_DIRECTORY}"

  # Recipe body
  echo_recipe_line 'cargo clean'
  env "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} \
    'cargo clean'  \
    || recipe_error "clean" "${LINENO:-}"

  # Post-recipe dependencies and teardown
  cd "${OLD_WD}"
  if [ -z "${FORCE_clean:-}" ]; then
    HAS_RUN_clean="true"
  fi
}

FUN_run_nwb() {
  if [ "$(os)" = 'macos' ]; then
    FUN_run_nwb_macos
  elif [ "$(os)" = 'linux' ]; then
    FUN_run_nwb_linux
  else
    echo_error "Justfile does not contain recipe "'`run_nwb`.'
  fi
}


#########################################################################################
# Helper functions                                                                      #
#########################################################################################

# Sane, portable echo that doesn't escape characters like "\n" behind your back
echo() {
  if [ "${#}" -gt 0 ]; then
    printf "%s\n" "${@}"
  else
    printf "\n"
  fi
}

# realpath is a GNU coreutils extension
realpath() {
  # The methods to replicate it get increasingly error-prone
  # TODO: improve
  if type -P realpath > /dev/null 2>&1; then
    "$(type -P realpath)" "${1}"
  elif type python3 > /dev/null 2>&1; then
    python3 -c 'import os.path, sys; print(os.path.realpath(sys.argv[1]))' "${1}"
  elif type python > /dev/null 2>&1; then
    python -c 'import os.path, sys; print os.path.realpath(sys.argv[1])' "${1}"
  elif [ -f "${1}" ] && ! [ -z "$(dirname "${1}")" ]; then
    # We assume the directory exists. For our uses, it always does
    echo "$(
      cd "$(dirname "${1}")";
      pwd -P
    )/$(
      basename "${1}"
    )"
  elif [ -f "${1}" ]; then
    pwd -P
  elif [ -d "${1}" ]; then
  (
    cd "${1}"
    pwd -P
  )
  else
    echo "${1}"
  fi
}

echo_error() {
  echo "${RED}error${NOCOLOR}: ${BOLD}${1}${NOCOLOR}" >&2
}

recipe_error() {
  STATUS="${?}"
  if [ -z "${2:-}" ]; then
      echo_error "Recipe "'`'"${1}"'`'" failed with exit code ${STATUS}"
  else
      echo_error "Recipe "'`'"${1}"'`'" failed on line ${2} with exit code ${STATUS}"
  fi
  exit "${STATUS}"
}

echo_recipe_line() {
  echo "${BOLD}${1}${NOCOLOR}" >&2
}
            
set_var() {
  export "VAR_${1}=${2}"
}
            
summarizefn() {
  while [ "$#" -gt 0 ]; do
    case "${1}" in
    -u|--unsorted)
      SORTED="false"
      ;;
    esac
    shift
  done

  if [ "${SORTED}" = "true" ]; then
    printf "%s " build clean dev dev-load dev-obj load obj run_nwb sim
  else
    printf "%s " obj dev-obj build dev load dev-load sim run_nwb clean
  fi
  echo

}

usage() {
  cat <<EOF
${GREEN}just.sh${NOCOLOR} 0.0.2
Jacob Strieb
    Auto-generated from a Justfile by just.sh - https://github.com/jstrieb/just.sh

${YELLOW}USAGE:${NOCOLOR}
    ./just.sh [FLAGS] [OPTIONS] [ARGUMENTS]...

${YELLOW}FLAGS:${NOCOLOR}
        ${GREEN}--choose${NOCOLOR}      Select one or more recipes to run using a binary. If ${TICK}--chooser${TICK} is not passed the chooser defaults to the value of ${DOLLAR}JUST_CHOOSER, falling back to ${TICK}fzf${TICK}
        ${GREEN}--dump${NOCOLOR}        Print justfile
        ${GREEN}--evaluate${NOCOLOR}    Evaluate and print all variables. If a variable name is given as an argument, only print that variable's value.
        ${GREEN}--init${NOCOLOR}        Initialize new justfile in project root
    ${GREEN}-l, --list${NOCOLOR}        List available recipes and their arguments
        ${GREEN}--summary${NOCOLOR}     List names of available recipes
    ${GREEN}-u, --unsorted${NOCOLOR}    Return list and summary entries in source order
    ${GREEN}-h, --help${NOCOLOR}        Print help information
    ${GREEN}-V, --version${NOCOLOR}     Print version information

${YELLOW}OPTIONS:${NOCOLOR}
        ${GREEN}--chooser <CHOOSER>${NOCOLOR}           Override binary invoked by ${TICK}--choose${TICK}
        ${GREEN}--list-heading <TEXT>${NOCOLOR}         Print <TEXT> before list
        ${GREEN}--list-prefix <TEXT>${NOCOLOR}          Print <TEXT> before each list item
        ${GREEN}--set <VARIABLE> <VALUE>${NOCOLOR}      Override <VARIABLE> with <VALUE>
        ${GREEN}--shell <SHELL>${NOCOLOR}               Invoke <SHELL> to run recipes
        ${GREEN}--shell-arg <SHELL-ARG>${NOCOLOR}       Invoke shell with <SHELL-ARG> as an argument

${YELLOW}ARGS:${NOCOLOR}
    ${GREEN}<ARGUMENTS>...${NOCOLOR}    Overrides and recipe(s) to run, defaulting to the first recipe in the justfile
EOF
}

err_usage() {
  cat <<EOF >&2
USAGE:
    ./just.sh [FLAGS] [OPTIONS] [ARGUMENTS]...

For more information try ${GREEN}--help${NOCOLOR}
EOF
}

listfn() {
  while [ "$#" -gt 0 ]; do
    case "${1}" in
    --list-heading)
      shift
      LIST_HEADING="${1}"
      ;;

    --list-prefix)
      shift
      LIST_PREFIX="${1}"
      ;;

    -u|--unsorted)
      SORTED="false"
      ;;
    esac
    shift
  done

  printf "%s" "${LIST_HEADING}"
  if [ "${SORTED}" = "true" ]; then 
    echo "${LIST_PREFIX}"'build'' '"${CYAN}"'obj_toggle'"${NOCOLOR}"'='"${GREEN}"'""'"${NOCOLOR}""${BLUE}"' # obj_toggle toggles whether it will need external data'"${NOCOLOR}"
    echo "${LIST_PREFIX}"'clean'"${BLUE}""${NOCOLOR}"
    echo "${LIST_PREFIX}"'dev'' '"${CYAN}"'obj_toggle'"${NOCOLOR}"'='"${GREEN}"'""'"${NOCOLOR}""${BLUE}"' # obj_toggle toggles whether it will need external data'"${NOCOLOR}"
    echo "${LIST_PREFIX}"'dev-load'' '"${CYAN}"'obj_name'"${NOCOLOR}"'='"${GREEN}"'""'"${NOCOLOR}""${BLUE}"' # obj_name toggles whether it is loaded with external data, containing object name (e.g. dog) if it is'"${NOCOLOR}"
    echo "${LIST_PREFIX}"'dev-obj'' '"${CYAN}"'obj_name'"${NOCOLOR}"'='"${GREEN}"'""'"${NOCOLOR}""${BLUE}"' # obj_name contains .obj and .obj file name (e.g. dog)'"${NOCOLOR}"
    echo "${LIST_PREFIX}"'load'' '"${CYAN}"'obj_name'"${NOCOLOR}"'='"${GREEN}"'""'"${NOCOLOR}""${BLUE}"' # obj toggles whether it is loaded with external data, containing object name (e.g. dog) if it is'"${NOCOLOR}"
    echo "${LIST_PREFIX}"'obj'' '"${CYAN}"'input_file'"${NOCOLOR}"' '"${CYAN}"'obj_name'"${NOCOLOR}""${BLUE}"' # obj_name contains .pbj file name (e.g. dog)'"${NOCOLOR}"
    echo "${LIST_PREFIX}"'run_nwb'"${BLUE}""${NOCOLOR}"
    echo "${LIST_PREFIX}"'sim'"${BLUE}"' # forget about sim for now'"${NOCOLOR}"
  else
    echo "${LIST_PREFIX}"'obj'' '"${CYAN}"'input_file'"${NOCOLOR}"' '"${CYAN}"'obj_name'"${NOCOLOR}""${BLUE}"' # obj_name contains .pbj file name (e.g. dog)'"${NOCOLOR}"
    echo "${LIST_PREFIX}"'dev-obj'' '"${CYAN}"'obj_name'"${NOCOLOR}"'='"${GREEN}"'""'"${NOCOLOR}""${BLUE}"' # obj_name contains .obj and .obj file name (e.g. dog)'"${NOCOLOR}"
    echo "${LIST_PREFIX}"'build'' '"${CYAN}"'obj_toggle'"${NOCOLOR}"'='"${GREEN}"'""'"${NOCOLOR}""${BLUE}"' # obj_toggle toggles whether it will need external data'"${NOCOLOR}"
    echo "${LIST_PREFIX}"'dev'' '"${CYAN}"'obj_toggle'"${NOCOLOR}"'='"${GREEN}"'""'"${NOCOLOR}""${BLUE}"' # obj_toggle toggles whether it will need external data'"${NOCOLOR}"
    echo "${LIST_PREFIX}"'load'' '"${CYAN}"'obj_name'"${NOCOLOR}"'='"${GREEN}"'""'"${NOCOLOR}""${BLUE}"' # obj toggles whether it is loaded with external data, containing object name (e.g. dog) if it is'"${NOCOLOR}"
    echo "${LIST_PREFIX}"'dev-load'' '"${CYAN}"'obj_name'"${NOCOLOR}"'='"${GREEN}"'""'"${NOCOLOR}""${BLUE}"' # obj_name toggles whether it is loaded with external data, containing object name (e.g. dog) if it is'"${NOCOLOR}"
    echo "${LIST_PREFIX}"'sim'"${BLUE}"' # forget about sim for now'"${NOCOLOR}"
    echo "${LIST_PREFIX}"'run_nwb'"${BLUE}""${NOCOLOR}"
    echo "${LIST_PREFIX}"'clean'"${BLUE}""${NOCOLOR}"
  fi
}

dumpfn() {
  cat <<"c8dba55605fa3feb"
current_target := "x86_64-unknown-linux-gnu" # TODO: get target

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
c8dba55605fa3feb
}

evaluatefn() {
  assign_variables || exit "${?}"
  if [ "${#}" = "0" ]; then
    echo 'current_target := "'"${VAR_current_target}"'"'
  else
    case "${1}" in
    current_target)
      printf "%s" "${VAR_current_target}"
      ;;
    *)
      echo_error 'Justfile does not contain variable `'"${1}"'`.'
      exit 1
      ;;
    esac
  fi
}

choosefn() {
  echo 'obj' 'dev-obj' 'build' 'dev' 'load' 'dev-load' 'sim' 'run_nwb' 'clean' \
    | "${DEFAULT_SHELL}" ${DEFAULT_SHELL_ARGS} "${CHOOSER}"
}


#########################################################################################
# Main entrypoint                                                                       #
#########################################################################################

RUN_DEFAULT='true'
while [ "${#}" -gt 0 ]; do
  case "${1}" in 
  
  # User-defined recipes
  obj)
    shift
    assign_variables || exit "${?}"
    FUN_obj "$@"
    RUN_DEFAULT='false'
    if [ "${#}" -ge "2" ]; then
      shift 2
    elif [ "${#}" -gt 0 ]; then
      shift "${#}"
    fi
    ;;

  dev-obj)
    shift
    assign_variables || exit "${?}"
    FUN_dev_obj "$@"
    RUN_DEFAULT='false'
    if [ "${#}" -ge "1" ]; then
      shift 1
    elif [ "${#}" -gt 0 ]; then
      shift "${#}"
    fi
    ;;

  build)
    shift
    assign_variables || exit "${?}"
    FUN_build "$@"
    RUN_DEFAULT='false'
    if [ "${#}" -ge "1" ]; then
      shift 1
    elif [ "${#}" -gt 0 ]; then
      shift "${#}"
    fi
    ;;

  dev)
    shift
    assign_variables || exit "${?}"
    FUN_dev "$@"
    RUN_DEFAULT='false'
    if [ "${#}" -ge "1" ]; then
      shift 1
    elif [ "${#}" -gt 0 ]; then
      shift "${#}"
    fi
    ;;

  load)
    shift
    assign_variables || exit "${?}"
    FUN_load "$@"
    RUN_DEFAULT='false'
    if [ "${#}" -ge "1" ]; then
      shift 1
    elif [ "${#}" -gt 0 ]; then
      shift "${#}"
    fi
    ;;

  dev-load)
    shift
    assign_variables || exit "${?}"
    FUN_dev_load "$@"
    RUN_DEFAULT='false'
    if [ "${#}" -ge "1" ]; then
      shift 1
    elif [ "${#}" -gt 0 ]; then
      shift "${#}"
    fi
    ;;

  sim)
    shift
    assign_variables || exit "${?}"
    FUN_sim "$@"
    RUN_DEFAULT='false'
    ;;

  run_nwb)
    shift
    assign_variables || exit "${?}"
    FUN_run_nwb "$@"
    RUN_DEFAULT='false'
    ;;

  clean)
    shift
    assign_variables || exit "${?}"
    FUN_clean "$@"
    RUN_DEFAULT='false'
    ;;
  
  # Built-in flags
  -l|--list)
    shift 
    listfn "$@"
    RUN_DEFAULT="false"
    break
    ;;
    
  -f|--justfile)
    shift 2
    echo "${YELLOW}warning${NOCOLOR}: ${BOLD}-f/--justfile not implemented by just.sh${NOCOLOR}" >&2
    ;;

  --summary)
    shift
    summarizefn "$@"
    RUN_DEFAULT="false"
    break
    ;;

  --list-heading)
    shift
    LIST_HEADING="${1}"
    shift
    ;;

  --list-prefix)
    shift
    LIST_PREFIX="${1}"
    shift
    ;;

  -u|--unsorted)
    SORTED="false"
    shift
    ;;

  --shell)
    shift
    DEFAULT_SHELL="${1}"
    shift
    ;;

  --shell-arg)
    shift
    DEFAULT_SHELL_ARGS="${1}"
    shift
    ;;
    
  -V|--version)
    shift
    echo "just.sh 0.0.2"
    echo
    echo "https://github.com/jstrieb/just.sh"
    RUN_DEFAULT="false"
    break
    ;;

  -h|--help)
    shift
    usage
    RUN_DEFAULT="false"
    break
    ;;

  --choose)
    shift
    assign_variables || exit "${?}"
    TARGET="$(choosefn)"
    env "${0}" "${TARGET}" "$@"
    RUN_DEFAULT="false"
    break
    ;;
    
  --chooser)
    shift
    CHOOSER="${1}"
    shift
    ;;
    
  *=*)
    assign_variables || exit "${?}"
    NAME="$(
        echo "${1}" | tr '\n' '\r' | sed 's/\([^=]*\)=.*/\1/g' | tr '\r' '\n'
    )"
    VALUE="$(
        echo "${1}" | tr '\n' '\r' | sed 's/[^=]*=\(.*\)/\1/g' | tr '\r' '\n'
    )"
    shift
    set_var "${NAME}" "${VALUE}"
    ;;

  --set)
    shift
    assign_variables || exit "${?}"
    NAME="${1}"
    shift
    VALUE="${1}"
    shift
    set_var "${NAME}" "${VALUE}"
    ;;
    
  --dump)
    RUN_DEFAULT="false"
    dumpfn "$@"
    break
    ;;
    
  --evaluate)
    shift
    RUN_DEFAULT="false"
    evaluatefn "$@"
    break
    ;;
    
  --init)
    shift
    RUN_DEFAULT="false"
    if [ -f "justfile" ]; then
      echo_error "Justfile "'`'"$(realpath "justfile")"'`'" already exists"
      exit 1
    fi
    cat > "justfile" <<EOF
default:
    echo 'Hello, world!'
EOF
    echo 'Wrote justfile to `'"$(realpath "justfile")"'`' 2>&1 
    break
    ;;

  -*)
    echo_error "Found argument '${NOCOLOR}${YELLOW}${1}${NOCOLOR}${BOLD}' that wasn't expected, or isn't valid in this context"
    echo >&2
    err_usage
    exit 1
    ;;

  *)
    assign_variables || exit "${?}"
    echo_error 'Justfile does not contain recipe `'"${1}"'`.'
    exit 1
    ;;
  esac
done

if [ "${RUN_DEFAULT}" = "true" ]; then
  if [ "${#}" -lt "2" ]; then
    echo_error 'Recipe `obj` cannot be used as default recipe since it requires at least 2 arguments.'
    exit 1
  fi
  assign_variables || exit "${?}"
  FUN_obj "$@" 
fi


#########################################################################################
#                                                                                       #
# This script was auto-generated from a Justfile by just.sh.                            #
#                                                                                       #
# Generated on 2025-10-06 with just.sh version 0.0.2.                                   #
# https://github.com/jstrieb/just.sh                                                    #
#                                                                                       #
# Run `./just.sh --dump` to recover the original Justfile.                              #
#                                                                                       #
#########################################################################################

