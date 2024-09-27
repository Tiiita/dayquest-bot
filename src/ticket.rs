pub struct Ticket {
    _channel: u64,
    _creator: u64,
    _ticket_type: TicketType,
}

impl Ticket {
    pub fn new(channel: u64, creator: u64, ticket_type: TicketType) -> Self {
        Ticket {
            _channel: channel,
            _creator: creator,
            _ticket_type: ticket_type
        }
    }

    pub fn create(&self) {

    }
}

pub enum TicketType {
    Support,
    Application,
    Question,
}