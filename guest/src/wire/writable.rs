pub trait WritableMessage: Sized {
    fn write_to<W>(mut self, writer: &mut W) -> io::Result<usize>
    where
        W: Write,
    {
        self.write_header_to(writer)?;
        self.write_data_to(writer)
    }

    fn write_header_to<W>(&mut self, writer: &mut W) -> io::Result<usize>
    where
        W: Write;

    fn write_data_to<W>(&mut self, writer: &mut W) -> io::Result<usize>
    where
        W: Write;
}

impl<B> WritableMessage for Request<B>
where
    B: Body,
{
    fn write_header_to<W>(&mut self, writer: &mut W) -> io::Result<usize>
    where
        W: Write,
    {
        let length = self.body().length().unwrap_or_default();
        let mut buffer = Cursor::new(Vec::new());

        buffer.encode(self.method() as u8)?;
        buffer.encode(self.uri())?;
        buffer.encode(length)?;
        buffer.set_position(0);

        let header = Frame::new(FrameType::Header, buffer.get_ref().len());
        let mut bytes = 0;

        bytes += writer.encode(header)?;
        bytes += io::copy(&mut buffer, writer)? as usize;

        Ok(bytes)
    }

    fn write_data_to<W>(&mut self, writer: &mut W) -> io::Result<usize>
    where
        W: Write,
    {
        let length = self.body().length().unwrap_or_default();
        let mut body = FrameIo::new(writer, length);

        io::copy(self.body(), &mut body).map(|n| n as usize)
    }
}

impl<B> WritableMessage for Response<B>
where
    B: Body,
{
    fn write_header_to<W>(&mut self, writer: &mut W) -> io::Result<usize>
    where
        W: Write,
    {
        let length = self.body().length().unwrap_or_default();
        let mut buffer = Cursor::new(Vec::new());

        buffer.encode(self.status())?;
        buffer.encode(length)?;
        buffer.set_position(0);

        let header = Frame::new(FrameType::Header, buffer.get_ref().len());
        let mut bytes = 0;

        bytes += writer.encode(header)?;
        bytes += io::copy(&mut buffer, writer)? as usize;

        Ok(bytes)
    }

    fn write_data_to<W>(&mut self, writer: &mut W) -> io::Result<usize>
    where
        W: Write,
    {
        let length = self.body().length().unwrap_or_default();
        let mut body = FrameIo::new(writer, length);

        io::copy(self.body(), &mut body).map(|n| n as usize)
    }
}
