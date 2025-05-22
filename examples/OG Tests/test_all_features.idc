on "start"
    load a from "2"
    load b from "3"
    load sum from "{{a}} + {{b}}"
    respond "Sum is:" {{sum}}

    if {{sum}} == 5
        respond "Condition passed: sum equals 5"
    else
        respond "Condition failed"

    load counter from "0"
    while {{counter}} < 3
        respond "Loop iteration:" {{counter}}
        load counter from "{{counter}} + 1"

    delete sum
    delete counter
    respond "Deleted vars. Memory should now only have a and b."

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