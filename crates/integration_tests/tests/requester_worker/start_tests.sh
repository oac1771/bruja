cargo make local-node > /dev/null 2>&1 &
sleep 5

PID=$(pgrep -f target/debug/node)

cargo test -p integration_tests --test requester_worker --features integration_tests  -- --nocapture
ERROR_CODE=$(echo $?)

kill -2 $PID

exit $ERROR_CODE
