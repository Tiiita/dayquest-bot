pub struct Ticket {
    channel: u64,
    creator: u64,
    ticket_type: TicketType,
}

impl Ticket {
    pub fn new(channel: u64, creator: u64, ticket_type: TicketType) -> Self {
        Ticket {
            channel,
            creator,
            ticket_type
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