# Learning SED

The sed utility reads the specified files, or the standard input if no files
are specified, modifying the input as specified by a list of commands. <br>
<br>

The best way to learn it is to read `man sed` and `man re_format(7)`<br>
<br>

MacOS sed works slightly different with respect to, labels, branching, and
the -i command. On Mac, one can only branch backwards, and -i <b>requires</b>
a suffix. An empty string `-i '' -e '...'` will do.<br>

read [this](https://unix.stackexchange.com/questions/13711/differences-between-sed-on-mac-osx-and-other-standard-sed) <br>
<br>

### Some terminology to know:
Pattern Space, Hold Space, Segment, Cycle, Branch <br>

### Common Commands:
The most common command by far is `s///g`<br>
<br>

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
<br>

On mac os `a\` `i\` `c\` <b>must</b> be a an ANSI-C string $'i\\\n', or the \n will be read literaly.<br>
<br>

### Similar Tools:

sed can be used on its own, but it is often used in combination with
other Unix utilities such as `cat`, `expr`, `head`, `seq`,
`sort`, `tail`, `tr`, and `uniq`. <br>
`wc`, `cut`, `paste`, `join`. <br>
<br>

# Learning AWK

# Learning GREP

### Similar Tools:

ack, ag, rg (ripgrep)

# Learning FZF


# Learn VIM

Start with the vim tutor. Do it a few time. Learn how to use :help. <br>
Vim is a matter of doing it, over and over, and one will find that, although
it vim takes some months to really "get," vim is top two most powerful text
manipulation tools. <br>

### Damn near mandatory plugins:

Know how to use each:<br>
vinegar <br>
surround <br>
polyglot <br>
coc <br>
a theme <br>