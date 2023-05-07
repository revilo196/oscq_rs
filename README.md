# oscq_rs

OSC (Open Sound Control) is a network communication protocol used for real-time musical performances, multimedia systems, and other applications that involve the exchange of time-critical data. It provides a standardized way of sending messages and data between software and hardware components in a distributed system. OSCQuery is an extension to the OSC protocol that allows for the dynamic discovery of available OSC endpoints and their properties. See this introduction [OSCQuery](https://vdmx.vidvox.net/blog/oscquery)

The oscq_rs library is a Rust implementation of the OSCQuery protocol that provides an easy way to generate OSCQuery descriptions from an list of endpoint descriptions and answer OSCQuery requests. It allows developers to quickly build OSC-enabled applications that can dynamically discover and controlled via OSC on the network.


## Getting Started
### Installation

To use oscq_rs in your Rust project, you can add it to your Cargo.toml file:

```toml
[dependencies]
oscq_rs = "0.1.0"
rosc = "0.10.0"
serde_json = "1.0.95"
```
### Usage

Here's an example of how to create an OSCNode tree, serialize it, and serve it as an OSCQuery server using the integrated HTTP service:

```rust

use oscq_rs::{OscHostInfo, OscQueryParameter, OSCAccess, OSCUnit, OSCNode};
use rosc::OscType;
use std::net::SocketAddr;

fn main() {
    let info = OscHostInfo::new("OSCQuery Test".to_string(),  "127.0.0.1".to_string(), 6666)
        .with_ext_access()
        .with_ext_unit()
        .with_ext_value()
        .with_ext_description()
        .with_ext_range();

    let mut root = OSCNode::root(Some(Box::new(info)));

    let par1 = OscQueryParameter::new(
        "/group/test".to_string(),
        OscType::Float(1f32),
    )
    .with_description("My First Description".to_string())
    .with_min_max(0f32, 10f32)
    .with_access(OSCAccess::ReadWrite)
    .with_unit(OSCUnit::Distance(crate::OSCDistance::Centimeter));

    let par2 = OscQueryParameter::new(
        "/group/test2".to_string(),
        OscType::Float(1f32),
    )
    .with_description("My Second Description".to_string())
    .with_min_max(0f32, 10f32)
    .with_access(OSCAccess::ReadWrite)
    .with_unit(OSCUnit::Distance(crate::OSCDistance::Meter));

    root.add(par1).unwrap();
    root.add(par2).unwrap();

    // example output
    let serialized_tree = serde_json::to_string(&root).unwrap();
    println!("{}", serialized_tree);


    // Set the IP address and port number for the oscquery service
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    // Run the oscquery service and get the futures for the server and the zeroconf server
    spawn_oscquery_service(root, addr);

    loop {
        // ... 
        // the oscquery service will run in the background
    }
}
```

This creates an OSCNode tree with two endpoints, /group/test and /group/test2, both of which are of type Float. The tree is then served as an OSCQuery server using the integrated HTTP service provided by oscq_rs::serve_oscquery function. In this example, the server listens on http://127.0.0.1:3000 for incoming OSCQuery requests.

Note that in this example we've only added Float endpoints, but oscq_rs supports other types as well, such as Int, Bool, and String.

## Limitations: 

- Currently, only float types are supported for OscType parameters.
- Not all OSCQuery EXTENSIONS are supported yet.
- The OSCNode tree data structure is constructed once and can not easily be updated dynamically.
- The library has not yet been extensively tested in a production environment.

## Future Work:
 - Test and Implement all OSC datatypes

 Note that this is not an exhaustive list of limitations and future work, and the development of the library may evolve as new features and improvements are identified.

## Contributing

Contributions are welcome! Please feel free to open issues and pull requests on GitHub. I open for feedback, and any new ideas.

 ## Summary:
 In conclusion, the oscq_rs library provides an easy way to implement OSCQuery in Rust-based projects. Its implementation is relatively easy to understand, making it a good starting point for developers who want to build OSC-enabled applications that can be dynamically discovered and controlled via OSC on the network. While there are some limitations to the library, such as only supporting 'float' types and the inability to easily update the OSCNode tree dynamically, there is potential for future work to address these limitations and add more features to the library. Overall, oscq_rs is a solid tool for building OSCQuery-enabled applications in Rust.

