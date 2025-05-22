on "start"
    load x from "10"
    load y from "5"
    load z from "{{x}} * {{y}}"
    respond "Multiplied: {{z}}"

    if z == 50
        respond "Math check passed!"
    else
        respond "Math check failed!"
