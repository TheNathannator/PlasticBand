// SPDX-License-Identifier: GPL-3.0-or-later

use std::{io, mem};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::*;

fn invalid_data<U>(message: &str) -> io::Result<U> {
    Err(io::Error::new(io::ErrorKind::InvalidInput, message))
}

fn invalid_input<U>(message: &str) -> io::Result<U> {
    Err(io::Error::new(io::ErrorKind::InvalidInput, message))
}

fn checked_cast<T, U>(value: T) -> io::Result<U>
where
    U: TryFrom<T>,
{
    U::try_from(value).or_else(|_| invalid_input("value too large for binary storage format"))
}

// Custom reader to help validate data lengths and offsets
struct BoundedReader<Reader: io::Read + io::Seek> {
    reader: Reader,
    inner_position: u64,

    inner_start: u64,
    inner_end: u64,
}

impl<R: io::Read + io::Seek> BoundedReader<R> {
    fn new(mut reader: R, start: u64, length: u64) -> io::Result<Self> {
        let position = reader.stream_position()?;
        assert!(position >= start);
        assert!(position <= start + length);

        Ok(Self {
            reader,
            inner_position: position,
            inner_start: start,
            inner_end: start + length,
        })
    }
}

impl<R: io::Read + io::Seek> io::Read for BoundedReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.inner_position < self.inner_start || self.inner_position >= self.inner_end {
            return Ok(0);
        }

        // Clamp buffer length to inner bounds
        let remaining = self.inner_end - self.inner_position;
        let length = buf.len().min(remaining as usize);
        let buf = &mut buf[..length];

        let amount_read = self.reader.read(buf)?;
        self.inner_position += amount_read as u64;
        Ok(amount_read)
    }
}

impl<R: io::Read + io::Seek> io::Seek for BoundedReader<R> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        // Translate seek position to be relative to inner bounds
        let position = match pos {
            io::SeekFrom::Start(offset) => self.inner_start.saturating_add(offset),
            io::SeekFrom::End(offset) => self.inner_end.saturating_add_signed(offset.min(0)),
            io::SeekFrom::Current(offset) => self.inner_position.saturating_add_signed(offset),
        };

        if position < self.inner_start {
            return invalid_input("cannot perform a negative seek");
        }

        let position = position.clamp(self.inner_start, self.inner_end);
        self.inner_position = self.reader.seek(io::SeekFrom::Start(position))?;
        Ok(self.inner_position - self.inner_start)
    }
}

impl Metadata {
    // Mock structure to help visualize the format
    // struct BinaryMetadata {
    //     header: BinaryMetadataHeader,
    //     device_metadata: BinaryDeviceMetadata,
    //     messages: BinaryArray<BinaryMessage>,
    // }

    // struct BinaryArray<T> {
    //     count: u8,
    //     items: [T; count],
    // }

    pub fn from_reader(mut reader: impl io::Read + io::Seek) -> io::Result<Self> {
        let start_position = reader.stream_position()?;

        let (header, total_size) = MetadataHeader::from_reader(&mut reader)?;
        let mut reader = BoundedReader::new(reader, start_position, total_size as u64)?;

        let device_metadata = DeviceMetadata::from_reader(&mut reader, &header)?;
        let messages = read_array(&mut reader, |r| Message::from_reader(r))?;

        Ok(Self {
            _schema: IgnoredAny,
            header,
            device_metadata,
            messages,
        })
    }

    pub fn compile(&self, mut writer: impl io::Write + io::Seek) -> io::Result<()> {
        let mut total_size = MetadataHeader::BINARY_LENGTH;
        total_size += self.device_metadata.calculate_byte_size(&self.header)?;
        total_size += get_array_size(Message::BINARY_LENGTH, self.messages.len())?;

        self.header.compile(&mut writer, total_size)?;
        self.device_metadata.compile(&mut writer, &self.header)?;
        write_array(&mut writer, &self.messages, |w, i| i.compile(w))?;

        Ok(())
    }
}

impl MetadataHeader {
    // struct BinaryMetadataHeader {
    //     length: u16,
    //     major_version: u16,
    //     minor_version: u16,
    //     reserved1: u16,
    //     reserved2: u16,
    //     reserved3: u16,
    //     reserved4: u16,
    //     total_size: u16,
    // }

    const BINARY_LENGTH: usize = 16;

