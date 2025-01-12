pub mod actions;
mod node_platform;
mod target_hash;

pub use node_platform::NodePlatform;
pub use target_hash::NodeTargetHash;

use moon_common::Id;
use moon_config::{NodePackageManager, PartialTaskConfig, PlatformType};
use moon_javascript_platform::ScriptParser;
use moon_node_lang::PackageJson;
use std::collections::BTreeMap;

pub fn create_tasks_from_scripts(
    project_id: &str,
    package_json: &mut PackageJson,
    package_manager: NodePackageManager,
) -> miette::Result<BTreeMap<Id, PartialTaskConfig>> {
    let mut parser = ScriptParser::new(project_id, PlatformType::Node, package_manager);

    parser.parse_scripts(package_json)?;
    parser.update_package(package_json)?;

    Ok(parser.tasks)
}

pub fn infer_tasks_from_scripts(
    project_id: &str,
    package_json: &PackageJson,
    package_manager: NodePackageManager,
) -> miette::Result<BTreeMap<Id, PartialTaskConfig>> {
    let mut parser = ScriptParser::new(project_id, PlatformType::Node, package_manager);

    parser.infer_scripts(package_json)?;

    Ok(parser.tasks)
}
