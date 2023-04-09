# OSCNode and OscQueryParameter

*'OSCNode'* is the basic data structure in the **'oscq_rs'** library that represents an OSC endpoint. It can be a leaf node that represents a single OSC endpoint, or it can be an internal node that represents a group of OSC endpoints. Each *'OSCNode'* has a unique path identifier that begins with /.

*'OscQueryParameter'* is a struct that describes an OSC endpoint. It contains information such as the endpoint's path, data type, range, description, and access permission. It is used to build the OSCNode tree and generate the OSCQuery descriptions.

To create an *'OSCNode'* tree, first create a *root* node using `OSCNode::root(None)` or `OSCNode::root(Some(info))` where info is an OscHostInfo struct that contains the host information such as the name and IP address. Then, create *OscQueryParameter* structs for each endpoint and add them to the root using `root.add(param).unwrap()` method.

Here's an example of creating an *OSCNode* tree with two endpoints:

```rust

use oscq_rs::{OSCNode, OscQueryParameter, OscAccess, OscUnit, OSCDistance};
use rosc::OscType;

// create the root node with host information
let root = OSCNode::root(Some(OscHostInfo::new("My OSC Server".to_string(), "127.0.0.1".to_string(), 9000)));

// create an OscQueryParameter for a float type endpoint
let param1 = OscQueryParameter::new("/endpoint1".to_string(), OscType::Float(0.0))
    .with_access(OscAccess::ReadWrite)
    .with_unit(OscUnit::Distance(OSCDistance::Centimeter))
    .with_description("This is endpoint1".to_string())
    .with_min_max(0.0, 100.0);

// create an OscQueryParameter for a int type endpoint
let param2 = OscQueryParameter::new("/endpoint2".to_string(), OscType::Int(0))
    .with_access(OscAccess::ReadOnly)
    .with_description("This is endpoint2".to_string());

// add the parameters to the root node
root.add(param1).unwrap();
root.add(param2).unwrap();

```

To generate an OSCQuery description from the *OSCNode* tree, use serde_json: `serde_json::to_string(&root).unwrap()`, which returns a JSON string representation of the tree. This JSON string can be used as a response to an OSCQuery GET request.

```rust

let json_str = serde_json::to_string(&root).unwrap();
println!("{}", json_str);
```

This will output the following JSON string:

```json

{
   "DESCRIPTION":"",
   "FULL_PATH":"/",
   "ACCESS":0,
   "CONTENTS":{
      "endpoint1":{
         "DESCRIPTION":"This is endpoint1",
         "FULL_PATH":"/endpoint1",
         "ACCESS":3,
         "TYPE":"f",
         "VALUE":[
            0.0
         ],
         "RANGE":[
            {
               "MIN":0.0,
               "MAX":100.0
            }
         ],
         "UNIT":[
            "distance.cm"
         ]
      },
      "endpoint2":{
         "DESCRIPTION":"This is endpoint2",
         "FULL_PATH":"/endpoint2",
         "ACCESS":1,
         "TYPE":"f",
         "VALUE":[
            1.0
         ]
      }
   },
   "HOST_INFO":{
      "NAME":"My OSC Server",
      "OSC_IP":"127.0.0.1",
      "OSC_PORT":9000,
      "EXTENSIONS":{
         "ACCESS":false,
         "VALUE":false,
         "RANGE":false,
         "DESCRIPTION":false,
         "TAGS":false,
         "EXTENDED_TYPE":false,
         "UNIT":false,
         "CRITICAL":false,
         "CLIPMODE":false,
         "LISTEN":false,
         "PATH_CHANGED":false
      }
   }
}
```
See the [OSCQuery Proposal](https://github.com/Vidvox/OSCQueryProposal) for reference