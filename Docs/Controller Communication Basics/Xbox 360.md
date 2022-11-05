# Communicating with Xbox 360 Controllers

## Windows

### XInput

On Windows, Xbox 360 controllers are typically used by programs through the XInput API.

Windows 8 and onward include XInput with the system as `xinput1_4.dll`, along with `xinput9_1_0.dll` for backwards compatibility. Windows 7 includes it as just `xinput9_1_0.dll`. It is also available in the DirectX SDK as `xinput1_3.dll`, `xinput1_2.dll`, and `xinput1_1.dll`.

An example of using XInput:

```cpp
#include <Xinput.h>

for (int userIndex = 0; userIndex < XUSER_MAX_COUNT; userIndex++)
{
    XINPUT_STATE state;
    if (XInputGetState(userIndex, &state) != ERROR_SUCCESS)
    {
        // Processing of a disconnected controller goes here
        continue;
    }

    // Input processing code goes here
}
```

XInput 1.4 and 1.3 include a number of hidden exports which provide additional functionality. These exported functions can be called by loading the respective XInput library and getting the address for the export by ordinal:

```cpp
#include <Windows.h>
#include <Xinput.h>

// This function is the exact same as XInputGetState, except it doesn't mask off the guide button in the state.
// It's present at ordinal #100.
typedef DWORD (WINAPI* XINPUT_GET_STATE_EX)(DWORD userIndex, XINPUT_STATE* pState);
#define XINPUT_GET_STATE_EX_ORDINAL (LPSTR)100

HMODULE hXInput = LoadLibrary(TEXT("xinput1_4.dll"));
if (hXInput)
{
    XINPUT_GET_STATE_EX pfnXInputGetStateEx = (XINPUT_GET_STATE_EX)GetProcAddress(hXInput, XINPUT_GET_STATE_EX_ORDINAL);
    if (pfnXInputGetStateEx)
    {
        XINPUT_STATE state;
        if (pfnXInputGetStateEx(userIndex, &state) != ERROR_SUCCESS)
        {
            // ...
        }
    }
}
```

For the other hidden exports, see [OpenXInput](https://github.com/Nemirtingas/OpenXinput)'s header and .def file.

### Other Options

Other options for using XInput include the [OpenXInput](https://github.com/Nemirtingas/OpenXinput) project, which allows using more than 4 controllers and has additional functions.

There is also my own [SharpXusb](https://github.com/TheNathannator/SharpXusb) library for .NET that is more focused on direct communication with the XUSB driver, which allows for getting data beyond what standard XInput reports (modifying OpenXInput can also provide this functionality).

## Mac

On the latest versions of macOS, there is currently no way to use Xbox 360 controllers. Previously, there was an [open-source Xbox 360 controller driver](https://github.com/360Controller/360Controller) that enabled using it via HID, but this driver no longer works due to kernel extensions being deprecated and unsupported since macOS Big Sur. The maintainers of that driver have no plans to update it unfortunately.

## Linux

On Linux, Xbox 360 controller support is built directly into the kernel. There are also alternative drivers which provide additional functionality and may work better with some controllers:

- [xboxdrv](https://gitlab.com/xboxdrv/xboxdrv)
- a modified [xpad](https://github.com/paroj/xpad)

(TODO: how to identify and interface with devices)
