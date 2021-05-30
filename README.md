# .todo PARSER
a program which parses '.todo' files for better readability. could be used in small projects for easier planning and development.

# SYNTAX
syntax is a bit symilar to the task lists from .md

`[ ] <description>` - indicates the beginning of the task

`<tab>[ ]` - sub task 

`[x]` - completed task

`[/]` - half completed / in progress

`[?]` - in the process of planning

`[!]` - inactive task

# RULES
while parsing some sub tasks could be transformed to another state:

1. when `[!]` is parent:

`[ ]`, `[x]`, `[/]` and `[?]` -> `[!]`

2. when `[x]` is parent:

`[ ]` and `[?]` -> `[!]`

3. when `[ ]` and `[/]` are parents nothing happens	

# EXAMPLE
.todo file:
```
[/] main task (half complited)
	[ ] sub task 1 (active)
		[?] sub sub task 1 (planning)
	[x] sub task 2 (completed)
		[ ] sub sub task 1 (inactive)
		[/] sub sub task 2 (completed)
		[?] sub sub task 3 (inactive)
	[x] sub task 3 (active)
	[?] sub task 4 (planning)
		[x] sub sub task 1 (planning)
		[!] sub sub task 2 (inactive)
	[!] sub task 5 (inactive)
```
command line: `py parser.py <filename>`

flags: 

`-i` display inactive tasks

`-p` display planning tasks
