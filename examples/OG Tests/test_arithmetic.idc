on "start"
    load a from "4"
    load b from "6"
    load result from "{{a}} + {{b}}"
    respond "The result is: {{result}}"

    if result == 10
        respond "It worked!"
    else
        respond "Something went wrong"