    fn from_reader(mut reader: impl io::Read) -> io::Result<(Self, usize)> {
        let length = reader.read_u16::<LittleEndian>()?;
        if length as usize != Self::BINARY_LENGTH {
            return invalid_data("invalid metadata header length");
        }

        let major_version = reader.read_u16::<LittleEndian>()?;
        let minor_version = reader.read_u16::<LittleEndian>()?;
        let _reserved1 = reader.read_u16::<LittleEndian>()?;
        let _reserved2 = reader.read_u16::<LittleEndian>()?;
        let _reserved3 = reader.read_u16::<LittleEndian>()?;
        let _reserved4 = reader.read_u16::<LittleEndian>()?;
        let total_size = reader.read_u16::<LittleEndian>()?;

        assert_eq!(_reserved1, 0, "unrecognized reserved value in metadata header");
        assert_eq!(_reserved2, 0, "unrecognized reserved value in metadata header");
        assert_eq!(_reserved3, 0, "unrecognized reserved value in metadata header");
        assert_eq!(_reserved4, 0, "unrecognized reserved value in metadata header");

        let header = Self { major_version, minor_version };
        Ok((header, total_size as usize))
    }

    fn compile(&self, mut writer: impl io::Write, total_size: usize) -> io::Result<()> {
        let total_size = u16::try_from(total_size)
            .or_else(|_| invalid_input("total metadata size is too large for the binary storage format"))?;

        writer.write_u16::<LittleEndian>(checked_cast(Self::BINARY_LENGTH)?)?;
        writer.write_u16::<LittleEndian>(self.major_version)?;
        writer.write_u16::<LittleEndian>(self.minor_version)?;
        writer.write_u16::<LittleEndian>(0)?; // reserved1
        writer.write_u16::<LittleEndian>(0)?; // reserved2
        writer.write_u16::<LittleEndian>(0)?; // reserved3
        writer.write_u16::<LittleEndian>(0)?; // reserved4
        writer.write_u16::<LittleEndian>(total_size)?;

        Ok(())
    }
}

impl DeviceMetadata {
    // struct BinaryDeviceMetadata {
    //     length: u16,
    //     firmware_versions_offset: u16,
    //     audio_formats_offset: u16,
    //     in_commands_offset: u16,
    //     out_commands_offset: u16,
    //     preferred_types_offset: u16,
    //     interfaces_offset: u16,
    //     hid_descriptor_offset: u16,
    //     reserved1: u16,
    //     reserved2: u16,
    //     reserved3: u16,

    //     firmware_versions: BinaryArray<BinaryFirmwareVersion>,
    //     audio_formats: BinaryArray<BinaryAudioFormat>,
    //     in_commands: BinaryArray<u8>,
    //     out_commands: BinaryArray<u8>,
    //     preferred_types: BinaryArray<BinaryString>,
    //     interfaces: BinaryArray<Uuid>,
    //     hid_descriptor: BinaryArray<u8>,
    // }

    // struct BinaryArray<T> {
    //     count: u8,
    //     items: [T; count],
    // }

    // struct BinaryFirmwareVersion {
    //     major: u16,
    //     minor: u16,
    // }

    // struct BinaryAudioFormat {
    //     input: u8,
    //     output: u8,
    // }

    // struct BinaryString {
    //     length: u16,
    //     text: [u8; length],
    // }

    const HEADER_LENGTH: usize = 22;

    fn calculate_byte_size(&self, header: &MetadataHeader) -> io::Result<usize> {
        let mut total_size = Self::HEADER_LENGTH;

        total_size += get_array_size(FirmwareVersion::BINARY_LENGTH, self.firmware_versions.len())?;
        total_size += get_array_size(AudioIoFormat::BINARY_LENGTH, self.audio_formats.len())?;
        total_size += get_bytes_size(&self.in_commands);
        total_size += get_bytes_size(&self.out_commands);
        total_size += get_strings_size(&self.preferred_types)?;
        total_size += get_array_size(mem::size_of::<uuid::Bytes>(), self.interfaces.len())?;

        if header.major_version >= 1 && header.minor_version >= 1 {
            if let Some(hid_descriptor) = self.hid_descriptor.as_ref() {
                total_size += get_bytes_size(&hid_descriptor.0);
            }
        }

        Ok(total_size)
    }

