#!/usr/bin/env bats

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
