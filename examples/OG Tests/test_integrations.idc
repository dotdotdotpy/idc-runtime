on "start"
    # Load initial values
    load x from "3"
    load y from "7"
    load sum from "{{x}} + {{y}}"
    respond "Sum of x and y is: {{sum}}"

    # Test conditionals
    if {{sum}} == 10
        respond "Condition passed: sum equals 10"
    else
        respond "Condition failed"

    # Test while loop
    load counter from "0"
    while {{counter}} < 3
        respond "Counter: {{counter}}"
        load counter from "{{counter}} + 1"

    # Test math error handling
    load div from "10 / 0"
    respond "Handled div by zero: {{div}}"

    load broken_math from "5 + (2 *"
    respond "Handled broken math: {{broken_math}}"

    # Test event handling
    on "every"
        load counter from "{{counter}} + 1"
        respond "Every loop: {{counter}}"

    # Delete memory and check state
    delete sum
    delete counter
    respond "Deleted sum and counter. Remaining: x, y, and div"

    # Check if we have undefined behavior
    remember foo
    get bar
    capture baz
    define thing
    alias alias_name
    print
    log
    show
    alert
    stop
    retry
    then
    authorize
    purge
    reset
    connect
    disconnect
    save
    update
    check
    verify
    use
    from
    to
    with
    as
    every
    route
