cd /d %~dp0
cd wasm/pkg
START /B /WAIT cmd /c npm link

cd /d %~dp0
cd web
START /B /WAIT cmd /c npm link loop_puzzle_web

cd /d %~dp0
