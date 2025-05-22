#  IDC Usage Guide

This document explains how to write and run programs using IDC.

---

##  Writing an IDC Program

Each line is a sentence made from IDC's 40-token vocabulary.

###  Basic Sentence

```
load config from "./settings.json"
respond "Initialized" with status 200
```

###  Conditional Logic

```
if query.id is missing
  respond "Missing ID" with status 400
else
  load user from db
  respond user
```

###  Chained Flow

```
on server start → route "/status" → respond "OK"
```

---

##  Memory & Variables

Use `remember`, `get`, `define`, and `capture`:

```
remember email as user_email
get user_email
```

Capture dynamic output:

```
capture response from verify token
```

---

##  Repeating Logic

```
every 5 minutes
  load metrics → if cpu > 90 → alert admin
```

---

##  Testing & Debugging

Use `test`, `print`, `log`, and `explain:`

```
test connection
print "Start"
log result to "debug.log"
```

Coming soon:
- `explain:` why a block failed
- CLI flags: `--tokens`, `--ast`, `--run`

---

##  Running IDC from CLI

Use the CLI runner:

```
idc run examples/server.idc
```

Flags:

- `--tokens`: print token stream
- `--ast`: print AST
- `--run`: execute program

---

##  Sample Program

```
on user login
  remember now as login_time
  log "User logged in"
  respond "Welcome"
```

This logs the login time and replies with a welcome message.