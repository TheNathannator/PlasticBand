# HID Device Info Template

This is a template and guide for creating HID device data documents. Text that is not part of describing the device should be removed, as it is for guidance purposes only.

This template probably needs some adjustments and additional categories

Resources to assist in the documentation process:

- [USB Implementers Forum documentation on HID](https://www.usb.org/hid)
- [Ryochan7's HID Report Inspector](https://github.com/Ryochan7/HidReportInspector/)
- [abend0c1's hidrdd (HID Report Descriptor Decoder)](https://github.com/abend0c1/hidrdd)
- [PCILookup](https://www.pcilookup.com) for vendor/product strings
- TODO: Should probably have more resources here, feel free to contribute!

## Device Info

One way vendor and product IDs may be retrieved is through Device Manager on Windows. HID APIs should also be capable of reporting this information, and they are also typically included in device instance paths.

- Vendor ID: `<Hexadecimal number>` ("Vendor Name", if applicable)
- Product ID: `<Hexadecimal number>` ("Product Name", if applicable)
- Device Descriptor: Link (set this to link to a descriptor dump in the [Descriptor Dumps](../Descriptor%20Dumps/) folder, such as `[Link](../Descriptor%20Dumps/PS3/PS3 Rock Band Guitar.txt)`)
  - Some HID devices might have broken report descriptors. If this is the case, make sure to note it as such.

If there are other IDs or properties that are necessary for distinguishing a device, those should be listed here as well.

## Additional Steps

Additional steps to initialize the device or otherwise make it actually work should be detailed here, if necessary.

## Input Info

Input reports should be detailed here.

## Output Info

Output reports should be detailed here, if applicable.

### As A Struct

You may include C/C++ struct representations of the data reports here.

```cpp
// Remove the comments here when done, as these are for tutorial purposes only.
// Replace <device> here with a name for the device.
struct <device>State
{
    // Data members go here.
    // Use bitfields where practical or needed, such as buttons.
}
```

## References

List any external references you used here.
