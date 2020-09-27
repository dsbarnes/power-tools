# This is somewhat pointless to do with a script,
# make it an alias by adding to bashrc

# AWK
alias swap-shop="awk '{\
    if(/^# /) print substr(\$0, 3);\
    else print \"#\" \$0;\
    }' ~/Documents/power-tools/comments.yaml\
    > asdfasdf.yaml"