    fn from_reader(mut reader: impl io::Read + io::Seek, header: &MetadataHeader) -> io::Result<Self> {
        fn read_offset_array<T, R>(
            reader: &mut R,
            offset: u64,
            f: impl Fn(&mut R) -> io::Result<T>,
        ) -> io::Result<Vec<T>>
        where
            R: io::Read + io::Seek,
        {
            if offset == 0 {
                return Ok(Vec::new());
            }

            reader.seek(io::SeekFrom::Start(offset))?;
            read_array(reader, f)
        }

        fn read_offset_bytes(mut reader: impl io::Read + io::Seek, offset: u64) -> io::Result<Vec<u8>> {
            if offset == 0 {
                return Ok(Vec::new());
            }

            reader.seek(io::SeekFrom::Start(offset))?;
            read_byte_array(reader)
        }

        fn read_offset_strings(mut reader: impl io::Read + io::Seek, offset: u64) -> io::Result<Vec<String>> {
            if offset == 0 {
                return Ok(Vec::new());
            }

            reader.seek(io::SeekFrom::Start(offset))?;
            read_string_array(reader)
        }

        let start_position = reader.stream_position()?;

        // Data block length
        let length = reader.read_u16::<LittleEndian>()?;
        let mut reader = BoundedReader::new(reader, start_position, length as u64)?;

        // Header values
        let firmware_versions_offset = reader.read_u16::<LittleEndian>()? as u64;
        let audio_formats_offset = reader.read_u16::<LittleEndian>()? as u64;
        let in_commands_offset = reader.read_u16::<LittleEndian>()? as u64;
        let out_commands_offset = reader.read_u16::<LittleEndian>()? as u64;
        let preferred_types_offset = reader.read_u16::<LittleEndian>()? as u64;
        let interfaces_offset = reader.read_u16::<LittleEndian>()? as u64;
        let hid_descriptor_offset = reader.read_u16::<LittleEndian>()? as u64;
        let _reserved1 = reader.read_u16::<LittleEndian>()?;
        let _reserved2 = reader.read_u16::<LittleEndian>()?;
        let _reserved3 = reader.read_u16::<LittleEndian>()?;

        assert_eq!(_reserved1, 0, "unrecognized reserved value in device metadata");
        assert_eq!(_reserved2, 0, "unrecognized reserved value in device metadata");
        assert_eq!(_reserved3, 0, "unrecognized reserved value in device metadata");

        // The actual metadata values
        let firmware_versions =
            read_offset_array(&mut reader, firmware_versions_offset, |r| FirmwareVersion::from_reader(r))?;
        let audio_formats = read_offset_array(&mut reader, audio_formats_offset, |r| AudioIoFormat::from_reader(r))?;
        let in_commands = read_offset_bytes(&mut reader, in_commands_offset)?;
        let out_commands = read_offset_bytes(&mut reader, out_commands_offset)?;
        let preferred_types = read_offset_strings(&mut reader, preferred_types_offset)?;
        let interfaces = read_offset_array(&mut reader, interfaces_offset, |r| {
            let mut bytes = [0u8; 16];
            io::Read::read_exact(r, &mut bytes)?;
            Ok(Uuid::from_bytes_le(bytes))
        })?;
        let hid_descriptor = if header.major_version >= 1 && header.minor_version >= 1 {
            let hid_descriptor = read_offset_bytes(&mut reader, hid_descriptor_offset)?;
            if hid_descriptor.is_empty() {
                None
            } else {
                Some(HidDescriptor(hid_descriptor))
            }
        } else {
            None
        };

        Ok(Self {
            in_commands,
            out_commands,
            audio_formats,
            firmware_versions,
            preferred_types,
            interfaces,
            hid_descriptor,
        })
    }

