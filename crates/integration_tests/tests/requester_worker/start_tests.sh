cargo make local-node > /dev/null 2>&1 &
sleep 5

PID=$(pgrep -f target/debug/node)
echo "Pid >>> $PID"

cargo make requester-worker
ERROR_CODE=$(echo $?)

kill -2 $PID

exit $ERROR_CODE
