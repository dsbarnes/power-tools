# Learning SED
The sed utility reads the specified files, or the standard input if no files
are specified, modifying the input as specified by a list of commands. <br>

The best way to learn it is to read `man sed` and `man re_format(7)`<br>

MacOS sed works slightly different with respect to, labels, branching, and
the -i command. On Mac, one can only branch backwards, and -i <b>requires</b>
a suffix. An empty string `-i '' -e '...'` will do.<br>

read [this](https://unix.stackexchange.com/questions/13711/differences-between-sed-on-mac-osx-and-other-standard-sed) <br>

### Some terminology to know:
Pattern Space,<br>
Hold Space,<br>
Segment,<br>
Cycle,<br>
Branch<br>

### Common Commands:
The most common command by far is `s///g`<br>

| Command                       | Description                                |
| ---                           | ---                                        |
| `[addr]q`                     | quit after [addr]
| `d`                           | delete the pattern space; immediately start the next cycle
| `-n [addr]p`                  | print out the [addr] line
| `n;` or `[addr(A)]~[arrd(B)]` | skip n or from [addr(A)] to [addr(B)]
| `{ commands }`                | sequences commands
| `y/source-chars/dest-chars/`  | Translate any of the source-chars with the corresponding character in dest-chars.
| `a\\\nfoo`                    | adds 'foo' AFTER finding a \n (similar to a in vim)
| `i\\\nfoo`                    | add 'foo' to the beginning of the line (similar to i in vim)
| `[addr(A)],[addr(B)]c\\\nfoo` | cuts from [addr(A)] to [addr(B)] and replaces with 'foo'

On mac os `a\` `i\` `c\` <b>must</b> be a an ANSI-C string $'i\\\n', or the \n will be read literaly.<br>

### Similar Tools:
`wc`, `cut`, `paste`, `join`. <br>

sed can be used on its own, but it is often used in combination with
other Unix utilities such as `cat`, `expr`, `head`, `seq`, `sort`, `tail`, `tr`, and `uniq`. <br>

# Learning AWK
pattern-directed scanning and processing language <br>

Best place to learn is to read the book written by the authors.<br>
[here](https://github.com/tpn/pdfs/blob/master/The%20AWK%20Programming%20Language%20(1988).pdf) <br>

TODO: A short awk cheat sheet like I have with sed

# Learning GREP
`man grep` is all one need.<br>
If you want Pearl compatible REs - `brew install grep --with-default-names`<br>

### Similar Tools:
ack, ag, rg (ripgrep), fzf<br>


# Learn VIM
Start with the vim tutor. Do it a few times. Learn how to use :help. <br>
Vim is a matter of doing it, over and over, and one will find that, although
vim takes some months to "get," vim is top two most powerful text
manipulation tools of all time. <br>

Help sections to read:<br>
`:help search-commands` <br>
`:help substitute` <br>
`:help object-motions` <br>
`:help text-objects` <br>
`:help jumplist` <br>
`:help changelist` <br>
`:help mark` <br>
`:help g` <br>
`:help mapleader` <br>
`:help abbreviations` <br>
`:help :Tabularize` after install will bring one to the 'walkthrough' <br>
Probably no one ever reads `:help netrw` but for sure know it exits <br>

Yeah, tags do kinda suck in vim ('cept for C programs) <br>
use `gd` to "goto definition" and yes, `<C-o>` will jump back. <br>
<br>

In terms of marks, tend to use a-z for within a base file, use A-Z to move
to other files.<br>

### Near mandatory plugins:
Know how to use each:<br>
vinegar - Project tray / file explorer <br>
surround - Easy mappings for changing quotes, brackets, and braces <br>
tabular - makes tables
polyglot - Tiny language server <br>
coc - Auto completion <br>
a theme - Gruvbox baby! <br>
I don't think of fugitive as necessary, bc git is fine on the command line.<br>

### Vimscripting:
[Learn Vimscrip The Hard Way]( https://learnvimscriptthehardway.stevelosh.com/ )<br>
This book isn't in the ideal order but is well worth referencing.<br>

`:res` works for horizontal resizing, but for some funky reason vres has
to be spelled out `vertical res +-N`, which is a silly oversight imo. <br>

That would be a good start learning how to write functions in vimscript.
A `:echo` or `:!cat` command for plugin key mappings can be done quite pithily as well<br>

