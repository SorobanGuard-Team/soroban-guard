use std::path::PathBuf;

use crate::{
    config::SorobanGuardConfig,
    error::CoreError,
    project::ProjectInfo,
    toolchain::ToolchainInfo,
};

/// All runtime context required to execute the validation pipeline.
#[derive(Debug)]
pub struct ProjectContext {
    pub project_dir: PathBuf,
    pub config: SorobanGuardConfig,
    pub project: ProjectInfo,
    pub toolchain: ToolchainInfo,
}

impl ProjectContext {
    pub fn load(project_dir: PathBuf) -> Result<Self, CoreError> {
        if !project_dir.exists() {
            return Err(CoreError::ProjectNotFound(
                project_dir.display().to_string(),
            ));
        }

        let config = SorobanGuardConfig::load(&project_dir)
            .map_err(|e| CoreError::Config(e.to_string()))?;

        let contract_dir = config
            .project
            .contract_dir
            .clone()
            .unwrap_or_else(|| project_dir.clone());

        let project = ProjectInfo::from_dir(&contract_dir)?;
        let toolchain = ToolchainInfo::detect();

        Ok(Self {
            project_dir,
            config,
            project,
            toolchain,
        })
    }
}
