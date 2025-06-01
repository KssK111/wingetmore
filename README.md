### An upgrade to winget, mainly for faster download (install/upgrade) times
### It uses async code to do operations concurently, which speeds up downloads if their speed is limited by the servers you're downloading from and not your internet speed

- Use wgm help or /help or --help or don't use any arguments to get this message
- How to use this program:
- wgm install program1 program2 program3 uninstall ... upgrade ... search ... other "full command"
- 
- Arguments:
- 	install
- 	uninstall
- 	upgrade (handles --all)
- 	search
- 	other
- 
- An example:
- wgm upgrade --all search Microsoft. Google. Github other "show Brave.Brave.Beta --versions" search Azure
