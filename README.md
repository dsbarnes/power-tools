# Learning SED
The sed utility reads the specified files, or the standard input if no files
are specified, modifying the input as specified by a list of commands. <br>
The best way to learn it is to read `man sed` and `man re_format(7)`
<br>
<br>
MacOS sed works slightly different with respect to, labels, branching, and
the -i command. On Mac, one can only branch backwards, and -i <b>requires</b>
a suffix. An empty string `-i '' -e '...'` will do.
read (this)[https://unix.stackexchange.com/questions/13711/differences-between-sed-on-mac-osx-and-other-standard-sed]
<br>
<br>
### Some terminology to know:
Pattern Space, Hold Space, Segment, Cycle<br>

### Common Commands:
The most common command by far is `s///g`<br>
<br>
`[1addr]q` | quit after [1addr]<br>
`d` | delete the pattern space; immediately start the next cycle<br>
`-n [1addr]p` | print out the [1addr] line<br>
`n;` or `[1addr(A)]~[1arrd(B)]` | skip n or from [addr(A)] to [addr(B)]<br>
`{ commands }` | sequences commands<br>
`y/source-chars/dest-chars/` | Transliterate any characters in the pattern space which match any of the source-chars with the corresponding character in dest-chars. <br>
`a\\\nfoo` | adds 'foo' AFTER finding a \n (similar to a in vim)<br>
`i\\\nfoo` | add 'foo' to the beginning of the line (similar to i in vim)<br>
`[addr(A)],[addr(B)]c\\\nfoo` | cuts from [addr(A)] to [addr(B)] and replaces with 'foo'<br>

### Similar Tools:
sed can be used on its own, but it is often used in combination with
other Unix utilities such as `cat`, `expr`, `head`, `seq`,
`sort`, `tail`, `tr`, and `uniq`. <br>
`wc`, `cut`, `paste`, `join`


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

