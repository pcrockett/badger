#!/usr/bin/env bats
# shellcheck disable=SC2030  # exporting vars in tests is a local operation
# shellcheck disable=SC2031  # exporting vars in tests is a local operation

source tests/util.sh

@test 'main - always - displays help' {
  capture_output badger
  assert_no_stdout
  assert_stderr 'Usage:'
  assert_exit_code 2
}

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

@test 'publish - flag after option separator - treated as message' {
  capture_output badger publish -- --level
  assert_no_stdout
  assert_no_stderr
  assert_exit_code 0

  capture_output badger next
  assert_stdout "^--level$"
}

@test 'publish - single hyphen long flag - treated as message' {
  capture_output badger publish -level
  assert_no_stdout
  assert_no_stderr
  assert_exit_code 0

  capture_output badger next
  assert_stdout "^-level$"

  capture_output badger publish --level -level message
  assert_no_stdout
  assert_no_stderr
  assert_exit_code 0

  capture_output badger next --format json
  assert_stdout '^\{
  "message": "message",
  "level": "-level",
  "data": null
}$'
}

@test 'publish - option separator twice - treated as message' {
  capture_output badger publish -- --
  assert_no_stdout
  assert_no_stderr
  assert_exit_code 0

  capture_output badger next
  assert_stdout "^--$"
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
  assert_stderr "Error: unable to save notification with timestamp \`hi\`"
  assert_no_stdout
  assert_exit_code 1
}

@test 'next - peek - leaves notification in queue' {
  badger publish "this is a test"
  capture_output badger next --peek
  assert_no_stderr
  assert_stdout "^this is a test$"
  assert_exit_code 0

  capture_output badger next
  assert_no_stderr
  assert_stdout "^this is a test$"
  assert_exit_code 0
}

@test 'count - always - counts notifications' {
  capture_output badger count
  assert_no_stderr
  assert_stdout "^0$"
  assert_exit_code 0

  badger publish "hello"
  capture_output badger count
  assert_no_stderr
  assert_stdout "^1$"
  assert_exit_code 0

  badger publish "hello again"
  capture_output badger count
  assert_no_stderr
  assert_stdout "^2$"
  assert_exit_code 0

  badger next &>/dev/null
  capture_output badger count
  assert_no_stderr
  assert_stdout "^1$"
  assert_exit_code 0
}

@test 'pending - no notifications - exit code 1' {
  capture_output badger pending
  assert_no_stdout
  assert_no_stderr
  assert_exit_code 1
}

@test 'pending - notifications - exit code 0' {
  badger publish hi
  capture_output badger pending
  assert_no_stdout
  assert_no_stderr
  assert_exit_code 0
}

@test 'next - json - outputs json' {
  capture_output badger publish hello --data '{"foo": "bar", "whatever": true}'
  assert_exit_code 0

  capture_output badger next --format json
  assert_no_stderr
  assert_exit_code 0
  assert_stdout '^\{
  "message": "hello",
  "level": "info",
  "data": \{
    "foo": "bar",
    "whatever": true
  }
}$'
}

@test 'publish - json on stdin - records data' {
  capture_output badger publish hello --data - <<EOF
{"foo": "bar", "whatever": true}
EOF

  assert_exit_code 0

  capture_output badger next --format json
  assert_no_stderr
  assert_exit_code 0
  assert_stdout '^\{
  "message": "hello",
  "level": "info",
  "data": \{
    "foo": "bar",
    "whatever": true
  }
}$'
}

@test 'version - always - returns consistent format' {
  capture_output badger --version
  assert_exit_code 0
  assert_no_stderr

  # if this format ever changes, don't forget to modify the release workflow
  assert_stdout '^badger [[:digit:]]+\.[[:digit:]]+\.[[:digit:]]+$'
}

@test 'run - zero exit - does not publish' {
  capture_output badger run true
  assert_exit_code 0
  assert_no_stderr
  assert_no_stdout
  test "$(badger count)" == "0"
}

@test 'run - nonzero exit - publishes' {
  capture_output badger run false
  assert_exit_code 1
  assert_no_stderr
  assert_no_stdout

  capture_output badger next --format json

  # shellcheck disable=SC2016
  assert_stdout '^\{
  "message": "`false` exited with code 1.",
  "level": "error",
  "data": \{
    "command": "false",
    "exit_code": 1,
    "signal": null
  }
}$'
}

@test 'run - child process sigtermmed - publishes' {
  badger run "sleep 10" &
  badger_pid=$!
  sleep 1
  badger_processes="$(ps --ppid "${badger_pid}" -o pid,cmd --no-headers)"
  sleep_pid="$(
    echo "${badger_processes}" \
      | awk 'index($0, "sleep 10") > 0 { printf("%s\n", $1) }'
  )"
  test "${sleep_pid}" != "" || fail "unable to determine the sleep process PID\n\n${badger_processes}"
  kill -SIGTERM "${sleep_pid}"

  # ensure badger exited non-zero
  wait -n && fail "badger exited with code $?"

  capture_output badger next --format json
  # shellcheck disable=SC2016
  assert_stdout '^\{
  "message": "`sleep 10` was terminated with signal 15.",
  "level": "error",
  "data": \{
    "command": "sleep 10",
    "exit_code": null,
    "signal": 15
  }
}$'
}

@test 'run - badger sigtermmed - publishes' {
  badger run "sleep 10" &
  badger_pid=$!
  sleep 1
  kill -SIGTERM "${badger_pid}"

  # ensure badger exited non-zero
  wait -n && fail "badger exited with code $?"

  capture_output badger next --format json
  # shellcheck disable=SC2016
  assert_stdout '^\{
  "message": "`sleep 10` was terminated with signal 15.",
  "level": "error",
  "data": \{
    "command": "sleep 10",
    "exit_code": null,
    "signal": 15
  }
}$'
}

@test 'run - always - preserves stdin' {
  capture_output badger run cat < <(echo foo)
  assert_exit_code 0
  assert_no_stderr
  assert_stdout "^foo$"

  capture_output badger run "cat; exit 1" < <(echo foo)
  assert_exit_code 1
  assert_no_stderr
  assert_stdout "^foo$"
}

@test 'run - always - preserves stdout' {
  capture_output badger run "echo foo"
  assert_exit_code 0
  assert_no_stderr
  assert_stdout "^foo$"

  capture_output badger run "echo foo; exit 1"
  assert_exit_code 1
  assert_no_stderr
  assert_stdout "^foo$"
}

@test 'run - always - preserves stderr' {
  capture_output badger run "echo foo >&2"
  assert_exit_code 0
  assert_no_stdout
  assert_stderr "^foo$"

  capture_output badger run "echo foo >&2; exit 1"
  assert_exit_code 1
  assert_no_stdout
  assert_stderr "^foo$"
}

@test 'run - trailing args - interpreted as command' {
  capture_output badger run echo foo bar
  assert_no_stderr
  assert_stdout "foo bar"
  assert_exit_code 0

  capture_output badger run -- echo foo bar
  assert_no_stderr
  assert_stdout "foo bar"
  assert_exit_code 0

  capture_output badger run echo foo bar --shell bash
  assert_no_stderr
  assert_stdout "foo bar"
  assert_exit_code 0

  capture_output badger run -- echo foo bar --shell bash
  assert_no_stderr
  assert_stdout "foo bar --shell bash"
  assert_exit_code 0
}
