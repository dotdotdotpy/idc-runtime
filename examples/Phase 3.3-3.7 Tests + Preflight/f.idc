on start
    load a "5"
    load b "3"
    load sum "{{a}} + {{b}}"
    load double_sum "{{sum}} * 2"
    load condition_check "{{double_sum}} > 10"

    respond "Sum is {{sum}}, Double Sum is {{double_sum}}"
    respond_event "{{double_sum}}"

    if "{{double_sum}} > 10"
        respond "Yes, double_sum is greater than 10!"
    unless "{{b}} > 10"
        respond "b is not greater than 10"

    trigger nested_math

on nested_math
    load result "{{a}} + ({{b}} * 3)"
    respond "Nested result = {{result}}"
