#!/usr/bin/env fish
# Builds the latest intelliED, installs it to /Applications and opens it.
# A running intelliED is quit first (it prompts to save unsaved files), since
# the single-instance lock would otherwise just bring the old version forward.

set -l root (path dirname (status filename))
cd $root; or exit 1

./release.sh; or exit 1

if pgrep -xq intelliED
    echo "Quitting the running intelliED"
    osascript -e 'quit app "intelliED"'
    while pgrep -xq intelliED
        sleep 0.2
    end
end

./install.sh; or exit 1
open -a /Applications/intelliED.app $argv
