@echo off
cd /d "%~dp0"

rem Verify Command Extensions
2>nul verify/
setlocal enableextensions

if errorlevel 1 (
	echo ERROR: Command Processor Extensions are unavailable!
	echo.
	echo 	This batch file requires command extensions, but they seem to be unavailable on your system.
	echo.
	pause
	exit /b 1
)
if not cmdextversion 2 (
	echo ERROR: Command Processor Extensions are of version 1!
	echo.
	echo Command extensions seem to be available on your system, but only of version 1. This batch file was designed for version 2.
	echo.
	pause
	exit /b 1
)

rem Verify all tools
if not exist ".\tools\nvcompress.exe" (
	echo.
	echo 'nvcompress.exe' not found! Did you extract everything in the tools folder?
	echo.
	pause
	exit /b
)

if not exist ".\tools\nvtt.dll" (
	echo.
	echo 'nvtt.dll' not found! Did you extract everything in the tools folder?
	echo.
	pause
	exit /b
)

if not exist ".\tools\texconv.exe" (
	echo.
	echo 'texconv.exe' not found! Did you extract everything in the tools folder?
	echo.
	pause
	exit /b
)

if not exist ".\tools\DivinityMachine.exe" (
	echo.
	echo 'DivinityMachine.exe' not found! Did you extract everything in the tools folder?
	echo.
	pause
	exit /b
)

if not exist ".\tools\EternalTextureCompressor.exe" (
	echo.
	echo 'EternalTextureCompressor.exe' not found! Did you extract everything in the tools folder?
	echo.
	pause
	exit /b
)

if not "%~1" == "" goto StartLoop

rem Display help
echo.
echo Usage:
echo   "%~nx0" [texture1] [texture2] [...]
echo.
echo Alternatively, drag files onto this batch.
echo.
pause
exit /b

rem Convert files
:StartLoop
if "%~1" == "" goto Exit

echo.
echo|set /p="Converting '%~nx1'..."
echo.

rem Use nvcompress to convert the files into the correct type
for /f "tokens=1 delims=.$" %%a in ("%~nx1") do (set "stem=%%a") >nul

echo %~nx1 | findstr /i /c:"$bc7" >nul
if errorlevel 1 (
	if "%stem:~-2%"=="_n" (
		.\tools\nvcompress.exe -bc5 -fast "%~1" "%~1.tmp" >nul
	) else (
		if "%stem:~-7%"=="_Normal" (
			.\tools\nvcompress.exe -bc5 -fast "%~1" "%~1.tmp" >nul
		) else (
			.\tools\nvcompress.exe -bc1a -fast -srgb "%~1" "%~1.tmp" >nul
		)
	)
) else (
	copy /b /y "%~1" "%~1%~x1" >nul
	.\tools\texconv.exe -y -f BC7_UNORM -srgb "%~1%~x1" >nul
	move /y "%~1.dds" "%~1.tmp" >nul
)

rem Use DivinityMachine to convert the files into the game's BIM format
.\tools\DivinityMachine.exe "%~1.tmp" >nul
del "%~1.tmp" >nul

rem Get output's extension
for /f "tokens=1 delims=$" %%a in ("%~n1") do (set "stripped=%%a") >nul

if "%stripped:~-4%"==".png" (
	set "extension=png"
) else (
	set "extension=tga"
)

rem Use EternalTextureCompressor to compress the files using oodle
.\tools\EternalTextureCompressor.exe "%~1.%extension%" >nul

rem Rename the output file
echo %~nx1 | findstr /i /c:"$" >nul
if errorlevel 1 (
	move /y "%~1.%extension%" "%~dpn1.%extension%" >nul
) else (
	move /y "%~1.%extension%" "%~dpn1" >nul
)

rem Go to the next file
shift
goto StartLoop

:Exit
rem Exit
echo.
pause
exit /b
