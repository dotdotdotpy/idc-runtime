on "start"
    // Loop that breaks after one iteration
    while true
        respond "This loop will break after one iteration"
        break  // Break immediately after first iteration

    // Loop that continues for 3 iterations, then breaks
    load counter from "0"
    while counter < 3
        respond "This loop will continue indefinitely unless broken"
        load counter from "{{counter}} + 1"  // Increment counter by 1
        if counter == 3
            respond "This loop will now stop after 3 iterations"
            break  // Break after 3 iterations

    // Loop that continues until the counter reaches 2, then breaks
    load counter from "0"
    while counter < 3
        respond "This loop will continue indefinitely unless broken"
        load counter from "{{counter}} + 1"  // Increment counter by 1
        if counter == 2
            respond "Stopping condition met, breaking loop"
            break  // Break when counter reaches 2
