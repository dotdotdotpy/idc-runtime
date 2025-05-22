on "start"
    # Valid math
    load a from "2"
    load b from "3"
    load sum from "{{a}} + {{b}} * 2"
    respond "Sum: {{sum}}"  # Expect: Sum: 8

    # Division
    load div from "{{sum}} / 2"
    respond "Div: {{div}}"  # Expect: Div: 4

    # Edge case: division by zero
    load bad_div from "10 / 0"
    respond "Should be handled: {{bad_div}}"  # Expect math error, fallback value

    # Mismatched parenthesis
    load broken from "1 + (2 *"
    respond "Broken: {{broken}}"  # Expect error msg logged, fallback to raw string

    # Unknown char
    load fail from "7 & 2"
    respond "Bad token: {{fail}}"  # Expect error msg logged
