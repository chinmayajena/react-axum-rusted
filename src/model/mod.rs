use crate::{ Error, Result };
use serde::{ Deserialize, Serialize };
use std::sync::{ Arc, Mutex };

#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    //Arc: Stands for "atomic reference count".
    //It's a smart pointer that provides shared
    //ownership of data across multiple threads.
    //In this case, it's used to enable safe concurrent
    // access to the data in tickets_store.

    //Mutex: Short for "mutual exclusion". It's a
    //synchronization primitive that provides exclusive
    //access to the data it wraps. In this case, the Mutex
    // ensures that only one thread can modify the data
    //in tickets_store at a time.

    //Vec<Option<Ticket>>: This is the actual data type being stored
    //within the Mutex. It's a Vec (short for "vector"), which is a
    // dynamic array in Rust. Each element in the vector is of type
    //Option<Ticket>, where Ticket seems to be a custom type defined
    //elsewhere in your code. The use of Option suggests that the
    // vector can contain either a Some(Ticket) to represent a
    //ticket or None to represent an absence of a ticket.

    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

//constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };

        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();
        let tickets = store
            .iter()
            .filter_map(|t| t.clone())
            .collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIDNotFound { id })
    }
}
