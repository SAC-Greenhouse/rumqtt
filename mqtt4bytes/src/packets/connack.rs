use crate::{FixedHeader, Error};
use bytes::{Bytes, Buf, BytesMut, BufMut};

/// Return code in connack
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ConnectReturnCode {
    Accepted = 0,
    RefusedProtocolVersion,
    BadClientId,
    ServiceUnavailable,
    BadUsernamePassword,
    NotAuthorized,
}

/// Acknowledgement to connect packet
#[derive(Debug, Clone, PartialEq)]
pub struct ConnAck {
    pub session_present: bool,
    pub code: ConnectReturnCode,
}

impl ConnAck {

}

impl ConnAck {
    pub fn new(code: ConnectReturnCode, session_present: bool) -> ConnAck {
        ConnAck {
            code,
            session_present,
        }
    }

    pub(crate) fn assemble(fixed_header: FixedHeader, mut bytes: Bytes) -> Result<Self, Error> {
        let variable_header_index = fixed_header.fixed_len;
        bytes.advance(variable_header_index);

        if fixed_header.remaining_len != 2 {
            return Err(Error::PayloadSizeIncorrect);
        }

        let flags = bytes.get_u8();
        let return_code = bytes.get_u8();

        let session_present = (flags & 0x01) == 1;
        let code = connect_return(return_code)?;
        let connack = ConnAck {
            session_present,
            code,
        };

        Ok(connack)
    }

    pub fn write(&self, buffer: &mut BytesMut) -> Result<usize, Error> {
        let session_present = self.session_present as u8;
        let code = self.code as u8;
        let o: &[u8] = &[0x20, 0x02, session_present, code];
        buffer.put_slice(o);
        Ok(4)
    }
}

/// Connection return code type
fn connect_return(num: u8) -> Result<ConnectReturnCode, Error> {
    match num {
        0 => Ok(ConnectReturnCode::Accepted),
        1 => Ok(ConnectReturnCode::RefusedProtocolVersion),
        2 => Ok(ConnectReturnCode::BadClientId),
        3 => Ok(ConnectReturnCode::ServiceUnavailable),
        4 => Ok(ConnectReturnCode::BadUsernamePassword),
        5 => Ok(ConnectReturnCode::NotAuthorized),
        num => Err(Error::InvalidConnectReturnCode(num)),
    }
}


#[cfg(test)]
mod test {
    use crate::{ConnAck, ConnectReturnCode};
    use bytes::BytesMut;
    use alloc::vec;

    #[test]
    fn write_packet_connack_works() {
        let connack = ConnAck {
            session_present: true,
            code: ConnectReturnCode::Accepted,
        };

        let mut buf = BytesMut::new();
        connack.write(&mut buf).unwrap();
        assert_eq!(buf, vec![0b0010_0000, 0x02, 0x01, 0x00]);
    }
}