    fn compile(&self, mut writer: impl io::Write + io::Seek, header: &MetadataHeader) -> io::Result<()> {
        let start_offset = writer.stream_position()?;
        // for de-duplication
        let adjust_offset = |offset: u64| -> io::Result<u16> { checked_cast(offset - start_offset) };

        // Header is filled with dummy data that gets filled in later
        writer.write_all(&[0u8; Self::HEADER_LENGTH])?;

        // Write out data and keep track of where it was written
        let firmware_versions_offset = write_array(&mut writer, &self.firmware_versions, |w, i| i.compile(w))?;
        let audio_formats_offset = write_array(&mut writer, &self.audio_formats, |w, i| i.compile(w))?;
        let in_commands_offset = write_byte_array(&mut writer, &self.in_commands)?;
        let out_commands_offset = write_byte_array(&mut writer, &self.out_commands)?;
        let preferred_types_offset = write_string_array(&mut writer, &self.preferred_types)?;
        let interfaces_offset = write_array(&mut writer, &self.interfaces, |w, i| w.write_all(&i.to_bytes_le()))?;
        let hid_descriptor_offset = match self.hid_descriptor.as_ref() {
            Some(hid_descriptor) if header.major_version >= 1 && header.minor_version >= 1 => {
                adjust_offset(write_byte_array(&mut writer, &hid_descriptor.0)?)?
            },
            _ => 0,
        };

        // Seek back to the start of the block and fill in header values
        let end_offset = writer.stream_position()?;
        writer.seek(io::SeekFrom::Start(start_offset))?;

        writer.write_u16::<LittleEndian>(adjust_offset(end_offset)?)?; // length
        writer.write_u16::<LittleEndian>(adjust_offset(firmware_versions_offset)?)?;
        writer.write_u16::<LittleEndian>(adjust_offset(audio_formats_offset)?)?;
        writer.write_u16::<LittleEndian>(adjust_offset(in_commands_offset)?)?;
        writer.write_u16::<LittleEndian>(adjust_offset(out_commands_offset)?)?;
        writer.write_u16::<LittleEndian>(adjust_offset(preferred_types_offset)?)?;
        writer.write_u16::<LittleEndian>(adjust_offset(interfaces_offset)?)?;
        writer.write_u16::<LittleEndian>(hid_descriptor_offset)?; // already adjusted
        writer.write_u16::<LittleEndian>(0)?; // reserved1
        writer.write_u16::<LittleEndian>(0)?; // reserved2
        writer.write_u16::<LittleEndian>(0)?; // reserved3

        // Restore position to end of written data
        writer.seek(io::SeekFrom::Start(end_offset))?;
        Ok(())
    }
}

impl AudioIoFormat {
    // struct BinaryAudioIoFormat {
    //     inbound: BinaryAudioFormat,
    //     outbound: BinaryAudioFormat,
    // }

    const BINARY_LENGTH: usize = AudioFormat::BINARY_LENGTH * 2;

    fn from_reader(mut reader: impl io::Read) -> io::Result<Self> {
        let inbound = AudioFormat::from_reader(&mut reader)?;
        let outbound = AudioFormat::from_reader(&mut reader)?;
        Ok(Self { inbound, outbound })
    }

    fn compile(&self, mut writer: impl io::Write) -> io::Result<()> {
        if let Some(inbound) = &self.inbound {
            inbound.compile(&mut writer)?;
        }

        if let Some(outbound) = &self.outbound {
            outbound.compile(&mut writer)?;
        }

        Ok(())
    }
}

impl AudioFormat {
    // struct BinaryAudioFormat {
    //     format: u8,
    // }

    const BINARY_LENGTH: usize = 1;

    fn from_reader(mut reader: impl io::Read) -> io::Result<Option<Self>> {
        let format = reader.read_u8()?;
        let (channels, rate) = match format {
            0 => return Ok(None),
            1 => (1, 8000),
            2 => (2, 8000),
            3 => (1, 12000),
            4 => (2, 12000),
            5 => (1, 16000),
            6 => (2, 16000),
            7 => (1, 20000),
            8 => (2, 20000),
            9 => (1, 24000),
            10 => (2, 24000),
            11 => (1, 32000),
            12 => (2, 32000),
            13 => (1, 40000),
            14 => (2, 40000),
            15 => (1, 48000),
            16 => (2, 48000),
            32 => (6, 48000),
            33 => (8, 48000),
            _ => return invalid_data("unrecognized audio format"),
        };
        Ok(Some(Self { channels, rate }))
    }

    fn compile(&self, mut writer: impl io::Write) -> io::Result<()> {
        let format = match (self.channels, self.rate) {
            (1, 8000) => 1,
            (2, 8000) => 2,
            (1, 12000) => 3,
            (2, 12000) => 4,
            (1, 16000) => 5,
            (2, 16000) => 6,
            (1, 20000) => 7,
            (2, 20000) => 8,
            (1, 24000) => 9,
            (2, 24000) => 10,
            (1, 32000) => 11,
            (2, 32000) => 12,
            (1, 40000) => 13,
            (2, 40000) => 14,
            (1, 48000) => 15,
            (2, 48000) => 16,
            (6, 48000) => 32,
            (8, 48000) => 33,
            _ => return invalid_input("unsupported audio format channel and sample rate combination"),
        };
        writer.write_u8(format)?;
        Ok(())
    }
}

