use crate::{ctx::Ctx, Error, Res};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};


// ticket types
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64,  // creator user_id
    pub title: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TicketForCreate {
    pub title: String
}

// model controller
// currently just an in-memory store, in reality we'd use a db connection
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>
}

impl ModelController {
    pub async fn new() -> Res<Self> {
        Ok(Self {
            tickets_store: Arc::default()
        })
    }
}

// CRUD stuff
impl ModelController {
    pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> Res<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
    
        let id = store.len() as u64;
        let ticket = Ticket {
            id,
            cid: ctx.user_id(),
            title: ticket_fc.title
        };
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, ctx: Ctx) -> Res<Vec<Ticket>> {
        let store: std::sync::MutexGuard<'_, Vec<Option<Ticket>>> = self.tickets_store.lock().unwrap();

        let tickets = store.iter()
            .filter_map(|t| t.clone())
            .collect::<Vec<_>>();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, ctx: Ctx, id: u64) -> Res<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
    
        let ticket = store.get_mut(id as usize)
            .and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}




