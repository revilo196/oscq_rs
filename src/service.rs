use crate::OSCNode;

use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request, Response};
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

struct OscQueryStatic {
    root: Arc<OSCNode>,
}

impl Service<Request<IncomingBody>> for OscQueryStatic {
    type Response = Response<String>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, req: Request<IncomingBody>) -> Self::Future {
        fn mk_response(s: String) -> Result<Response<String>, hyper::Error> {
            println!("{}",s);
            Ok(Response::builder().body(s).unwrap())
        }

        let res = mk_response(
            serde_json::to_string(&self.root.get(req.uri().path().to_string()).unwrap()).unwrap(),
        );

        Box::pin(async { res })
    }
}

use tokio::net::TcpListener;
use hyper::server::conn::http1;

pub async fn run_oscquery_service(root: OSCNode, address : SocketAddr) -> tokio::io::Result<()> {
    let arc_root=  Arc::new(root);
    let listener = TcpListener::bind(address).await?;

    tokio::task::spawn(async move { 
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let service = OscQueryStatic { root: arc_root.clone()  };
            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(stream, service)
                    .await
                {
                    println!("Failed to serve connection: {:?}", err);
                }
            });  
        
        }
    });
    Ok(())
}


#[tokio::test]
async fn test_service() {
    use crate::{OSCAccess, OSCUnit, OscHostInfo, OscQueryParameter};
    use hyper::server::conn::http1;
    use rosc::OscType;
    use std::net::SocketAddr;
 

    let info = OscHostInfo::new("OSCQuery Test".to_string(), "127.0.0.1".to_string(), 6666)
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

    root.add(par1).unwrap();
    root.add(par2).unwrap();
    root.add(par3).unwrap();

    let arc_root=  Arc::new(root);

    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    let listener = TcpListener::bind(addr).await.unwrap();



    let (stream, _) = listener.accept().await.unwrap();
    let service = OscQueryStatic { root: arc_root.clone()  };

    tokio::task::spawn(async move {
    if let Err(err) = http1::Builder::new()
        .serve_connection(stream, service)
        .await
    {
        println!("Failed to serve connection: {:?}", err);
    }
    });  

    
}