impl FirmwareVersion {
    // struct BinaryFirmwareVersion {
    //     major: u16,
    //     minor: u16,
    // }

    const BINARY_LENGTH: usize = 4;

    fn from_reader(mut reader: impl io::Read) -> io::Result<Self> {
        let major = reader.read_u16::<LittleEndian>()?;
        let minor = reader.read_u16::<LittleEndian>()?;
        Ok(Self { major, minor })
    }

    fn compile(&self, mut writer: impl io::Write) -> io::Result<()> {
        writer.write_u16::<LittleEndian>(self.major)?;
        writer.write_u16::<LittleEndian>(self.minor)?;
        Ok(())
    }
}

impl Message {
    // struct BinaryMessage {
    //     length: u16,
    //     message_id: u8,
    //     max_length: u16,
    //     message_kind: u16,
    //     flags: u32,
    //     period: u16,
    //     persistence_timeout: u16,
    //     reserved1: u16,
    //     reserved2: u16,
    //     reserved3: u16,
    //     reserved4: u16,
    // }

    const BINARY_LENGTH: usize = 23;

    const BIG_ENDIAN_MASK: u32 = 1 << 0;
    const RELIABLE_MASK: u32 = 1 << 1;
    const SEQUENCED_MASK: u32 = 1 << 2;
    const DOWNSTREAM_MASK: u32 = 1 << 3;
    const UPSTREAM_MASK: u32 = 1 << 4;
    const DOWNSTREAM_REQUESTS_RESPONSE_MASK: u32 = 1 << 5;

    fn from_reader(mut reader: impl io::Read) -> io::Result<Self> {
        let length = reader.read_u16::<LittleEndian>()?;
        if length as usize != Self::BINARY_LENGTH {
            return invalid_data("invalid custom message data length");
        }

        let message_id = reader.read_u8()?;
        let max_length = reader.read_u16::<LittleEndian>()?;
        let message_kind = match reader.read_u16::<LittleEndian>()? {
            1 => MessageKind::Custom,
            2 => MessageKind::Audio,
            3 => MessageKind::Security,
            4 => MessageKind::GIP,
            _ => return invalid_data("unrecognized custom message kind"),
        };

        let flags = reader.read_u32::<LittleEndian>()?;
        let period = reader.read_u16::<LittleEndian>()?;
        let persistence_timeout = reader.read_u16::<LittleEndian>()?;
        let _reserved1 = reader.read_u16::<LittleEndian>()?;
        let _reserved2 = reader.read_u16::<LittleEndian>()?;
        let _reserved3 = reader.read_u16::<LittleEndian>()?;
        let _reserved4 = reader.read_u16::<LittleEndian>()?;

        assert_eq!(_reserved1, 0, "unrecognized reserved value in custom message data");
        assert_eq!(_reserved2, 0, "unrecognized reserved value in custom message data");
        assert_eq!(_reserved3, 0, "unrecognized reserved value in custom message data");
        assert_eq!(_reserved4, 0, "unrecognized reserved value in custom message data");

        Ok(Self {
            message_id,
            max_length,
            message_kind,
            is_big_endian: (flags & Self::BIG_ENDIAN_MASK) != 0,
            is_reliable: (flags & Self::RELIABLE_MASK) != 0,
            is_sequenced: (flags & Self::SEQUENCED_MASK) != 0,
            is_upstream: (flags & Self::DOWNSTREAM_MASK) != 0,
            is_downstream: (flags & Self::UPSTREAM_MASK) != 0,
            downstream_requests_response: (flags & Self::DOWNSTREAM_REQUESTS_RESPONSE_MASK) != 0,
            period,
            persistence_timeout,
        })
    }

