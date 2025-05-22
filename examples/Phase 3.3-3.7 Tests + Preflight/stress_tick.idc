on start
    load counter from "0"
    respond "Starting stress tick test with counter = {{counter}}"

every 1
    respond "Tick {{counter}} entered"
    if "{{counter}} < 100"
        load counter "{{counter}} + 1"
        respond_event "{{counter}}"
        if "{{counter}} % 25 == 0"
            respond "Reached checkpoint: {{counter}}"
        end
    else
        respond "Tick loop completed at {{counter}}"
        trigger done
    end

on done
    respond "Stress tick sequence finished at count {{counter}}"
    reflect memory json
    reflect stack
    reflect log
