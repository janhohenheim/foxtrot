#!/usr/bin/env bash

set -euo pipefail
shopt -s globstar nullglob

readonly ASSETS_PATH='assets'

usage() {
    printf "Usage: %s [options]

Options:
  -q, --qbsp <path>    Path to qbsp executable
  -l, --light <path>   Path to light executable
  -h, --help           Show this help message
" "$0"
}

error() {
    echo "$0: $*" >&2
}

main() {
    # Parse arguments.
    local qbsp_path='qbsp'
    local light_path='light'
    while [[ "$#" -ge 1 ]]; do
        case "$1" in
            -q|--qbsp)
                qbsp_path="$2"
                shift
                ;;
            -l|--light)
                light_path="$2"
                shift
                ;;
            -h|--help)
                usage
                return 0
                ;;
            -*)
                error "Unknown option: $1"
                return 1
                ;;
            *)
                error "Unexpected positional argument: $1"
                return 1
                ;;
        esac

        shift
    done
    readonly qbsp_path
    readonly light_path

    # Verify that the qbsp and light executables exist.
    if ! type "${qbsp_path}" >/dev/null 2>&1; then
        error "qbsp executable not found at path: ${qbsp_path}"
        return 1
    fi
    if ! type "${light_path}" >/dev/null 2>&1; then
        error "light executable not found at path: ${light_path}"
        return 1
    fi

    # Compile maps.
    for map in "${ASSETS_PATH}"/maps/**/*.map; do
        echo "Compiling ${map}"
        "${qbsp_path}" -bsp2 -wrbrushesonly -nosubdivide -nosoftware -path "${ASSETS_PATH}" -notex "${map}" "${map%.map}.bsp"
        "${light_path}" -extra4 -novanilla -lightgrid -path "${ASSETS_PATH}" "${map%.map}.bsp"
    done
}

main "$@"
exit "$?"
