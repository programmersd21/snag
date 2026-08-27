Output demo.gif

Set Shell "bash"
Set FontSize 18
Set FontFamily "JetBrainsMono Nerd Font Mono"
Set Width 1200
Set Height 650
Set Padding 30
Set Margin 60
Set MarginFill "#000000"
Set WindowBar Colorful
Set WindowBarSize 40
Set BorderRadius 16
Set Theme { "name": "PitchBlack", "black": "#0a0a0a", "red": "#f38ba8", "green": "#a6e3a1", "yellow": "#f9e2af", "blue": "#89b4fa", "magenta": "#cba6f7", "cyan": "#94e2d5", "white": "#cdd6f4", "brightBlack": "#585b70", "brightRed": "#f38ba8", "brightGreen": "#a6e3a1", "brightYellow": "#f9e2af", "brightBlue": "#89b4fa", "brightMagenta": "#cba6f7", "brightCyan": "#94e2d5", "brightWhite": "#ffffff", "background": "#000000", "foreground": "#cdd6f4", "selectionBackground": "#313244", "cursor": "#cba6f7" }

Hide
Type "export PATH=$PWD/target/release:$PATH"
Enter
Type "clear"
Enter
Show

Sleep 600ms

Type "snag --help"
Enter
Sleep 3500ms

Type "clear"
Enter
Sleep 800ms

Type "# extract ip addresses"
Enter
Sleep 400ms
Type "echo 'request from 192.168.1.50 via 10.0.0.1' | snag ip"
Enter
Sleep 1400ms

Type "# extract sha256 checksums"
Enter
Sleep 400ms
Type "echo 'checksum: a3f2c8e9d1b4f7e6a9c2d5b8f1e4a7c6d9b2e5f8a1c4d7e0b3f6a9c2e5d8f1b4' | snag hash"
Enter
Sleep 1400ms

Type "# slice columns instantly"
Enter
Sleep 400ms
Type "echo 'admin:x:1000:1000' | snag :1"
Enter
Sleep 1400ms

Type "# parse json streams"
Enter
Sleep 400ms
Type `echo '{"status": "success", "code": 200}' | snag .status`
Enter
Sleep 1400ms

Type "# wildcard capture"
Enter
Sleep 400ms
Type `echo 'cpu=[87%] mem=[4.2GB]' | snag 'cpu=[{}]'`
Enter
Sleep 2800ms
