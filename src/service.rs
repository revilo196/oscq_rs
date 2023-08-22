use crate::tokiort::TokioIo;
use crate::OSCNode;

use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request, Response};
use std::any::Any;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::thread::spawn;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use zeroconf::prelude::*;

/// A Hyper service that implements the OSCQuery protocol.
///
/// This service is responsible for handling HTTP requests that conform to the OSCQuery protocol.
/// It takes an `OSCNode` as its root, and uses it to respond to requests.
///
/// Implements the `Service` trait from the Hyper crate, which is used to handle incoming requests.
///
/// The `call` method is used to handle each incoming request. It matches the request's path and query
/// parameters to OSCNode values, and returns a response in JSON format.
struct OscQueryStatic {
    /// The root of the OSCNode hierarchy.
    root: Arc<OSCNode>,
}

/// Implementation of the `hyper::service::Service` trait for serving OSC query requests.
/// Handles incoming HTTP requests and returns the appropriate OSC query responses.
impl Service<Request<IncomingBody>> for OscQueryStatic {
    type Response = Response<String>;
    type Error = hyper::Error;
    /// A future representing the eventual response value returned by this service.
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    /// Handle an incoming HTTP request and return a future representing the eventual response.
    /// If the requested resource is not found, a 404 response is returned. If a query string is present,
    /// the appropriate response is generated based on the query. Otherwise, the full OSC query data is returned.
    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        // Create a response with the given string, including the appropriate "Content-Type" header.
        fn mk_response(s: String) -> Result<Response<String>, hyper::Error> {
            println!("{}", s);
            Ok(Response::builder()
                .header("Content-Type", "application/json")
                .body(s)
                .unwrap())
        }

        // Log the incoming request method and URI for debugging purposes.
        println!("{:?} {:?}", req.uri(), req.method());

        // If the requested OSC node exists, generate an appropriate response based on the query string.
        if let Ok(node) = self.root.get(req.uri().path().to_string()) {
            if let Some(query) = req.uri().query() {
                let res = match query {
                    "HOST_INFO" => mk_response(
                        serde_json::to_value(node)
                            .unwrap()
                            .get("HOST_INFO")
                            .unwrap()
                            .to_string(),
                    ),
                    "VALUE" => mk_response(format!(
                        "{{\"VALUE\":{}}}",
                        serde_json::to_value(node).unwrap().get("VALUE").unwrap()
                    )),
                    "TYPE" => mk_response(
                        serde_json::to_value(node)
                            .unwrap()
                            .get("TYPE")
                            .unwrap()
                            .to_string(),
                    ),
                    _ => Ok(Response::builder()
                        .status(204)
                        .body("not supported".to_string())
                        .unwrap()),
                };
                return Box::pin(async { res });
            } else {
                // If no query string is present, return the full OSC query data.
                let res = mk_response(serde_json::to_string(node).unwrap());
                return Box::pin(async { res });
            }
        }

        // If the requested resource is not found, return a 404 response.
        let res = Ok(Response::builder()
            .status(404)
            .body("Not Found".to_string())
            .unwrap());
        Box::pin(async { res })
    }
}

fn on_service_registered(
    result: zeroconf::Result<zeroconf::ServiceRegistration>,
    _: Option<Arc<dyn Any>>,
) {
    let service = result.unwrap();

    println!("Service registered: {:?}", service);
}

/// Runs an OSCQuery server on the given socket address, serving the OSCNode
/// rooted at `root`.
///
/// # Arguments
///
/// * `root` - The root node of the OSCNode tree to serve.
/// * `address` - The socket address on which to listen for incoming requests.
///
/// # Returns
///
/// Returns a tuple containing two `JoinHandle`s: one for the main service loop,
/// and one for the Zeroconf service registration loop.
pub async fn run_oscquery_service(
    root: OSCNode,
    address: SocketAddr,
) -> tokio::io::Result<(tokio::task::JoinHandle<()>, tokio::task::JoinHandle<()>)> {
    let arc_root = Arc::new(root);
    println!("oscq_rs start tcp at {:?}", address);
    let listener = TcpListener::bind(address).await?;
    println!("oscq_rs started tcp at {:?}", address);

    let handle = tokio::task::spawn(async move {
        loop {
            println!("oscq_rs wait for connection {:?}", address);
            let (stream, con) = listener.accept().await.unwrap();
            println!("oscq_rs serve connection {:?}", con);
            let service = OscQueryStatic {
                root: arc_root.clone(),
            };
            let io = TokioIo::new(stream);
            tokio::task::spawn(async move {
                println!("oscq_rs serve connection async {:?}", con);
                if let Err(err) = http1::Builder::new()
                    .keep_alive(true)
                    .serve_connection(io, service)
                    .await
                {
                    println!("Failed to serve connection: {:?}", err);
                }
            });
        }
    });

    let handle1 = tokio::task::spawn(async move {
        let mut service = zeroconf::MdnsService::new(
            zeroconf::ServiceType::new("oscjson", "tcp").unwrap(),
            address.port(),
        );
        service.set_name("oscq_rs");
        service.set_registered_callback(Box::new(on_service_registered));
        let event_loop = service.register().unwrap();
        loop {
            event_loop.poll(Duration::from_secs(10)).unwrap();
        }
    });

    Ok((handle, handle1))
}

