/// Defines the types and auxilary functions to represent and work with
/// RADIUS attributes.
///
/// According to the RFC 2865 the format of the RADIUS attribute is:
///
///    0                   1                   2
///    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-
///   |     Type      |    Length     |  Value ...
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-
///
/// In addition vendor specific attributes are represented as:
///
///    0                   1                   2                   3
///    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |     Type      |  Length       |            Vendor-Id
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///   |     Vendor-Id (cont)          |  String...
///   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-
///
/// in general and the recomended structure is:
///
///  0                   1                   2                   3
///  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  |     Type      |  Length       |            Vendor-Id
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  | Vendor-Id (cont)           | Vendor type   | Vendor length    |
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///  |    Attribute-Specific...
///  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-

#[derive(Debug)]
pub struct Attribute {
    attr_type: u8,
    length: u8,
    vendor: Option<Vendor>,
    value: [u8; 255],
}

impl Attribute {
    /// Creates new `Attribute`.
    ///
    /// Initially the attribute will be created with zero length and empty value.
    /// If the `vendor` is not `None` `id` should be equal to `26` according to
    /// RFC 2865.
    ///
    /// # Examples
    ///
    /// ```
    /// use radius::attribute::{Attribute, Vendor};
    ///
    /// let attr: Attribute = Attribute::new(1, None);
    /// ```
    #[inline]
    pub fn new(id: u8, vendor: Option<Vendor>) -> Attribute {
        Attribute {
            attr_type: id,
            length: 0,
            vendor: vendor,
            value: [0; 255],
        }
    }
}

#[derive(Debug)]
pub struct Vendor {
    vendor_id: u32,
    vendor_type: u8,
    length: u8,
}

impl Vendor {
    /// Creates new `vendor` part of a RADIUS attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use radius::attribute::{Attribute, Vendor};
    ///
    /// let vendor: Vendor = Vendor::new(10415, 1);
    /// let attr: Attribute = Attribute::new(26, Some(vendor));
    /// ```
    #[inline]
    pub fn new(id: u32, vendor_type: u8) -> Vendor {
        Vendor {
            vendor_id: id,
            vendor_type: vendor_type,
            length: 0,
        }
    }
}
