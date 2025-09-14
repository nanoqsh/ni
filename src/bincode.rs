use {
    crate::{Error, name::Name},
    bincode::{
        Decode, Encode,
        de::{Decoder, read::Reader},
        enc::{Encoder, write::Writer},
        error::{DecodeError, EncodeError},
    },
};

impl Encode for Name {
    #[inline]
    fn encode<E>(&self, encoder: &mut E) -> Result<(), EncodeError>
    where
        E: Encoder,
    {
        (self.len() as u8).encode(encoder)?;
        encoder.writer().write(self.decode().as_slice())
    }
}

impl<C> Decode<C> for Name {
    #[inline]
    fn decode<D>(decoder: &mut D) -> Result<Self, DecodeError>
    where
        D: Decoder<Context = C>,
    {
        let len = usize::from(u8::decode(decoder)?);
        decoder.claim_container_read::<u8>(len)?;

        let mut buf = [0; Self::MAXLEN];
        let buf = buf
            .get_mut(..len)
            .ok_or(DecodeError::Other(Error::TooLong.as_str()))?;

        decoder.reader().read(buf)?;
        Self::encode(buf).map_err(|e| DecodeError::Other(e.as_str()))
    }
}

bincode::impl_borrow_decode!(Name);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bincode() {
        let name = crate::name!("hello");
        let conf = bincode::config::standard();

        let data = bincode::encode_to_vec(name, conf).expect("encode name");
        let (name, read): (Name, _) = bincode::decode_from_slice(&data, conf).expect("decode name");

        assert_eq!(name.decode(), "hello");
        assert_eq!(read, "hello".len() + 1);
    }

    #[test]
    fn bincode_borrow() {
        let name = crate::name!("hello");
        let conf = bincode::config::standard();

        let data = bincode::encode_to_vec(name, conf).expect("encode name");
        let (name, read): (Name, _) =
            bincode::borrow_decode_from_slice(&data, conf).expect("decode name");

        assert_eq!(name.decode(), "hello");
        assert_eq!(read, "hello".len() + 1);
    }

    #[test]
    fn decode_too_long() {
        let conf = bincode::config::standard();

        let data = [(Name::MAXLEN + 1) as u8];
        let e = bincode::decode_from_slice::<Name, _>(&data, conf).expect_err("failed to decode");

        assert!(matches!(e, DecodeError::Other(s) if s == Error::TooLong.as_str()));
    }
}
