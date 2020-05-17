use std::{env, fs};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct EcsContainerMetaData {
    #[serde(rename(deserialize="Cluster"))]
    pub cluster: String,

    #[serde(rename(deserialize="TaskARN"))]
    pub task_arn: String,

    #[serde(rename(deserialize="ContainerID"))]
    pub container_id: String,

    #[serde(rename(deserialize="ContainerName"))]
    pub container_name: String,

    #[serde(rename(deserialize="ImageID"))]
    pub image_id: String,

    #[serde(rename(deserialize="ImageName"))]
    pub image_name: String,
}

trait EcsRepository {
    fn new() -> EcsLocalRepository;
    fn from_mock(file_path: PathBuf) -> EcsLocalRepository;
    fn get(&self) -> Result<EcsContainerMetaData, String>;
}

struct EcsLocalRepository {
    path: PathBuf
}

impl EcsRepository for EcsLocalRepository {
    fn new() -> EcsLocalRepository {
        let env_variable = "ECS_CONTAINER_METADATA_FILE";
        let file_path = env::var(env_variable).map(|path| PathBuf::from(path)).expect("can't read resource file");

        EcsLocalRepository {
            path: file_path
        }
    }

    fn from_mock(file_path: PathBuf) -> EcsLocalRepository {
       EcsLocalRepository {
           path: file_path
       }
    }

    fn get(&self) -> Result<EcsContainerMetaData, String> {
        fn extract(json_str: &str) -> Result<EcsContainerMetaData, String> {
            match serde_json::from_str(json_str) {
                Ok(val) => Ok(val),
                Err(e) => return Err(e.to_string())
            }
        }

        fs::read_to_string(&self.path).map_err(|e| e.to_string()).and_then(|json| extract(&json) )
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs_info::{EcsLocalRepository, EcsRepository, EcsContainerMetaData};
    use std::env;

    #[test]
    fn it_works() {
        fn to_string(entity: &EcsContainerMetaData) -> Result<String, String> {
            match serde_json::to_string_pretty(entity) {
                Ok(val) => Ok(val),
                Err(e) => return Err(e.to_string())
            }
        }

        let resource_dir = env::current_dir().map(|dir| dir.join("resource/ecs_meta.json")).expect("can't get resource file.");
        let res: Result<String, String> = EcsLocalRepository::from_mock(resource_dir)
            .get()
            .and_then(|entity| to_string(&entity));

        match res {
            Ok(val) => println!("{}", val),
            Err(e) => println!("{}", e)
        }
    }
}