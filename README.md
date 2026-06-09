# Wingetmore

Wingetmore is a winget wrapper made mainly to speed up updates\
However it's possible to run multiple arbitrary winget commands concurently using it

## Help available in the program
```
How to use this program:
wgm install program1 program2 program3 uninstall ... upgrade ... search ... other "full command"

Arguments:
	install
	uninstall
	upgrade (handles --all)
	search
	other

An example:
wgm upgrade --all search Microsoft. Google. Github other "show Brave.Brave.Beta --versions" search Azure
```