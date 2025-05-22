on "start"
    # Testing event handling: Trigger the 'on' event
    respond "Testing 'on' event: Start triggered"

    # Load some values into memory
    load a from "2"
    load b from "3"
    load sum from "{{a}} + {{b}}"
    respond "Sum of a and b: {{sum}}"  # Expected: 5

    # Testing conditional logic: Check if sum is correct
    if {{sum}} == 5
        respond "Condition passed: sum equals 5"
    else
        respond "Condition failed"

    # Trigger 'every' event by starting a loop
    respond "Testing 'every' event loop: This should print every 1 second"
    every "1" second

    # Load counter value and run a loop
    load counter from "0"
    while {{counter}} < 3
        respond "Counter iteration: {{counter}}"
        load counter from "{{counter}} + 1"
        # Simulating a delay (in real tests, we can make this sleep for real)
        wait "1" second

    # Cleanup and memory check
    delete a
    delete b
    delete sum
    respond "Memory cleaned up. Remaining: {{counter}}"

    # Testing route handling
    route "main_route"

    # Final memory check and response
    respond "Final memory state: {{counter}}"
    respond "End of test runtime."