    fn compile(&self, mut writer: impl io::Write) -> io::Result<()> {
        writer.write_u16::<LittleEndian>(checked_cast(Self::BINARY_LENGTH)?)?;

        writer.write_u8(self.message_id)?;
        writer.write_u16::<LittleEndian>(self.max_length)?;
        writer.write_u16::<LittleEndian>(match self.message_kind {
            MessageKind::Custom => 1,
            MessageKind::Audio => 2,
            MessageKind::Security => 3,
            MessageKind::GIP => 4,
        })?;

        let mut flags = 0;
        if self.is_big_endian {
            flags |= Self::BIG_ENDIAN_MASK;
        }
        if self.is_reliable {
            flags |= Self::RELIABLE_MASK;
        }
        if self.is_sequenced {
            flags |= Self::SEQUENCED_MASK;
        }
        if self.is_upstream {
            flags |= Self::DOWNSTREAM_MASK;
        }
        if self.is_downstream {
            flags |= Self::UPSTREAM_MASK;
        }
        if self.downstream_requests_response {
            flags |= Self::DOWNSTREAM_REQUESTS_RESPONSE_MASK;
        }
        writer.write_u32::<LittleEndian>(flags)?;

        writer.write_u16::<LittleEndian>(self.period)?;
        writer.write_u16::<LittleEndian>(self.persistence_timeout)?;

        writer.write_u16::<LittleEndian>(0)?; // reserved1
        writer.write_u16::<LittleEndian>(0)?; // reserved2
        writer.write_u16::<LittleEndian>(0)?; // reserved3
        writer.write_u16::<LittleEndian>(0)?; // reserved4

        Ok(())
    }
}

fn get_array_size(element_size: usize, element_count: usize) -> io::Result<usize> {
    // Element count is a u8
    let total_size = 1;

    match element_size.checked_mul(element_count) {
        Some(size_to_add) => Ok(total_size + size_to_add),
        None => invalid_input("metadata is too large for the binary storage format"),
    }
}

fn get_bytes_size(bytes: &[u8]) -> usize {
    // Element count is a u8
    1 + bytes.len()
}

fn get_strings_size(strings: &[String]) -> io::Result<usize> {
    // Element count is a u8
    let mut total_size = 1;

    for string in strings {
        total_size += 2; // String length is a u16
        total_size += string.len();
    }

    Ok(total_size)
}

fn read_array<T, R>(reader: &mut R, f: impl Fn(&mut R) -> io::Result<T>) -> io::Result<Vec<T>>
where
    R: io::Read + io::Seek,
{
    let count = reader.read_u8()?;
    let mut items = Vec::with_capacity(count as usize);
    for _i in 0..count {
        items.push(f(reader)?);
    }

    Ok(items)
}

fn read_byte_array(mut reader: impl io::Read + io::Seek) -> io::Result<Vec<u8>> {
    let count = reader.read_u8()?;
    let mut bytes = vec![0u8; count as usize];
    reader.read_exact(&mut bytes)?;

    Ok(bytes)
}

fn read_string_array(mut reader: impl io::Read + io::Seek) -> io::Result<Vec<String>> {
    let count = reader.read_u8()?;
    let mut items = Vec::with_capacity(count as usize);
    for _i in 0..count {
        let string_length = reader.read_u16::<LittleEndian>()?;
        if string_length == 0 {
            continue;
        }

        let mut string = vec![0u8; string_length as usize];
        reader.read_exact(&mut string)?;

        // Only ASCII text is valid for strings
        if !string.is_ascii() {
            return invalid_input("string text is not representable in ASCII");
        }

        items.push(String::from_utf8(string).unwrap());
    }

    Ok(items)
}

fn write_array<T, W>(mut writer: W, items: &[T], f: impl Fn(&mut W, &T) -> io::Result<()>) -> io::Result<u64>
where
    W: io::Write + io::Seek,
{
    let start_offset = writer.stream_position()?;

    // Element count is a u8
    writer.write_u8(checked_cast(items.len())?)?;
    writer.flush()?;

    for item in items {
        f(&mut writer, item)?;
        writer.flush()?;
    }

    Ok(start_offset)
}

fn write_byte_array(mut writer: impl io::Write + io::Seek, bytes: &[u8]) -> io::Result<u64> {
    let start_offset = writer.stream_position()?;

    // Element count is a u8
    writer.write_u8(checked_cast(bytes.len())?)?;
    writer.flush()?;
    writer.write_all(bytes)?;
    writer.flush()?;

    Ok(start_offset)
}

fn write_string_array(mut writer: impl io::Write + io::Seek, strings: &[String]) -> io::Result<u64> {
    let start_offset = writer.stream_position()?;

    // Element count is a u8
    writer.write_u8(checked_cast(strings.len())?)?;
    writer.flush()?;

    for string in strings {
        writer.write_u16::<LittleEndian>(checked_cast(string.len())?)?;
        writer.flush()?;

        // Only ASCII text is valid for strings
        if !string.is_ascii() {
            return invalid_input("string text is not representable in ASCII");
        }

        writer.write_all(string.as_bytes())?;
        writer.flush()?;
    }

    Ok(start_offset)
}
