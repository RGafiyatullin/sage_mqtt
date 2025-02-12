use crate::{ReasonCode::ProtocolError, Result as SageResult};
use std::marker::Unpin;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Write the given byte into `writer`.
/// In case of success, returns `1`
pub async fn write_byte<W: AsyncWrite + Unpin>(byte: u8, writer: &mut W) -> SageResult<usize> {
    Ok(writer.write(&[byte]).await?)
}

/// Write the given bool into `writer` in a single byte value.
/// MQTT5 specifications do not define an actual boolean type but expresses it
/// with a byte being `0x00` for `false` or `0x01` for `false`. Other values are
/// considered incorrect.
/// In case of success, returns `1`
pub async fn write_bool<W: AsyncWrite + Unpin>(data: bool, writer: &mut W) -> SageResult<usize> {
    Ok(writer.write(&[data as u8]).await?)
}

/// Read the given `reader` for a byte value.
/// In case of success, returns an `u8`
pub async fn read_byte<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<u8> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf).await?;
    Ok(buf[0])
}

/// Read the given `reader` for a boolean value.
/// MQTT5 specifications do not define an actual boolean type but expresses it
/// with a byte being `0x00` for `false` or `0x01` for `false`. Other values are
/// considered incorrect.
/// In case of success, returns an `bool`
pub async fn read_bool<R: AsyncRead + Unpin>(reader: &mut R) -> SageResult<bool> {
    let byte = read_byte(reader).await?;
    match byte {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(ProtocolError.into()),
    }
}

#[cfg(test)]
mod unit {

    use super::*;
    use crate::Error;
    use std::io::{Cursor, ErrorKind};

    #[tokio::test]
    async fn encode() {
        let mut buffer = Vec::new();
        let result = write_byte(0b00101010, &mut buffer).await.unwrap();
        assert_eq!(result, 1);
        assert_eq!(buffer, vec![0x2A]);
    }

    #[tokio::test]
    async fn decode() {
        let mut test_stream = Cursor::new([0xAF_u8]);
        let result = read_byte(&mut test_stream).await.unwrap();
        assert_eq!(result, 0xAF);
    }

    #[tokio::test]
    async fn decode_eof() {
        let mut test_stream: Cursor<[u8; 0]> = Default::default();
        let result = read_byte(&mut test_stream).await;
        if let Some(Error::Io(err)) = result.err() {
            assert!(matches!(err.kind(), ErrorKind::UnexpectedEof));
        } else {
            panic!("Should be IO Error");
        }
    }

    #[tokio::test]
    async fn encode_true() {
        let mut buffer = Vec::new();
        let result = write_bool(true, &mut buffer).await.unwrap();
        assert_eq!(result, 1);
        assert_eq!(buffer, vec![0x01]);
    }

    #[tokio::test]
    async fn encode_false() {
        let mut buffer = Vec::new();
        let result = write_bool(false, &mut buffer).await.unwrap();
        assert_eq!(result, 1);
        assert_eq!(buffer, vec![0x00]);
    }

    #[tokio::test]
    async fn decode_true() {
        let mut test_stream = Cursor::new([0x01_u8]);
        let result = read_bool(&mut test_stream).await.unwrap();
        assert_eq!(result, true);
    }

    #[tokio::test]
    async fn decode_false() {
        let mut test_stream = Cursor::new([0x00_u8]);
        let result = read_bool(&mut test_stream).await.unwrap();
        assert_eq!(result, false);
    }
}
