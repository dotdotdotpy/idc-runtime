on start
    remember counter 0

every 1
    respond "Tick {{counter}} entered"
    if "{{counter}} < 100"
        load counter "{{counter}} + 1"
        if "{{counter}} % 25 == 0"
            respond "Checkpoint hit: {{counter}}"
        end
    else
        respond "Final value: {{counter}}"
        reflect memory all
        stop
    end
