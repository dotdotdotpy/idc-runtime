on start
    remember counter
    remember limit
    load "5" as limit to global
    set promote_always true
    reflect flags
    reflect event
    reflect stack
    reflect log
    reflect memory all

every 1
    load "{{counter}} + 1" as counter to global
    respond "Tick {{counter}} of {{limit}}"
    if "{{counter}} > {{limit}}"
        trigger "end"
    reflect flags

on tick
    respond_event "{{counter}} * 2"
    reflect memory local

on end
    respond "Reached limit! counter = {{counter}}, ending script."
    reflect memory json
    reflect log
    stop
