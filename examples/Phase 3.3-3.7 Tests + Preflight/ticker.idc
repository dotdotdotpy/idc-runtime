on start
    remember counter 0

every 1
    load "{{counter}} + 1" as counter to global

    if "{{counter}} % 25 == 0"
        respond Checkpoint reached: {{counter}}

    if "{{counter}} == 100"
        respond Final count reached at {{counter}}
        reflect memory json
        reflect stack
        reflect log
        stop
