use crate::{Fields, Filter, Sortby};
use geojson::Geometry;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// The core parameters for STAC search are defined by OAFeat, and STAC adds a few parameters for convenience.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Search {
    /// The maximum number of results to return (page size).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Requested bounding box.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<Vec<f64>>,

    /// Single date+time, or a range ('/' separator), formatted to [RFC 3339,
    /// section 5.6](https://tools.ietf.org/html/rfc3339#section-5.6).
    ///
    /// Use double dots `..` for open date ranges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime: Option<String>,

    /// Searches items by performing intersection between their geometry and provided GeoJSON geometry.
    ///
    /// All GeoJSON geometry types must be supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intersects: Option<Geometry>,

    /// Array of Item ids to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,

    /// Array of one or more Collection IDs that each matching Item must be in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collections: Option<Vec<String>>,

    /// Include/exclude fields from item collections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Fields>,

    /// Fields by which to sort results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sortby: Option<Vec<Sortby>>,

    /// Recommended to not be passed, but server must only accept
    /// <http://www.opengis.net/def/crs/OGC/1.3/CRS84> as a valid value, may
    /// reject any others
    #[serde(skip_serializing_if = "Option::is_none", rename = "filter-crs")]
    pub filter_crs: Option<String>,

    /// CQL2 filter expression.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,

    /// Additional filtering based on properties.
    ///
    /// It is recommended to use the filter extension instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Map<String, Value>>,

    /// Additional fields.
    #[serde(flatten)]
    pub additional_fields: Map<String, Value>,
}

impl Search {
    /// Creates a new, default, empty search.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stac_api::Search;
    /// let search = Search::new();
    /// ```
    pub fn new() -> Search {
        Default::default()
    }

    /// Adds a collection id to this search.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stac_api::Search;
    /// let search = Search::new().collection("sentinel-2-l1a");
    /// ```
    pub fn collection(mut self, id: impl ToString) -> Search {
        let mut collections = self.collections.unwrap_or_default();
        collections.push(id.to_string());
        self.collections = Some(collections);
        self
    }

    /// Sets this search's limit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stac_api::Search;
    /// let search = Search::new().limit(42);
    /// ```
    pub fn limit(mut self, limit: impl Into<Option<u64>>) -> Search {
        self.limit = limit.into();
        self
    }

    /// Sets this search's fields.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stac_api::Search;
    /// let fields = "+foo,-bar".parse().unwrap();
    /// let search = Search::new().fields(Some(fields));
    /// ```
    pub fn fields(mut self, fields: impl Into<Option<Fields>>) -> Search {
        self.fields = fields.into();
        self
    }
}
