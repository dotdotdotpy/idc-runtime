on "start"
    load counter from 3
    respond "Counter initialized to {{counter}}"

    while counter
        respond "Current count: {{counter}}"
        delete counter
        load counter from 2

        if counter
            respond "Still looping, counter = {{counter}}"
        else
            respond "Should break now"
            break

    respond "Exited loop"

    load name from "Grant"
    respond "Hello, {{name}}"
    delete name
    respond "After delete: {{name}}"
