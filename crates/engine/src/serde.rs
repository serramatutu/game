use std::collections::BTreeMap;

use hashbrown::HashMap;
use serde::{Serialize, Serializer};

/// For use with serde's [serialize_with] attribute
pub(crate) fn ordered_map<S, K: Ord + Serialize, V: Serialize>(
    value: &HashMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

/// For use with serde's [skip_serializing_if] attribute
pub(crate) fn is_empty<T>(value: impl IntoIterator<Item = T>) -> bool {
    value.into_iter().next().is_none()
}
