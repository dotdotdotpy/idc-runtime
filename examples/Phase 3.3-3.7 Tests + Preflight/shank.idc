on start
    load counter value "1"
    respond "Starting loop at {{counter}}"
    trigger loop_logic
    reflect memory json
    reflect stack
    reflect log

on loop_logic
    if "{{counter}} <= 3"
        respond "Loop count is {{counter}}"
        load counter value "{{counter}} + 1"
        wait
        trigger loop_logic
    unless "{{counter}} <= 3"
        respond "Done looping at count {{counter}}"
