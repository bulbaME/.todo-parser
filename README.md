# .todo PARSER
a program which parses '.todo' files for better readability. could be used in small projects for easier planning and development.

# SYNTAX
syntax is a bit symilar to the task lists from .md

`[ ] <description>` - indicates the beginning of the task

`<tab>[ ]` - sub task 

`[x]` - completed task

`[/]` - half completed / in progress

`[?]` - in the process of planning

`[!]` - unactive task

# EXAMPLE
.todo file:
```
[ ] main task (active)
	[ ] sub task 1 (active)
	[x] sub task 2 (completed)
		[ ] sub sub task 1 (completed)
		[/] sub sub task 2 (completed)
		[?] sub sub task 3 (hidden)
	[/] sub task 3 (half completed)
	[?] sub task 4 (planning)
	[!] sub task 5 (hidden)
```
command line: `py parser.py <filename>`
