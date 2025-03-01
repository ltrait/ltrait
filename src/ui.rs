use color_eyre::Result;

pub trait UI<'a> {
    type Context: 'a;

    fn run<Cusion: 'a + Sync>(
        &self,
        batcher: crate::launcher::batcher::Batcher<Cusion, Self::Context>,
    ) -> impl std::future::Future<Output = Result<Cusion>> + Send;
}

#[derive(Debug)]
pub struct Buffer<T> {
    // あとからactionをじっこうするためにhashmapてきに
    vec: Vec<T>,
    pos: usize,
}

impl<T> Default for Buffer<T> {
    fn default() -> Self {
        Self {
            vec: vec![],
            pos: 0,
        }
    }
}

impl<T> Buffer<T> {
    pub(crate) fn reset(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn reset_pos(&mut self) {
        self.pos = 0;
    }

    pub(crate) fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.vec
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }

    /// not iterator
    pub fn next(&mut self) -> Option<&T> {
        self.pos += 1;
        self.vec.get(self.pos - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() -> Result<(), Box<dyn std::error::Error>> {
        let mut buf = {
            let mut buf = Buffer::default();
            let v = buf.as_mut();
            v.push((1, 1));
            v.push((2, 2));
            dbg!(v);
            buf
        };

        assert_eq!(buf.next(), Some((1u32, 1)).as_ref());
        assert_eq!(buf.next(), Some((2u32, 2)).as_ref());
        assert_eq!(buf.next(), None);
        buf.reset_pos();
        assert_eq!(buf.next(), Some((1u32, 1)).as_ref());
        assert_eq!(buf.next(), Some((2u32, 2)).as_ref());
        assert_eq!(buf.next(), None);

        Ok(())
    }
}
