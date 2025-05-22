on start
    remember counter 0
    remember threshold 3
    remember msg "preflight success"
    respond startup "Boot OK: Counter at {{counter}}"
end

every 1
    load "{{counter}} + 1" as counter
    if "{{counter}} >= {{threshold}}"
        respond checkpoint "Tick {{counter}} passed threshold"
        break
    else
        respond ticking "Tick {{counter}} not yet at {{threshold}}"
    end
end

on stop
    reflect memory
    reflect stack
    reflect log
    reflect event
    reflect flags
    respond summary "{{msg}} after {{counter}} ticks"
end
