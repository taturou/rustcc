#!/bin/bash
CLANG="/usr/bin/clang"
LLI="/usr/bin/lli-6.0"

try() {
  expected="$1"
  input="$2"

  cargo run "$input" > tmp.ll
  if [ -e $CLANG ]; then
    $CLANG -o tmp ./tmp.ll
    ./tmp
  else
    $LLI ./tmp.ll
  fi
  actual="$?"

  if [ "$actual" != "$expected" ]; then
    echo "$input expected, but got $actual"
    exit 1
  fi
}

try 0 0
try 42 42

echo OK
