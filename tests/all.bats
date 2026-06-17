#!/usr/bin/env bats

source tests/util.sh

@test '--help works' {
  capture_output badger --help
  assert_no_stderr
  assert_stdout 'Usage:'
  assert_exit_code 0
}
