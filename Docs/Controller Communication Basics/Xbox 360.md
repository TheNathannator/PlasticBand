# Communicating with Xbox 360 Controllers

## Windows

### XInput

On Windows, Xbox 360 controllers are typically used by programs through the XInput API.

Windows 8 and onward include XInput with the system as `xinput1_4.dll`, along with `xinput9_1_0.dll` for backwards compatibility. Windows 7 includes it as just `xinput9_1_0.dll`. It is/was also available in the DirectX SDK as `xinput1_3.dll`, `xinput1_2.dll`, and `xinput1_1.dll`.

An example of using XInput:

```cpp
#include <Windows.h>
#include <Xinput.h>

for (DWORD userIndex = 0; userIndex < XUSER_MAX_COUNT; userIndex++)
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

XInput 1.4 and 1.3 include a number of hidden exports which provide additional functionality. These exported functions can be called by loading the respective XInput library and getting the address for the export by ordinal. For a full list, see [OpenXInput](https://github.com/Nemirtingas/OpenXinput)'s header and .def file (be mindful of the other ordinals it's added).

```cpp
#include <Windows.h>
#include <Xinput.h>

// This function is the exact same as XInputGetState, except it doesn't mask off the guide button.
// It's present at ordinal #100.
typedef DWORD (WINAPI *XINPUT_GET_STATE_EX)(DWORD userIndex, XINPUT_STATE* pState);
#define XINPUT_GET_STATE_EX_ORDINAL (LPSTR)100

HMODULE hXInput = LoadLibrary(TEXT("xinput1_4.dll"));
if (!hXInput)
    return;

XINPUT_GET_STATE_EX pfnXInputGetStateEx = (XINPUT_GET_STATE_EX)GetProcAddress(hXInput, XINPUT_GET_STATE_EX_ORDINAL);
if (!pfnXInputGetStateEx)
    return;

// Repeat with any other functions that should be retrieved
```

### Windows.Gaming.Input

The Universal Windows Platform `Windows.Gaming.Input` namespace also lets you use Xbox 360 controllers, among other things. However, WGI doesn't just hand you the input data as-is, and not all Xbox 360 subtypes are supported by it (including instruments).

There is also the `Windows.Gaming.Input.Custom` namespace, however its Xbox 360 support shuffles around subtype values and doesn't expose the raw value, so it cannot be used for undocumented subtypes.

### Other Options

XInput works just fine for getting input data from an Xbox 360 controller: it does little to no processing on it. But it has some limitations, functionality it doesn't allow, and data that it doesn't expose, which requires alternatives to work around.

- The [OpenXInput](https://github.com/Nemirtingas/OpenXinput) project is a reverse-engineering/re-implementation of XInput, which allows using more than 4 controllers at once, and has some additional functionality.
- My own [SharpXusb](https://github.com/TheNathannator/SharpXusb) library for .NET interfaces directly with the driver, which allows for full support of certain instrument types that can't otherwise be supported.

## Mac

Xbox 360 controllers are not supported natively on macOS. The [360Controller driver](https://github.com/360Controller/360Controller) enables this, however it is currently unmaintained and may stop working at any point. On macOS 11 (Big Sur) and onwards, it also takes a little more trouble to enable, and does not have an official Apple Silicon build.

## Linux

On Linux, Xbox 360 controller support is built directly into the kernel. There are also alternative drivers which provide additional functionality and may work better with some controllers:

- [xboxdrv](https://gitlab.com/xboxdrv/xboxdrv)
- a modified [xpad](https://github.com/paroj/xpad)
- [xpad-noone](https://github.com/medusalix/xpad-noone) (required if using [xone](https://github.com/medusalix/xone))

These drivers generally just register everything as standard `js`/`evdev` devices, and don't expose any raw data from the controller. Works fine for general purposes, not so much if you require that raw data.

## The Shotgun Method (Direct USB Communication)

As a final general method, you can ignore any potential existing solutions and implement everything yourself using a raw USB transport such as libusb. On Windows this is the most invasive method, as most libusb drivers completely override the original driver and make the device unusable in other programs. It's the most complicated method out of everything, as you have to implement the entire protocol handling yourself. This route is advanced and not recommended, and there are a number of things that can go wrong if you're not careful.

As of September 16, 2024, the Xbox 360 controller protocol (known officially as the Gaming Input Protocol, or GIPUSB) [has been published as a Microsoft open specification](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-xusbi/c79474e7-3968-43d1-8d2f-175d47bef43e). Refer to the documentation there for guidance on implementing the protocol.