/// Spawns a new thread to run the OSCQuery service with the provided `root` OSCNode and `address`.
///
/// This function creates a new Tokio runtime and spawns a new thread to run the OSCQuery service on. The `root` argument
/// specifies the root node of the OSCQuery hierarchy, and the `address` argument specifies the network address that the
/// service should bind to.
///
/// # Arguments
///
/// * `root` - The root node of the OSCNode tree to serve.
/// * `address` - The socket address on which to listen for incoming requests.
///
/// # Returns
/// The function returns immediately after spawning the thread, and the thread will continue running until the process
/// terminates or the thread panics.
pub fn spawn_oscquery_service(root: OSCNode, address: SocketAddr) {
    spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let (x, y) = run_oscquery_service(root, address).await.unwrap();
            let res = tokio::join!(x, y);
            res.0.unwrap();
            res.1.unwrap();
        });
        loop {
            panic!("oscQueryServer Stopped");
        }
    });
}

/// This is Rust test that creates an OSCQuery server with three parameters,
/// sets the IP address and port number, and runs the OSCQuery service on the specified address.
/// It then creates a UDP socket to receive incoming OSC messages, binds it to a different address and port number,
/// and enters a loop to receive and send back any incoming messages. Finally,
/// it joins the futures for the OSCQuery server and the zeroconf server and unwraps their results.
#[tokio::test]
async fn test_service() {
    // Import necessary modules
    use crate::{OSCAccess, OSCUnit, OscHostInfo, OscQueryParameter};
    use rosc::OscType;
    use std::net::SocketAddr;

    // Create an instance of OscHostInfo with name "OSCQuery Test", IP address "127.0.0.1" and port number 6668
    let info = OscHostInfo::new("OSCQuery Test".to_string(), "127.0.0.1".to_string(), 6668)
        .with_ext_access()
        .with_ext_unit()
        //.with_ext_value()
        .with_ext_description()
        .with_ext_range();

    let mut root = OSCNode::root(Some(Box::new(info)));

    let par1 = OscQueryParameter::new("/group/test".to_string(), OscType::Float(1f32))
        .with_description("My First Description".to_string())
        .with_min_max(0f32, 10f32)
        .with_access(OSCAccess::ReadWrite)
        .with_unit(OSCUnit::Distance(crate::OSCDistance::Centimeter));

    let par2 = OscQueryParameter::new("/group/test2".to_string(), OscType::Float(1f32))
        .with_description("My First Description".to_string())
        .with_min_max(0f32, 10f32)
        .with_access(OSCAccess::ReadWrite)
        .with_unit(OSCUnit::Distance(crate::OSCDistance::Meter));

    let par3 = OscQueryParameter::new("/group/test/subtest".to_string(), OscType::Float(1f32))
        .with_description("My First Description".to_string())
        .with_min_max(0f32, 10f32)
        .with_access(OSCAccess::ReadWrite)
        .with_unit(OSCUnit::Distance(crate::OSCDistance::Meter));

    // Add the above parameters to the root node
    root.add(par1).unwrap();
    root.add(par2).unwrap();
    root.add(par3).unwrap();

    // Set the IP address and port number for the oscquery service
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    // Run the oscquery service and get the futures for the server and the zeroconf server
    let (x, y) = run_oscquery_service(root, addr).await.unwrap();

    // Create a UDP socket for receiving incoming osc messages and bind it to the address and port number
    let addr_osc: SocketAddr = ([127, 0, 0, 1], 6669).into();
    let sock = tokio::net::UdpSocket::bind(addr_osc).await.unwrap();
    let mut buf = [0; 1024];

    // Loop to receive incoming osc messages
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await.unwrap();
        println!("{:?} bytes received from {:?}", len, addr);

        let len = sock.send_to(&buf[..len], addr).await.unwrap();
        println!("{:?} bytes sent", len);
    }

    let res = tokio::join!(x, y);
    res.0.unwrap();
    res.1.unwrap();
}
