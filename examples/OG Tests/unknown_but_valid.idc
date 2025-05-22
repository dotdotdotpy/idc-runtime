on "start"
    load name from "Grant"
    respond "User is" {{name}}

    unknown_action "mystery"
        respond "This will still run"

    create "session"

    if name
        respond "Name is valid: {{name}}"
    else
        respond "Name is missing"

    delete name
    respond "After delete:" {{name}}

    unknown_thing
        load debug from "true"
        respond "Debug mode: {{debug}}"
