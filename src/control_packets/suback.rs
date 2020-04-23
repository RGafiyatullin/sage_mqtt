use crate::{
    ControlPacketType, Decode, Encode, Error, PropertiesDecoder, Property, ReadByte, ReasonCode,
    Result as SageResult, TwoByteInteger, VariableByteInteger, WriteByte,
};
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct SubAck {
    pub packet_identifier: u16,
    pub user_properties: Vec<(String, String)>,
    pub reason_codes: Vec<ReasonCode>,
}

impl Default for SubAck {
    fn default() -> Self {
        SubAck {
            packet_identifier: 0,
            user_properties: Default::default(),
            reason_codes: Default::default(),
        }
    }
}

impl SubAck {
    pub fn write<W: Write>(self, writer: &mut W) -> SageResult<usize> {
        let mut n_bytes = TwoByteInteger(self.packet_identifier).encode(writer)?;

        let mut properties = Vec::new();

        for (k, v) in self.user_properties {
            n_bytes += Property::UserProperty(k, v).encode(&mut properties)?;
        }

        n_bytes += VariableByteInteger(properties.len() as u32).encode(writer)?;
        writer.write_all(&properties)?;

        for reason_code in self.reason_codes {
            n_bytes += reason_code.write_byte(writer)?;
        }

        Ok(n_bytes)
    }

    pub fn read<R: Read>(reader: &mut R, remaining_size: usize) -> SageResult<Self> {
        let mut reader = reader.take(remaining_size as u64);

        let packet_identifier = TwoByteInteger::decode(&mut reader)?.into();
        let mut user_properties = Vec::new();
        let mut properties = PropertiesDecoder::take(&mut reader)?;
        while properties.has_properties() {
            match properties.read()? {
                Property::UserProperty(k, v) => user_properties.push((k, v)),
                _ => return Err(Error::ProtocolError),
            }
        }

        let mut reason_codes = Vec::new();

        while reader.limit() > 0 {
            reason_codes.push(ReasonCode::try_parse(
                u8::read_byte(&mut reader)?,
                ControlPacketType::SUBACK,
            )?);
        }

        Ok(SubAck {
            packet_identifier,
            user_properties,
            reason_codes,
        })
    }
}
