# Learning SED
The sed utility reads the specified files, or the standard input if no files
are specified, modifying the input as specified by a list of commands. <br>

The best way to learn it is to read `man sed` and `man re_format(7)`<br>

MacOS sed works slightly different with respect to, labels, branching, and
the -i command. On Mac, one can only branch backwards, and -i <b>requires</b>
a suffix. An empty string `-i '' -e '...'` will do.<br>

read [this](https://unix.stackexchange.com/questions/13711/differences-between-sed-on-mac-osx-and-other-standard-sed) <br>

### Some terminology to know:
Pattern Space, Hold Space, Segment, Cycle, Branch <br>

### Common Commands:
The most common command by far is `s///g`<br>

|Command | Description |
|---|---|
|`[addr]q` | quit after [addr]|
|`d` | delete the pattern space; immediately start the next cycle|
|`-n [addr]p` | print out the [1addr] line|
|`n;` or `[addr(A)]~[arrd(B)]` | skip n or from [addr(A)] to [addr(B)]|
|`{ commands }` | sequences commands|
|`y/source-chars/dest-chars/` | Transliterate any characters in the pattern space whi|match any of the source-chars with the corresponding character in dest-chars. |
|`a\\\nfoo` | adds 'foo' AFTER finding a \n (similar to a in vim)|
|`i\\\nfoo` | add 'foo' to the beginning of the line (similar to i in vim)|
|`[addr(A)],[addr(B)]c\\\nfoo` | cuts from [addr(A)] to [addr(B)] and replaces wi|'foo'|

On mac os `a\` `i\` `c\` <b>must</b> be a an ANSI-C string $'i\\\n', or the \n will be read literaly.<br>

### Similar Tools:
sed can be used on its own, but it is often used in combination with
other Unix utilities such as `cat`, `expr`, `head`, `seq`,
`sort`, `tail`, `tr`, and `uniq`. <br>
`wc`, `cut`, `paste`, `join`. <br>

# Learning AWK
pattern-directed scanning and processing language <br>

Best place to learn is to read the book written by the authors.<br>
[here](https://github.com/tpn/pdfs/blob/master/The%20AWK%20Programming%20Language%20(1988).pdf) <br>

# Learning GREP
Literally `man grep` is all you need.<br>
It's an easy to learn, and mega powerful program.<br>
If you want Pearl compatible REs - `brew install grep --with-default-names`<br>

### Similar Tools:
ack, ag, rg (ripgrep), fzf<br>


# Learn VIM
Start with the vim tutor. Do it a few times. Learn how to use :help. <br>
Vim is a matter of doing it, over and over, and one will find that, although
vim takes some months to "get," vim is top two most powerful text
manipulation tools of all time. <br>

Basic commands one needs to know to use vim at all are left out. <br>
For example:<br>

`:help search-commands` <br>
`:help object-motions` <br>
`:help text-objects` <br>
`:help jumplist` <br>
`:help changelist` <br>
`:help mark` <br>
`:help g` <br>
`:help mapleader` <br>
`:help abbreviations` <br>

Yeah, tags do kinda suck in vim <br>
In terms of marks, tend to use a-z for within a base file, use A-Z to move
to other files.<br>

### Damn near mandatory plugins:
Know how to use each:<br>
vinegar - Project tray / file explorer <br>
surround - Easy mappings for changing quotes, brackets, and braces <br>
polyglot - Tiny language server <br>
coc - Auto completion <br>
a theme - Gruvbox baby! <br>

### Vimscripting:
[Learn Vimscrip The Hard Way]( https://learnvimscriptthehardway.stevelosh.com/ )

