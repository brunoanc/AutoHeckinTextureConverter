#include <iostream>
#include "utils.hpp"

#ifdef _WIN32
#define WIN32_LEAN_AND_MEAN
#include <Windows.h>
#include <conio.h>

// Check if process is running on a terminal by checking the console processes
inline bool isRunningOnTerminal()
{
    DWORD buffer[1];
    return GetConsoleProcessList(buffer, 1) > 1;
}
#endif

// Display the 'press any key to exit' if process is not running in a terminal (Windows only)
void pressAnyKey()
{
#ifdef _WIN32
    if (isRunningOnTerminal())
        return;

    std::cout << "Press any key to exit..." << std::endl;
    _getch();
#endif
}

// Check if string ends with substring
bool endsWith(const std::string& fullString, const std::string& suffix)
{
    if (fullString.length() >= suffix.length()) {
        return 0 == fullString.compare(fullString.length() - suffix.length(), suffix.length(), suffix);
    }
    else {
        return false;
    }
}
