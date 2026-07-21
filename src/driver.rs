//! The even loop.
use std::collections::HashMap;
use std::os::fd::OwnedFd;

use rustix::buffer::spare_capacity;
use rustix::io::{ioctl_fionbio, read};
use rustix::net::accept;
use rustix::event::epoll::{self, EventData};

use rustix::net::{
    AddressFamily,
    SocketType,
    SocketAddrV4,
    Ipv4Addr,
    socket,
    ipproto,
    bind,
    listen,
};

/// The event loop type. It does not hold any state. The purpose of type `Driver` is merely for
/// organization and scalability.
#[derive(Debug)]
pub struct Driver();

impl Driver {
    fn create_socket() -> Result<OwnedFd, rustix::io::Errno> {
        let socket = socket(
            AddressFamily::INET, 
            SocketType::STREAM, 
            Some(ipproto::TCP)
        )?;
        let port = SocketAddrV4::new(
            Ipv4Addr::LOCALHOST, 
            8080
        ); 

        bind(&socket, &port)?;
        listen(&socket, 1)?;

        Ok(socket)
    }

    fn register_socket(
        socket: &OwnedFd,
        epoll_file: &OwnedFd,
        sockets: &mut HashMap<EventData, OwnedFd>,
        next_id: EventData,
    ) -> Result<(), rustix::io::Errno> {
        let conn_sock = accept(&socket)?;
        ioctl_fionbio(&conn_sock, true)?;
            
        epoll::add(
            &epoll_file, 
            &conn_sock,
            next_id.clone(), 
            epoll::EventFlags::IN | epoll::EventFlags::ET,
        )?;

        sockets.insert(next_id, conn_sock);
        
        Ok(())
    }
    
    fn watch_events() -> Result<(), rustix::io::Errno> {
        let socket = Self::create_socket()?;
        let epoll_file = epoll::create(epoll::CreateFlags::CLOEXEC)?;
        
        epoll::add(
            &epoll_file, 
            &socket,
            epoll::EventData::new_u64(1),
            epoll::EventFlags::IN,
        )?;

        let mut next_id = epoll::EventData::new_u64(2);
        let mut sockets = HashMap::new();
        let mut event_list = Vec::with_capacity(4);

        loop {
            epoll::wait(&epoll_file, spare_capacity(&mut event_list), None)?;

            for event in event_list.drain(..) {
                match event.flags {
                    epoll::EventFlags::IN => {
                        let target = event.data;
                        
                        if target.u64() == 1 {
                            Self::register_socket(&socket, &epoll_file,&mut sockets, next_id)?;
                            next_id = epoll::EventData::new_u64(next_id.u64() + 1);
                        } else {
                            let client = sockets.get(&target).unwrap();
                            let mut buf = vec![0u8; 512];
                            
                            let read_bytes = read(client, &mut buf).expect("the message to be read");

                            if read_bytes == 0 {
                                let target = sockets.remove(&target).unwrap();
                                let _ = epoll::delete(&epoll_file, &target);
                                println!("socket closed");
                            };
                            
                            buf.truncate(read_bytes);
                            println!("{:?}", String::from_utf8(buf).unwrap());
                        }
                    },
                    _ => eprint!("Event is not known")
                };
            }
        }
    }
    
    /// Starts the event loop openning a new connection in a random port.
    ///
    /// # Errors
    /// It will return a `DriverError::Io` if it's not possible to open a new connection.
    pub fn start() {
        let _ = Self::watch_events();
    }
}