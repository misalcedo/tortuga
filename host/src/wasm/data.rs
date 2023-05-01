#[derive(Clone)]
pub struct Data<Additional, Connection> {
    connection: Connection,
    additional: Additional,
}

impl<Additional, Connection> Data<Additional, Connection> {
    pub fn new(connection: Connection, additional: Additional) -> Self {
        Data {
            connection,
            additional,
        }
    }

    pub fn additional_mut(&mut self) -> &mut Additional {
        &mut self.additional
    }

    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.connection
    }

    pub fn into_connection(self) -> Connection {
        self.connection
    }
}
