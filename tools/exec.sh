#!/bin/zsh

CASE_PATH=$1
BIN_DIR=./tmp
shift

run_command=""

for arg in "$@"; do
  case $arg in
  --lang=java)
    javac "$CASE_PATH/java/src/Main.java" -d "$BIN_DIR"
    run_command="java -cp $BIN_DIR Main"
    shift
    ;;
  --lang=rust)
    rustc "$CASE_PATH/rust/src/main.rs" --out-dir "$BIN_DIR"
    run_command="./$BIN_DIR/main"
    shift
    ;;
  *)
    echo "Invalid language specified. Please use --lang=java or --lang=rust"
    exit 1
    ;;
  esac
done

for case_file in "$CASE_PATH/cases/"*-in; do
  temp_file=$(mktemp) # Create a temporary file
  eval $run_command <"$case_file" >"$temp_file"
  expected_out_file="${case_file%-in}-out" # Replace "-in" with "-out" in the filename

  # Remove trailing newline and empty lines from files
  awk '{if (NR>1 && length($0)==0 && length(prev)==0) {next}; prev=$0; print}' "$temp_file" >"$temp_file.stripped"
  awk '{if (NR>1 && length($0)==0 && length(prev)==0) {next}; prev=$0; print}' "$expected_out_file" >"$expected_out_file.stripped"

  if diff -q "$temp_file.stripped" "$expected_out_file.stripped" >/dev/null; then
    echo -e "Test case $case_file \033[32mpassed.\033[0m"
  else
    echo -e "Test case $case_file \033[31mfailed.\033[0m"
  fi

  rm "$temp_file" "$temp_file.stripped" "$expected_out_file.stripped" # Remove the temporary files
done
