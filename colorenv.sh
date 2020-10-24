#! /bin/bash

# For whaever reason the "exit;" command doesn't exit the virtual env.
# manually exiting and then re-entering will color the ".venv" text
sed -i .txt "56s/.*/            printf \"%s%s\"\"[\"(set_color --dim brcyan)\".venv\"(set_color normal)\"]\"/" .venv/bin/activate.fish;
