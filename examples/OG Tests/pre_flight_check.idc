on "start"
    # Arithmetic base
    load a from "2"
    load b from "3"
    load result from "{{a}} + {{b}} * 2"
    respond "Computed result: {{result}}"        # Should be 8

    # Arithmetic expression substitution check
    load test_expr from "{{result}} / 2 + 1"
    respond "Expression result: {{test_expr}}"   # Should be 5

    # Conditional true
    if {{test_expr}} == 5
        respond "Passed: conditional true"
    else
        respond "Failed: conditional true"

    # Conditional false
    if {{test_expr}} == 99
        respond "Failed: conditional false"
    else
        respond "Passed: conditional false"

    # Loop with continue + break
    load counter from "0"
    while {{counter}} < 5
        if {{counter}} == 2
            respond "Continue hit at {{counter}}"
            load counter from "{{counter}} + 1"
            continue

        if {{counter}} == 4
            respond "Break hit at {{counter}}"
            break

        respond "Looping at {{counter}}"
        load counter from "{{counter}} + 1"

    # Invalid math fallback check
    load divzero from "10 / 0"
    respond "Handled div by zero: {{divzero}}"

    load parenfail from "1 + (2 *"
    respond "Handled broken math: {{parenfail}}"

    load unknownchar from "7 & 3"
    respond "Handled unknown char: {{unknownchar}}"

    # Delete and memory cleanup test
    delete test_expr
    delete counter
    delete divzero
    delete parenfail
    delete unknownchar
    respond "Deleted test vars. Remaining: a, b, result"

    # Stubbed command sweep
    remember foo
    get bar
    capture thought
    define logic
    alias altname

    print "Hello"
    log "Event logged"
    show warning
    alert user

    stop
    retry
    then

    authorize token
    purge cache
    reset session

    connect service
    disconnect service
    save file
    update config

    check status
    verify identity

    use "tool"
    from "source"
    to "target"
    with "input"
    as "name"
    every "hour"
    route "destination"

    # Unknown handler validation
    mysteryblock "test"
        respond "Child of unknown block runs fine"

    # Memory echo test
    respond "Final memory check: a={{a}}, b={{b}}, result={{result}}"

    # Pre-flight success
    respond "Interpreter readiness confirmed."
