# Communicating with Xbox One Controllers

## Windows

### XInput

Standard Xbox One controllers are usable via XInput. Unfortunately, Xbox One guitars and drums are not. The only interface Windows recognizes from them is the one for navigation controllers, and as such they work for navigation in the Windows UI and UWP apps, but not for anything else.

### Windows.Gaming.Input

The UWP `Windows.Gaming.Input` namespace also lets you use Xbox One controllers, among other things. However, it doesn't give you the raw data, and not all types of Xbox One controllers are supported by it. To get around this, you want to use `Windows.Gaming.Input.Custom` to create your own custom device classes. There's a catch though: one of the interfaces needed to use it is undocumented, so using it isn't very straight-forward at first. A demonstration for this may be found [here](https://github.com/TheNathannator/WGIC).

### Packet Sniffing (Wireless)

Xbox One receivers, at their lowest level, are 5GHz 802.11 networking devices. They are proprietary, but Windows still exposes them as network devices, so they can be captured through WinPcap/Npcap (note: at the time of writing, Npcap doesn't seem to pick up Xbox One receivers). These packets can then be interpreted, and all data sent from connected devices can be read.

The captured data starts with a standard 802.11 data header. The only packets of interest are those with a type of Data (2) and a subtype of QoS Data (8), other traffic types are not protocol/input data and should be ignored. The receiver address (address 1) in the header is the address of the Xbox One receiver. The transmitter address is the address of the controller (in reverse-endian to how the protocol reports it), and the destination address is usually that of the receiver but in some cases it will be slightly different (e.g. where the first byte of the address would normally be `0x62`, it will instead be `0x60`).

After the 802.11 header comes the protocol data, a reference for which may be found here [here](https://gist.github.com/TheNathannator/c5d3b41a12db739b7ffc3d8d1a87c60a).

Packet sniffing isn't the most elegant solution since it requires additional dependencies to be installed on the user's PC. However, it is currently the only way to get data from wireless devices directly without running into certain limitations imposed by the Xbox One driver (though they really only matter for things such as remapper programs though).

### Directly from the Driver

The Xbox One driver itself exposes an interface which may be used to get similar data to that which you would get through packet sniffing. Certain semantics of the protocol are already handled by the time the driver hands over the data to these programs, so it is arguably the easiest method of interacting with Xbox One devices directly to implement. A full writeup on the driver interface may be found [here](https://gist.github.com/TheNathannator/bcebc77e653f71e77634144940871596).

This interface has certain limitations which can make it unusable in some scenarios. Certain types of data are not sent if your app doesn't have a focused, non-console window, notably input data. These limitations also apply to `Windows.Gaming.Input`, as this interface is what it uses under the hood for Xbox One controllers.

## Mac

As with Xbox 360 controllers, Xbox One controllers are not natively supported on macOS (with the exception of gamepads that support Bluetooth, as they just use HID). The [360Controller driver](https://github.com/360Controller/360Controller) enables this, however it is currently unmaintained and may stop working at any point. On macOS 11 (Big Sur) and onwards, it also takes a little more trouble to enable, and does not have an official Apple Silicon build.

This driver does not support the Xbox One receiver, and does not support Xbox One guitars/drumkits.

## Linux

Linux natively supports wired Xbox One gamepads through the built-in xpad driver. For wireless controllers, and other controller types, there are alternatives:

- [xone](https://github.com/medusalix/xone)
  - Primary source for protocol info, and is the only driver to support Xbox One guitars to my knowledge.
- [xboxdrv](https://gitlab.com/xboxdrv/xboxdrv)
- a modified [xpad](https://github.com/paroj/xpad)
- [xpadneo](https://github.com/atar-axis/xpadneo/)

These drivers generally just register everything as standard `js`/`evdev` devices, and don't expose any raw data from the controller. Works fine for general purposes, not so much if you require that raw data.

## The Shotgun Method (Direct USB Communication)

Of course, on all of these platforms you can just completely ignore any potential existing solutions and implement everything yourself using libusb. On Windows this is the most invasive method, as most libusb drivers completely override the original driver and make the device unusable in other programs (however for remapper programs and the like this would be acceptable, as the entire point of the program is to re-interpret the data for other programs). It's the most complicated method out of everything, as you have to implement the entire protocol handling yourself. This route is advanced and not recommended, as there are a number of things that can go wrong if you're not careful.

A reference for the Xbox One controller protocol may be found here [here](https://gist.github.com/TheNathannator/c5d3b41a12db739b7ffc3d8d1a87c60a).
