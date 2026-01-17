#!/bin/bash
# Auto-limit CPU for pt-core processes
# Run with: nohup ./cpu-limiter.sh &

LIMIT=50  # Max CPU % per process

while true; do
    for pid in $(pgrep -f "pt-core"); do
        # Check if already being limited
        if ! pgrep -f "cpulimit.*$pid" > /dev/null 2>&1; then
            cpu=$(ps -p $pid -o %cpu= 2>/dev/null | tr -d ' ')
            if [[ -n "$cpu" ]] && (( $(echo "$cpu > 80" | bc -l) )); then
                echo "[$(date)] Limiting PID $pid (CPU: $cpu%)"
                cpulimit -p $pid -l $LIMIT -b 2>/dev/null
            fi
        fi
    done
    sleep 5
done
