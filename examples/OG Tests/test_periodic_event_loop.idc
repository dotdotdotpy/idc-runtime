on "start"
    load limit from "3"
    respond "Starting periodic event loop test"

    # Start the periodic event loop with a limit of 3 iterations
    start_periodic_event with limit

    # Respond when the loop finishes
    respond "Periodic event loop test completed"
