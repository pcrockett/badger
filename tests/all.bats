#!/usr/bin/env bats
# shellcheck disable=SC2030  # exporting vars in tests is a local operation
# shellcheck disable=SC2031  # exporting vars in tests is a local operation

source tests/util.sh

@test 'main help - always - works' {
  capture_output badger --help
  assert_no_stderr
  assert_stdout 'Usage:'
  assert_exit_code 0
}

@test 'next - no pending notifications - returns nothing' {
  capture_output badger next
  assert_no_stderr
  assert_no_stdout
  assert_exit_code 0
}

@test 'next - multiple pending notifications - returns first' {
  badger publish "this is a test"
  badger publish "this is another test"
  capture_output badger next
  assert_no_stderr
  assert_stdout "^this is a test$"
  assert_exit_code 0

  capture_output badger next
  assert_no_stderr
  assert_stdout "^this is another test$"
  assert_exit_code 0

  capture_output badger next
  assert_no_stderr
  assert_no_stdout
  assert_exit_code 0
}

@test 'publish - verbose - returns file path' {
  capture_output badger publish "hi" --verbose
  assert_stdout "^Saved to ${XDG_STATE_HOME}/badger/.+_000\.json$"
  assert_no_stderr
  assert_exit_code 0
}

@test 'publish - custom timestamp - uses timestamp' {
  BADGER_TIMESTAMP="WHATEVER" badger publish "hi"
  test -f "${TEST_HOME}/.local/state/badger/WHATEVER_000.json" || {
    ls -lha "${TEST_HOME}/.local/state/badger"
    fail "Couldn't find the correct file in state dir."
  }

  capture_output badger next
  assert_no_stderr
  assert_stdout "^hi$"
  assert_exit_code 0
}

@test 'publish - timestamp collision - increments counter' {
  export BADGER_TIMESTAMP="WHATEVER"
  badger publish "first"
  badger publish "second"
  badger publish "third"

  for index in 000 001 002; do
    test -f "${XDG_STATE_HOME}/badger/WHATEVER_${index}.json"
  done
}

@test 'publish - index overflow - errors' {
  export BADGER_TIMESTAMP="hi"
  badger_dir="${XDG_STATE_HOME}/badger"
  mkdir --parent "${badger_dir}"

  # fill up our indexes
  seq --equal-width 0 999 \
    | xargs -L 1 printf "${badger_dir}/hi_%s.json\n" \
    | xargs touch

  capture_output badger publish "hello"
  assert_stderr "Exhausted unique message slugs"
  assert_no_stdout
  assert_exit_code 1
}
