# Where the game is installed on Windows, for the scripts that build and run
# it from WSL (sourced, not run): UNIVERSE_HOME if set, or `universe-game`
# in the Windows user's own folder (C:\Users\<you>\universe-game). The
# client goes in its `client` folder: TARGET.
if [ -z "${UNIVERSE_HOME:-}" ]; then
    windows_home=$(cd /mnt/c 2>/dev/null && cmd.exe /c 'echo %USERPROFILE%' 2>/dev/null | tr -d '\r')
    [ -n "$windows_home" ] || { echo "Can't find your Windows folder: set UNIVERSE_HOME."; exit 1; }
    UNIVERSE_HOME="$(wslpath "$windows_home")/universe-game"
fi
TARGET="$UNIVERSE_HOME/client"
