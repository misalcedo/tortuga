#[derive(Clone)]
pub struct Data<Connection, Additional> {
    connection: Connection,
    additional: Additional,
}

impl<Additional, Connection> Data<Connection, Additional> {
    pub fn new(connection: Connection, additional: Additional) -> Self {
        Data {
            connection,
            additional,
        }
    }

    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.connection
    }

    pub fn into_connection(self) -> Connection {
        self.connection
    }

    pub fn additional_mut(&mut self) -> &mut Additional {
        &mut self.additional
    }
}
