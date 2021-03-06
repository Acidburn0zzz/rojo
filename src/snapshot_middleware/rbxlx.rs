use crate::{
    snapshot::{InstanceContext, InstanceMetadata, InstanceSnapshot},
    vfs::{Vfs, VfsEntry, VfsFetcher},
};

use super::{
    middleware::{SnapshotInstanceResult, SnapshotMiddleware},
    util::match_file_name,
};

pub struct SnapshotRbxlx;

impl SnapshotMiddleware for SnapshotRbxlx {
    fn from_vfs<F: VfsFetcher>(
        context: &InstanceContext,
        vfs: &Vfs<F>,
        entry: &VfsEntry,
    ) -> SnapshotInstanceResult {
        if entry.is_directory() {
            return Ok(None);
        }

        let instance_name = match match_file_name(entry.path(), ".rbxlx") {
            Some(name) => name,
            None => return Ok(None),
        };

        let options = rbx_xml::DecodeOptions::new()
            .property_behavior(rbx_xml::DecodePropertyBehavior::ReadUnknown);

        let temp_tree = rbx_xml::from_reader(entry.contents(vfs)?.as_slice(), options)
            .expect("TODO: Handle rbx_xml errors");

        let root_id = temp_tree.get_root_id();

        let snapshot = InstanceSnapshot::from_tree(&temp_tree, root_id)
            .name(instance_name)
            .metadata(
                InstanceMetadata::new()
                    .instigating_source(entry.path())
                    .relevant_paths(vec![entry.path().to_path_buf()])
                    .context(context),
            );

        Ok(Some(snapshot))
    }
}
