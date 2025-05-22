on "start"
    load depth from "2"
    respond "Init depth = {{depth}}"

    while depth
        respond "Looping with depth = {{depth}}"

        if depth
            respond "Depth is still active: {{depth}}"
            delete depth
            load depth from "0"  # ← Set to 0 so loop exits on next check
        else
            respond "Depth is now inactive"
            break

    respond "Exited depth loop"
