mod oscquery_types;
mod oscunit;
mod service;

pub use oscquery_types::*;
pub use oscunit::*;
pub use service::*;

pub mod osc {
    pub use rosc::*;
}

#[test]
fn readme_example() {
    use crate::{OSCAccess, OSCUnit, OscHostInfo};
    use rosc::OscType;

    // create the root node with host information
    let mut root = OSCNode::root(Some(Box::new(OscHostInfo::new(
        "My OSC Server".to_string(),
        "127.0.0.1".to_string(),
        9000,
    ))));

    // create an OscQueryParameter for a float type endpoint
    let param1 = OscQueryParameter::new("/endpoint1".to_string(), OscType::Float(0.0))
        .with_access(OSCAccess::ReadWrite)
        .with_unit(OSCUnit::Distance(OSCDistance::Centimeter))
        .with_description("This is endpoint1".to_string())
        .with_min_max(0.0, 100.0);

    // create an OscQueryParameter for a int type endpoint
    let param2 = OscQueryParameter::new("/endpoint2".to_string(), OscType::Float(1.0f32))
        .with_access(OSCAccess::Read)
        .with_description("This is endpoint2".to_string());

    // add the parameters to the root node
    root.add(param1).unwrap();
    root.add(param2).unwrap();

    let json_str = serde_json::to_string(&root).unwrap();
    println!("{}", json_str);
}
