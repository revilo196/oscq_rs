use crate::OSCUnit;
use rosc::{OscError, OscType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// options how to define a Range in OscQuery
enum OscRangeBounds {
    #[serde(rename = "MIN")]
    Min,
    #[serde(rename = "MAX")]
    Max,
    #[serde(rename = "VALS")]
    Discrete,
}

/// OscQueryParameter describes a single OSC Value for use in the OSCQuery Protocol
/// the OSCQuery Protocol adds a more detailed description to the OSC Value
#[derive(Debug)]
pub struct OscQueryParameter {
    description: String,                          // short description of the Value
    address: String,                              // OSC address/path of the value
    value: OscType,                               // value&type description
    access: Option<OSCAccess>,                    // access rights description
    range: Option<BTreeMap<OscRangeBounds, f32>>, // value range description
    unit: Option<OSCUnit>,                        // unit description
}

impl OscQueryParameter {
    /// Create a new `OscQueryParameter` with the given `address` and `value`.
    /// ```
    /// use oscq_rs::OscQueryParameter;
    /// let parameter = OscQueryParameter::new("/test/param".to_string(), rosc::OscType::Int(42));
    /// println!("{:?}",parameter);
    /// ```
    pub fn new(addr: String, value: OscType) -> Self {
        Self {
            description: "".to_string(),
            address: addr,
            value,
            access: None,
            range: None,
            unit: None,
        }
    }

    /// Set the `access` for the `OscQueryParameter` and return a new `OscQueryParameter` instance.
    /// ```
    /// use oscq_rs::{OscQueryParameter,OSCAccess};
    /// let parameter = OscQueryParameter::new("/test/param".to_string(), rosc::OscType::Int(42))
    ///                 .with_access(OSCAccess::Read);
    /// println!("{:?}",parameter);
    /// ```
    pub fn with_access(mut self, access: OSCAccess) -> Self {
        self.access = Some(access);
        self
    }

    /// Set the `unit` for the `OscQueryParameter` and return a new `OscQueryParameter` instance.
    /// ```
    /// use oscq_rs::{OscQueryParameter,OSCUnit,OSCDistance};
    /// let parameter = OscQueryParameter::new("/test/param".to_string(), rosc::OscType::Int(42))
    ///                 .with_unit(OSCUnit::Distance(OSCDistance::Meter));
    /// println!("{:?}",parameter);
    /// ```
    pub fn with_unit(mut self, unit: OSCUnit) -> Self {
        self.unit = Some(unit);
        self
    }

    /// Set the `min` and `max` values for the `range` of the `OscQueryParameter` and return a new `OscQueryParameter` instance.
    /// ```
    /// use oscq_rs::OscQueryParameter;
    /// let parameter = OscQueryParameter::new("/test/param".to_string(), rosc::OscType::Int(42))
    ///                 .with_min_max(0.0, 100.0);;
    /// println!("{:?}",parameter);
    /// ```
    pub fn with_min_max(mut self, min: f32, max: f32) -> Self {
        let mut range = BTreeMap::new();
        range.insert(OscRangeBounds::Min, min);
        range.insert(OscRangeBounds::Max, max);
        self.range = Some(range);
        self
    }

    /// Set the `description` for the `OscQueryParameter` and return a new `OscQueryParameter` instance.
    /// ```
    /// use oscq_rs::OscQueryParameter;
    /// let parameter = OscQueryParameter::new("/test/param".to_string(), rosc::OscType::Int(42))
    ///                 .with_description("This is a test parameter".to_string());
    /// println!("{:?}",parameter);
    /// ```
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
/// description of the OSCQuery Access Rules
pub enum OSCAccess {
    NoAcces = 0,
    Read = 1,
    Write = 2,
    ReadWrite = 3,
}

#[derive(Debug, Serialize, Deserialize)]
/// OSCQuery Host Information Section
pub struct OscHostInfo {
    #[serde(rename = "NAME")]
    name: String, // OSC device name
    #[serde(rename = "OSC_IP")]
    osc_ip: String, // IP of the OSC device
    #[serde(rename = "OSC_PORT")]
    osc_port: u16, // Port of the OSC device
    #[serde(rename = "OSC_TRANSPORT")]
    osc_trans: String, // IP of the OSC device
    #[serde(rename = "EXTENSIONS")]
    extension: OscHostInfoExtension, // Information of the supported OSCQuery extension
}

impl OscHostInfo {
    /// create new HostInformation
    pub fn new(device_name: String, osc_ip: String, osc_port: u16) -> Self {
        OscHostInfo {
            name: device_name,
            osc_ip,
            osc_port,
            osc_trans: "UDP".to_string(),
            extension: OscHostInfoExtension::default(),
        }
    }
    /// enable access extension
    pub fn with_ext_access(mut self) -> Self {
        self.extension.access = true;
        self
    }
    /// enable clipmode extension (WIP)
    pub fn with_ext_clipmode(mut self) -> Self {
        self.extension.clipmode = true;
        self
    }
    /// enable critical extension (WIP)
    pub fn with_ext_critical(mut self) -> Self {
        self.extension.critical = true;
        self
    }
    /// enable description
    pub fn with_ext_description(mut self) -> Self {
        self.extension.description = true;
        self
    }
    /// enable extended type extension (WIP)
    pub fn with_ext_extended_type(mut self) -> Self {
        self.extension.extended_type = true;
        self
    }
    /// enable listen extension (WIP)
    pub fn with_ext_listen(mut self) -> Self {
        self.extension.listen = true;
        self
    }
    /// enable path changed extension (WIP)
    pub fn with_ext_path_changed(mut self) -> Self {
        self.extension.path_changed = true;
        self
    }
    /// enable range extension
    pub fn with_ext_range(mut self) -> Self {
        self.extension.range = true;
        self
    }
    /// enable tags extension (WIP)
    pub fn with_ext_tags(mut self) -> Self {
        self.extension.tags = true;
        self
    }
    /// enable unit extension
    pub fn with_ext_unit(mut self) -> Self {
        self.extension.unit = true;
        self
    }
    /// enable value extension
    pub fn with_ext_value(mut self) -> Self {
        self.extension.value = true;
        self
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
/// Collection of the Available and Unavailable OSC Extension
struct OscHostInfoExtension {
    #[serde(rename = "ACCESS")]
    access: bool,
    #[serde(rename = "VALUE")]
    value: bool,
    #[serde(rename = "RANGE")]
    range: bool,
    #[serde(rename = "DESCRIPTION")]
    description: bool,
    #[serde(rename = "TAGS")]
    tags: bool,
    #[serde(rename = "EXTENDED_TYPE")]
    extended_type: bool,
    #[serde(rename = "UNIT")]
    unit: bool,
    #[serde(rename = "CRITICAL")]
    critical: bool,
    #[serde(rename = "CLIPMODE")]
    clipmode: bool,
    #[serde(rename = "LISTEN")]
    listen: bool,
    #[serde(rename = "PATH_CHANGED")]
    path_changed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
/// Representation of a Node in the OSCQuery tree data structure
/// This struct can be serialized into a JSON string.
/// This JSON then will follow the OSCQuery protocol
pub struct OSCNode {
    #[serde(rename = "DESCRIPTION")]
    description: String,
    #[serde(rename = "FULL_PATH")]
    full_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ACCESS")]
    access: Option<OSCAccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "CONTENTS")]
    contents: Option<BTreeMap<String, OSCNode>>,
    #[serde(serialize_with = "osc_type_serialize")]
    #[serde(deserialize_with = "osc_type_deserialize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "TYPE")]
    osc_type: Option<Vec<OscType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "VALUE")]
    #[serde(serialize_with = "osc_value_serialize")]
    #[serde(deserialize_with = "osc_value_deserialize")]
    value: Option<Vec<OscType>>,
    #[serde(rename = "RANGE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<Vec<BTreeMap<OscRangeBounds, f32>>>,
    #[serde(rename = "UNIT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<Vec<OSCUnit>>,
    #[serde(rename = "HOST_INFO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    host_info: Option<Box<OscHostInfo>>,
}

impl OSCNode {
    /// create a osc root node
    /// option to provide Host Information
    pub fn root(host_info: Option<Box<OscHostInfo>>) -> Self {
        let full_path = "/".to_string();
        let description = "".to_string();
        let access = OSCAccess::NoAcces;

        Self {
            description,
            full_path,
            access: Some(access),
            contents: None,
            osc_type: None,
            value: None,
            range: None,
            unit: None,
            host_info,
        }
    }

    /// walks down the tree, creates new nodes if necessary,
    /// and insert the Parament at the Position defined by its address
    fn add_recursion(
        &mut self,
        parameter: OscQueryParameter,
        mut addr: VecDeque<String>,
    ) -> Result<(), OscError> {
        // if there is some address part left
        if let Some(key) = addr.pop_front() {
            //Node does not have any content yet
            if self.contents.is_none() {
                self.contents = Some(BTreeMap::new())
            }

            //Node does not have the next address key jet
            if !self.contents.as_ref().unwrap().contains_key(&key) {
                let next_addr = self.full_path.to_string() + &key;
                // new empty node for now
                let value = OSCNode {
                    description: "".to_string(),
                    full_path: next_addr,
                    access: Some(OSCAccess::NoAcces),
                    contents: None,
                    osc_type: None,
                    value: None,
                    range: None,
                    unit: None,
                    host_info: None,
                };
                self.contents
                    .as_mut()
                    .unwrap()
                    .insert(key.to_string(), value);
            }

            // run recursion on the next node in the tree
            self.contents
                .as_mut()
                .unwrap()
                .get_mut(&key)
                .unwrap()
                .add_recursion(parameter, addr)?
        } else {
            // base case insert into this address
            self.description = parameter.description;
            self.access = parameter.access;
            self.full_path = parameter.address;

            // work with all the optional values...
            // TYPE
            match &mut self.osc_type {
                Some(v) => v.push(parameter.value.clone()),
                None => self.osc_type = Some(vec![parameter.value.clone()]),
            }
            // UNIT
            if let Some(unit) = parameter.unit {
                match &mut self.unit {
                    Some(v) => v.push(unit),
                    None => self.unit = Some(vec![unit]),
                }
            }
            // RANGE
            if let Some(range) = parameter.range {
                match &mut self.range {
                    Some(v) => v.push(range),
                    None => self.range = Some(vec![range]),
                }
            }
            // VALUE
            match &mut self.value {
                Some(v) => v.push(parameter.value),
                None => self.value = Some(vec![parameter.value]),
            }
        }
        Ok(())
    }

    /// add a new Parameter to the root node
    pub fn add(&mut self, parameter: OscQueryParameter) -> Result<(), OscError> {
        let mut addr: VecDeque<String> = parameter
            .address
            .split('/')
            .map(|s| s.to_string())
            .collect();
        addr.pop_front();
        self.add_recursion(parameter, addr)
    }

    // get a subnode using a OSC path
    pub fn get(&self, path: String) -> Result<&OSCNode, OscError> {
        let path_s = path.clone();
        let mut addr: VecDeque<_> = path_s.split('/').collect();

        if let Some(_current_node) = addr.pop_front() {
            // this is the current Node
            if let Some(next_node) = addr.front() {
                if next_node.is_empty() {
                    return Ok(self);
                }

                let node = self
                    .contents
                    .as_ref()
                    .ok_or(OscError::BadAddress(path.clone()))?
                    .get(*next_node)
                    .ok_or(OscError::BadAddress(path))?;
                let v: Vec<_> = addr.into();
                node.get(v.join("/"))
            } else {
                // the current node is the final node
                Ok(self)
            }
        } else {
            Ok(self)
        }
    }
}

/// convert a Vec of OscType to its OSC type string("f", "i", "fff" ...)
fn osc_type_serialize<S: Serializer>(
    addr: &Option<Vec<OscType>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut s = String::new();
    match addr {
        Some(v) => {
            for osc_type in v {
                match osc_type {
                    OscType::Int(_) => s += "i",
                    OscType::Float(_) => s += "f",
                    OscType::String(_) => s += "s",
                    OscType::Blob(_) => s += "b",
                    OscType::Time(_) => s += "t",
                    OscType::Long(_) => s += "l",
                    OscType::Double(_) => s += "d",
                    OscType::Char(_) => s += "c",
                    OscType::Color(_) => s += "r",
                    OscType::Midi(_) => s += "m",
                    OscType::Bool(_) => s += "T",
                    OscType::Array(_) => todo!(),
                    OscType::Nil => s += "N",
                    OscType::Inf => s += "I",
                }
            }
            serializer.serialize_str(s.as_str())
        }
        None => serializer.serialize_none(),
    }
}

/// Convert a OSC type string("i", "f", "fff"...) into a Vec of OscType
fn osc_type_deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Vec<OscType>>, D::Error> {
    let s = String::deserialize(deserializer)?;
    if !s.is_empty() {
        let mut vec = Vec::new();
        for char in s.chars() {
            match char {
                'i' => vec.push(OscType::Int(0i32)),
                'f' => vec.push(OscType::Float(0f32)),
                's' => vec.push(OscType::String("".to_string())),
                'b' => vec.push(OscType::Blob(Vec::new())),
                't' => vec.push(OscType::Time(rosc::OscTime::from((2_208_988_800, 0)))),
                'l' => vec.push(OscType::Long(0i64)),
                'd' => vec.push(OscType::Double(0f64)),
                'c' => vec.push(OscType::Char(' ')),
                'r' => vec.push(OscType::Color(rosc::OscColor {
                    red: 0,
                    green: 0,
                    blue: 0,
                    alpha: 0,
                })),
                'm' => vec.push(OscType::Midi(rosc::OscMidiMessage {
                    port: 0,
                    status: 0,
                    data1: 0,
                    data2: 0,
                })),
                'T' => vec.push(OscType::Bool(true)),
                'N' => vec.push(OscType::Nil),
                'I' => vec.push(OscType::Inf),

                _ => {
                    return Err(serde::de::Error::unknown_variant(
                        char.to_string().as_str(),
                        &[
                            "i", "f", "s", "b", "t", "l", "d", "c", "r", "m", "T", "N", "I",
                        ],
                    ))
                }
            }
        }
        Ok(Some(vec))
    } else {
        Err(serde::de::Error::custom("Invalid OSC Type"))
    }
}

use serde::ser::SerializeSeq;
/// convert a Vec of OscType to its OSC type string("f", "i", "fff" ...)
fn osc_value_serialize<S: Serializer>(
    addr: &Option<Vec<OscType>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match addr {
        Some(values) => {
            let mut seq = serializer.serialize_seq(Some(values.len()))?;
            for val in values {
                match val {
                    OscType::Int(i) => seq.serialize_element(i)?,
                    OscType::Float(f) => seq.serialize_element(f)?,
                    OscType::String(g) => seq.serialize_element(g)?,
                    OscType::Blob(b) => seq.serialize_element(b)?,
                    OscType::Time(_t) => todo!(),
                    OscType::Long(l) => seq.serialize_element(l)?,
                    OscType::Double(d) => seq.serialize_element(d)?,
                    OscType::Char(c) => seq.serialize_element(c)?,
                    OscType::Color(_r) => todo!(),
                    OscType::Midi(_m) => todo!(),
                    OscType::Bool(b) => seq.serialize_element(b)?,
                    OscType::Array(_a) => todo!(),
                    OscType::Nil => todo!(),
                    OscType::Inf => todo!(),
                }
            }
            seq.end()
        }
        None => serializer.serialize_none(),
    }
}

fn osc_value_deserialize<'de, D: Deserializer<'de>>(
    _deserializer: D,
) -> Result<Option<Vec<OscType>>, D::Error> {
    // problem that the this Deserializer depends on the type value that is Deserializer separately
    todo!()
}

///-----------------------------------
/// Some Test left from development --
///-----------------------------------

#[test]
fn serialize_osc_node() {
    let mut range = BTreeMap::new();
    range.insert(OscRangeBounds::Min, 100.0f32);
    range.insert(OscRangeBounds::Max, 200.0f32);

    let node = OSCNode {
        description: "A test node".to_string(),
        access: Some(OSCAccess::ReadWrite),
        full_path: "/test/node".to_string(),
        contents: Some({
            let mut contents = BTreeMap::new();
            contents.insert(
                "child_node".to_string(),
                OSCNode {
                    description: "A child node".to_string(),
                    full_path: "/test/node/child_node".to_string(),
                    access: Some(OSCAccess::ReadWrite),
                    contents: None,
                    osc_type: Some(vec![OscType::Int(0)]),
                    value: Some(vec![OscType::Int(123)]),
                    range: Some(vec![range]),
                    unit: None,
                    host_info: None,
                },
            );
            contents
        }),
        osc_type: Some(vec![OscType::Float(0f32), OscType::Float(0f32)]),
        value: Some(vec![OscType::Float(3.1234), OscType::Float(2.7182)]),
        range: None,
        unit: Some(vec![
            OSCUnit::Distance(crate::OSCDistance::Meter),
            OSCUnit::Speed(crate::OSCSpeed::KilometersPerHour),
        ]),
        host_info: None,
    };

    let serialized = serde_json::to_string(&node).unwrap();
    println!("{}", serialized);
    assert_eq!(
        serialized,
        r#"{"DESCRIPTION":"A test node","FULL_PATH":"/test/node","ACCESS":3,"CONTENTS":{"child_node":{"DESCRIPTION":"A child node","FULL_PATH":"/test/node/child_node","ACCESS":3,"TYPE":"i","VALUE":[123],"RANGE":[{"MIN":100.0,"MAX":200.0}]}},"TYPE":"ff","VALUE":[3.1234,2.7182],"UNIT":["distance.m","speed.km/h"]}"#
    );
}

#[test]
fn add_parameters() {
    let ext = OscHostInfoExtension {
        access: true,
        value: true,
        range: true,
        description: true,
        tags: false,
        extended_type: false,
        unit: false,
        critical: false,
        clipmode: false,
        listen: false,
        path_changed: false,
    };

    let info = OscHostInfo {
        name: "OSCQuery Test".to_string(),
        osc_ip: "127.0.0.1".to_string(),
        osc_port: 6666,
        extension: ext,
        osc_trans: "UDP".to_string(),
    };

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

    let serialized = serde_json::to_string(&root).unwrap();
    println!("{}\n\n", serialized);

    let serialized = serde_json::to_string(root.get("/group/test".to_string()).unwrap()).unwrap();

    println!("{}\n\n", serialized);
}
