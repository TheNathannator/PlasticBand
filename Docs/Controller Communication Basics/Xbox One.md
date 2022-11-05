# Communicating with Xbox One Controllers

## Windows

### XInput

Standard Xbox One controllers are usable via XInput. Unfortunately, guitars and drums are not. The only interface Windows recognizes from them is the one for navigation controllers, and as such they work for navigation in the Windows UI and whatnot, but not for anything else.

### Windows.Gaming.Input

The Universal Windows Platform `Windows.Gaming.Input` namespace also lets you use Xbox One controllers, among other things. However, WGI doesn't give you the exact raw data, as axis values are normalized to floats, and not all types of Xbox One controllers are supported by it.

There is also the `Windows.Gaming.Input.Custom` namespace, however there is no official documentation on how to use it, and all of my attempts to use it have failed so far.

### Packet Sniffing

Since all other options fail for reading Xbox One drums/guitars, some other method had to be devised.

Xbox One receivers, at their lowest-level infrastructure, are 802.11 networking devices. They are proprietary, but that doesn't prevent capturing packets through WinPcap/Npcap and USBPcap (note: at the time of writing, Npcap doesn't seem to pick up on Xbox One receivers). These packets can be interpreted, and all data sent from connected devices can be read.

Packet sniffing isn't the most elegant solution since it requires a number of dependencies to be installed on the user's PC. It would be nice if there was a more direct way to get this data...

### Directly via the Driver

And indeed there is! Once the time was put into it, it wasn't difficult to work out what exactly `Windows.Gaming.Input` was doing to interact with Xbox One devices.

A full writeup on the driver interface may be found [here](https://gist.github.com/TheNathannator/bcebc77e653f71e77634144940871596). This information was determined through [detours and DLL injection](https://github.com/nefarius/XInputHooker), alongside a bit of reverse-engineering and existing knowledge of the Xbox One protocol.

In summary:

```cpp
#include <Windows.h>

// The Xbox One driver interface lives at a device file with a path of "\\.\XboxGIP".
// The first thing to do is open that file. This file does not exist if no Xbox One controllers are
// currently connected to the system, so you'll need to be prepared to handle that.
HANDLE hFile = CreateFileW(L"\\\\.\\XboxGIP", GENERIC_READ | GENERIC_WRITE, FILE_SHARE_READ | FILE_SHARE_WRITE, nullptr, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, nullptr);

// Once this file is opened, you need to send an IOCTL transfer to it to have it
// re-enumerate all currently-connected devices to you.
#define GIP_ADD_REENUMERATE_CALLER_CONTEXT CTL_CODE(0x4000, 0x734, METHOD_BUFFERED, FILE_ANY_ACCESS) // 0x40001CD0
DeviceIoControl(hFile, GIP_ADD_REENUMERATE_CALLER_CONTEXT, nullptr, 0, nullptr, 0, nullptr, nullptr);

// After that, you use ReadFile to receive messages.
// You'll want to prepare a large buffer (500 bytes at minimum, 1000+ bytes recommended)
// or be prepared to receive messages in multiple passes, as one of the messages sent
// when a device is connected/re-enumerated can be several hundred bytes large.
BYTE readBuffer[500];
DWORD bytesRead;
while (ReadFile(hFile, &readBuffer, sizeof(readBuffer), &bytesRead, nullptr))
{
    // Process messages here.
    // Messages are prefixed with a header that contains information such as an ID for the device
    // the message is for, a command ID for the message, and a message length (not including the header).

    // Some important command IDs include:
    // - 0x04: Descriptor for the device. Includes GUIDs that indicate which interfaces a device supports.
    // - 0x20: Input data from the device. Unlike XInput, the format varies per device type.
}

// To send messages, use WriteFile.
// Sent messages must start with the same header that received messages do.
BYTE sendBuffer[] { ... };
WriteFile(hFile, &writeBuffer, sizeof(writeBuffer), nullptr, nullptr)
```

A note of warning: this interface does not send input data if your app doesn't have a non-console window. It also does not send input while the window is unfocused. This is a limitation imposed by the driver, and while there are potentially ways around it, more research is needed to determine how it's done.

This limitation means that only direct users of Xbox One devices should use this interface. Remapper programs and the like still have to use the packet sniffing method.

## Mac

As with the Xbox 360 controllers, on the latest versions of macOS, there is no way to use Xbox One controllers that cannot connect through Bluetooth. The [360Controller driver](https://github.com/360Controller/360Controller) enabled this, but this driver no longer works.

## Linux

Linux natively supports wired Xbox One controllers through the built-in xpad driver. For wireless controllers, and other controllers not natively supported, there are these alternatives:

- [xboxdrv](https://gitlab.com/xboxdrv/xboxdrv)
- a modified [xpad](https://github.com/paroj/xpad)
- [xone](https://github.com/medusalix/xone)
- [xow](https://github.com/medusalix/xow)
  - Only listed for completeness: `xone` is the successor to this and is recommended over `xow`.
- [xpadneo](https://github.com/atar-axis/xpadneo/)

(TODO: how to identify and interface with devices)
