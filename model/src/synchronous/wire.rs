use std::io;

pub trait Wire {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize>;

    fn read_exact(&mut self, buffer: &mut [u8]) -> io::Result<()>;

    fn write(&mut self, buffer: &[u8]) -> io::Result<usize>;

    fn write_all(&mut self, buffer: &[u8]) -> io::Result<()>;
}

impl<W> Wire for W
where
    W: io::Read + io::Write,
{
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        io::Read::read(self, buffer)
    }

    fn read_exact(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        io::Read::read_exact(self, buffer)
    }

    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        io::Write::write(self, buffer)
    }

    fn write_all(&mut self, buffer: &[u8]) -> io::Result<()> {
        io::Write::write_all(self, buffer)
    }
}
