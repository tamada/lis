# Where should we write the GIF?
Output demo.gif

Set FontFamily "JetBrainsMono Nerd Font"
Set FontSize 20
Set Width 1200
Set Height 600

# Type a command in the terminal.
Type "# This is the demo animation of lis command."
Sleep 200ms
Enter

Type "# lis is minimal and alternative ls implementation written in Rust."
Sleep 200ms
Enter

Type "# Print the entries of the current directory."
Sleep 200ms
Enter
Type "lis"
Sleep 300ms
Enter
Sleep 2000ms

Type "# Enable icons, sort by name (default), time, size, and extension."
Sleep 200ms
Enter
Type "lis --icon --sort name"
Sleep 500ms
Backspace 4
Type "size"
Sleep 500ms
Backspace 4
Type "extension"
Sleep 500ms
Enter
Sleep 2000ms

Type "# Print the details of each entry (includes git status)."
Sleep 200ms
Enter
Type "lis --long"
Sleep 500ms
Enter
Sleep 2000ms

Type "# Support json, csv, and yaml."
Sleep 200ms
Enter
Type "lis --format plain"
Sleep 300ms
Backspace 5
Type "csv"
Sleep 300ms
Backspace 3
Type "json"
Sleep 300ms
Enter
Sleep 1000ms

Type "# Since the execution results were flowed,"
Sleep 300ms
Enter
Type "# I'll show only the first entry."
Sleep 300ms
Enter
Type "lis --format json | jq '.[0]'"
Sleep 1000ms
Enter
Sleep 5000ms
