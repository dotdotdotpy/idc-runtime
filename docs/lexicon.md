# IDC Foundational 40 Lexicon


## IDC Foundational 40 Lexicon

This document defines the first 40 core words (tokens) of the IDC language. Each entry includes the token, its category, a short definition, and example uses in real IDC syntax.

on

**Type:** Event

Triggers logic when a specific event happens.

**Examples:**
- `on user login, verify token`
- `on error, log to "errors.log"`
every

**Type:** Event

Creates a repeating timer event that executes logic at intervals.

**Examples:**
- `every 10 minutes, fetch status`
- `every day at 3am, run backup`
if

**Type:** Condition

Checks if a condition is true before continuing logic.

**Examples:**
- `if user.id is missing, respond "Missing ID"`
- `if error, retry once → else notify admin`
unless

**Type:** Condition

Runs logic only if a condition is false.

**Examples:**
- `unless user.is_verified, deny access`
- `unless token valid, stop`
is

**Type:** Condition

Tests identity or equality.

**Examples:**
- `if role is admin, show dashboard`
- `if token is expired, redirect to login`
has

**Type:** Condition

Tests whether an object or memory contains something.

**Examples:**
- `if user has email, send verification`
- `if cart has items, proceed to checkout`
load

**Type:** Action

Loads data or files into memory or runtime scope.

**Examples:**
- `load config from "./settings.json"`
- `load users from db`
save

**Type:** Action

Saves data to a destination (file, memory, database).

**Examples:**
- `save session to "sessions/"`
- `save current_user to memory`
remember

**Type:** Memory

Stores a value into memory using a named key.

**Examples:**
- `remember name as username`
- `remember query.id as user_id`
get

**Type:** Memory

Retrieves a remembered or known value.

**Examples:**
- `get username`
- `get last_login_time`
respond

**Type:** Action

Returns output to the user, system, or interface.

**Examples:**
- `respond "OK"`
- `respond data[query.id]`
show

**Type:** Action

Displays a message, alert, or visual element.

**Examples:**
- `show error "Invalid email"`
- `show success "Saved!"`
log

**Type:** Action

Appends a message or data to a log destination.

**Examples:**
- `log error to "errors.log"`
- `log event as activity`
retry

**Type:** Modifier

Attempts an action again after failure.

**Examples:**
- `retry once`
- `retry 3 times → else alert admin`
else

**Type:** Modifier

Fallback logic if previous condition fails.

**Examples:**
- `if error, retry once → else notify admin`
- `if not found, respond 404 → else show page`
then

**Type:** Modifier

Defines the next step in a chain clearly.

**Examples:**
- `verify token → then load user → then respond user`
- `save data → then notify team`
wait

**Type:** Modifier

Delays next action for a specified duration.

**Examples:**
- `wait 2 seconds → show message`
- `on start, wait 1s → load config`
as

**Type:** Modifier

Renames or aliases a value for reuse.

**Examples:**
- `remember token as session_id`
- `log event as user_action`
with

**Type:** Modifier

Provides additional arguments or configuration.

**Examples:**
- `respond "Missing ID" with status 400`
- `load config with defaults`
from

**Type:** Modifier

Indicates a source of data or reference.

**Examples:**
- `load settings from file`
- `get value from user input`
to

**Type:** Modifier

Indicates a destination of data or output.

**Examples:**
- `save file to storage`
- `log data to "logfile.txt"`
authorize

**Type:** System

Confirms a permission or trust level is met.

**Examples:**
- `authorize network access`
- `authorize file write`
purge

**Type:** System

Deletes or resets memory, cache, or state.

**Examples:**
- `purge session`
- `purge remembered users`
reset

**Type:** System

Resets part of the program or memory to defaults.

**Examples:**
- `reset config`
- `reset form on cancel`
stop

**Type:** Action

Halts current or future execution.

**Examples:**
- `if unauthorized, stop`
- `stop on failure`
continue

**Type:** Flow

Skips or resumes execution beyond current line.

**Examples:**
- `if valid, continue`
- `check user → if blocked, stop → else continue`
alert

**Type:** Action

Sends a high-priority notification or message.

**Examples:**
- `alert team on error`
- `alert admin with issue details`
check

**Type:** Action

Evaluates or verifies a condition actively.

**Examples:**
- `check token validity`
- `check server status`
verify

**Type:** Action

Confirms correctness or matching data.

**Examples:**
- `verify token`
- `verify email and password`
update

**Type:** Action

Replaces or modifies existing data.

**Examples:**
- `update user record`
- `update config with new values`
delete

**Type:** Action

Removes data or state from the system.

**Examples:**
- `delete session`
- `delete user from database`
create

**Type:** Action

Instantiates a new resource, object, or item.

**Examples:**
- `create new user`
- `create file with default content`
connect

**Type:** Action

Opens a connection to an external system or service.

**Examples:**
- `connect to server`
- `connect to database`
disconnect

**Type:** Action

Closes an existing connection.

**Examples:**
- `disconnect from server`
- `on shutdown, disconnect database`
use

**Type:** Flow

Temporarily applies a value or resource in scope.

**Examples:**
- `use test config`
- `use cached version if available`
define

**Type:** Memory

Declares a new named variable or configuration block.

**Examples:**
- `define default_timeout as 30`
- `define environment as "production"`
alias

**Type:** Memory

Creates an alternative name for a command or value.

**Examples:**
- `alias notify_team as alert staff`
- `alias daily_task as backup run`
capture

**Type:** Memory

Saves dynamic output to a variable for later use.

**Examples:**
- `capture result from verify token`
- `capture response as status_code`
test

**Type:** Action

Checks an operation in a non-destructive or debug mode.

**Examples:**
- `test email config`
- `test login with mock data`
print

**Type:** Action

Displays a debug or direct message to the screen.

**Examples:**
- `print "Hello, IDC"`
- `print current settings`
route

**Type:** Entity

Defines a handler or logic block for a given path or event.

**Examples:**
- `route "/status" respond "OK"`
- `route "/user" → if query.id exists → respond user[query.id]`