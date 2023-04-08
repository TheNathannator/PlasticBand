# Communicating with Xbox One Controllers

## Windows

### XInput

Standard Xbox One controllers are usable via XInput. Unfortunately, Xbox One guitars and drums are not. The only interface Windows recognizes from them is the one for navigation controllers, and as such they work for navigation in the Windows UI and UWP apps, but not for anything else.

### Windows.Gaming.Input

The UWP `Windows.Gaming.Input` namespace also lets you use Xbox One controllers, among other things. However, it doesn't give you the raw data, and not all types of Xbox One controllers are supported by it. To get around this, you want to use `Windows.Gaming.Input.Custom` to create your own custom device classes. There's a catch though: one of the interfaces needed to use it is undocumented, so using it isn't very straight-forward at first. A demonstration for this may be found [here](https://github.com/TheNathannator/WGIC).

### Packet Sniffing

Xbox One receivers, at their lowest-level infrastructure, are 5GHz 802.11 networking devices. They are proprietary, but that doesn't prevent capturing packets through WinPcap/Npcap (note: at the time of writing, Npcap doesn't seem to pick up on Xbox One receivers). These packets can then be interpreted, and all data sent from connected devices can be read.

The captured data starts with a standard 802.11 data header. The only packets of interest are those with a type of Data (2) and a subtype of QoS Data (8), other traffic types are not protocol/input data and should be ignored. The receiver address (address 1) in the header is, well, the address of the Xbox One receiver. The transmitter address is the address of the controller, and the destination address is usually that of the receiver but in some cases it will be slightly different (where the first byte of the address would normally be `0x62`, it will instead be `0x60`).

After the 802.11 header comes the protocol data. It has its own header to describe what is happening, which is then followed by the actual packet data. A reference on this header and the following data may be found here [here](https://gist.github.com/TheNathannator/c5d3b41a12db739b7ffc3d8d1a87c60a).

Packet sniffing isn't the most elegant solution since it requires additional dependencies to be installed on the user's PC. However, it is currently the only way to get data from wireless devices directly without running into certain limitations imposed by the Xbox One driver's interface. This limitation only really matters for things such as remapper programs though, for games this is perfectly fine.

### Directly from the Driver

If you want to implement support for Xbox One devices directly and don't care about the limitations it has, the Xbox One driver exposes an interface which may be used to get similar data to that which you would get through packet sniffing. Certain semantics of the protocol are already handled by the time the driver hands over the data to these programs, so it is arguably the easiest method of interacting with Xbox One devices directly to implement. A full writeup on the driver interface may be found [here](https://gist.github.com/TheNathannator/bcebc77e653f71e77634144940871596).

The limitations this interface has are minor, but make it unusable in certain niche scenarios. Certain types of data are not sent if your app doesn't have a focused, non-console window, including input data. This is a limitation imposed by the driver, and means that only direct users of Xbox One devices should use this interface. While there are potentially ways around it, there are currently no known ways to do so.

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

## The Shotgun Method (libusb)

Of course, on all of these platforms you can just completely ignore any potential existing solutions and just implement everything yourself using libusb. On Windows this is the most invasive method, as most libusb drivers completely override the original driver and make the device unusable in other programs (however for remapper programs and the like this would be acceptable, as the entire point of the program is to re-interpret the data for other programs). It's the most complicated method out of everything, as you have to implement the entire protocol handling yourself. This route is very advanced and not recommended, as there is a lot that can go wrong.
