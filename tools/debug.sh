#!/bin/zsh

CASE_PATH=$1
BIN_DIR=./tmp
shift

run_command=""
case_id=""

for arg in "$@"; do
  case $arg in
  --lang=java)
    javac "$CASE_PATH/Main.java" -d "$BIN_DIR"
    run_command="java -cp $BIN_DIR Main"
    shift
    ;;
  --lang=rust)
    rustc "$CASE_PATH/main.rs" --out-dir "$BIN_DIR"
    run_command="./$BIN_DIR/main"
    shift
    ;;
  --case-id=*)
    case_id="${arg#*=}"
    shift
    ;;
  *)
    echo "Invalid argument. Please use --lang=java, --lang=rust, or --case-id=<integer>"
    exit 1
    ;;
  esac
done

if [ -z "$case_id" ]; then
  echo "Please specify a case id with --case-id=<integer>"
  exit 1
fi

case_file="$CASE_PATH/cases/$case_id-in"

temp_file=$(mktemp) # Create a temporary file
eval $run_command <"$case_file"
