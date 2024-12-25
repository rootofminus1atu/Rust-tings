use std::cell::RefCell;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let johnny = Rc::new(RefCell::new(Pc::new("johnny".into(), MacAddress { value: "m1".into() })));
    let mark = Rc::new(RefCell::new(Pc::new("mark".into(), MacAddress { value: "m2".into() })));

    let cable = Rc::new(RefCell::new(EthernetCable::default()));

    connect(&johnny, &mark, &cable);

    let res = johnny.borrow().send_data("some_data", MacAddress { value: "m2".into() });
    dbg!(&res);

    Ok(())
}

fn connect(pc1: &Rc<RefCell<Pc>>, pc2: &Rc<RefCell<Pc>>, cable: &Rc<RefCell<EthernetCable>>) {
    pc1.borrow_mut().ethernet_connection = Some(cable.clone());
    pc2.borrow_mut().ethernet_connection = Some(cable.clone());
    cable.borrow_mut().pc1 = Some(pc1.clone());
    cable.borrow_mut().pc2 = Some(pc2.clone());
}


#[derive(thiserror::Error, Debug, Clone)]
enum Error {
    #[error("someerr")]
    SomeErr,
    #[error("not connected")]
    NotConnected,
}

#[derive(Debug, Clone)]
struct Pc {
    name: String,
    ethernet_port: MacAddress,
    ethernet_connection: Option<Rc<RefCell<EthernetCable>>>,
}

impl Pc {
    pub fn new(name: String, ethernet_port: MacAddress) -> Self {
        Self { name, ethernet_port, ethernet_connection: None }
    }

    pub fn send_data(&self, data: &str, destination: MacAddress) -> Result<(), Error> {
        if let Some(cable) = &self.ethernet_connection {

        }
        Err(Error::NotConnected)
    }

    pub fn receive_data(&self) -> Result<(), Error> {
        println!("i am {} and i received {}", self.name, );
        // hmmmmm....
        Ok(())
    }

    // pub fn connect_cable(&mut self, cable: Rc<RefCell<EthernetCable>>) {
    //     self.ethernet_connection = Some(cable);
    // }
}

#[derive(Debug, Clone, Default)]
struct EthernetCable {
    pc1: Option<Rc<RefCell<Pc>>>,
    pc2: Option<Rc<RefCell<Pc>>>,
}

impl EthernetCable {
    // pub fn new(pc1: Rc<RefCell<Pc>>, pc2: Rc<RefCell<Pc>>) -> Self {
    //     Self { pc1, pc2 }
    // }

    // pub fn connect(&mut self, pc: Rc<RefCell<Pc>>) {
    //     match (&self.pc1, &self.pc2) {
    //         (None, _) => self.pc1 = Some(pc),
    //         (Some(_), None) => self.pc2 = Some(pc),
    //         _ => (), // both pc1 and pc2 already connected
    //     }
    // }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MacAddress {
    value: String,
}