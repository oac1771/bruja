use crate::chain_spec;

use sc_cli::{CliConfiguration, SubstrateCli, ChainSpec};

#[derive(Debug, clap::Args)]
pub struct StartCmd {}

impl SubstrateCli for StartCmd {
    fn impl_name() -> String {
        todo!()
    }

    fn impl_version() -> String {
        todo!()
    }

    fn description() -> String {
        todo!()
    }

    fn author() -> String {
        todo!()
    }

    fn support_url() -> String {
        todo!()
    }

    fn copyright_start_year() -> i32 {
        todo!()
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
        Ok(match id {
            "" | "dev" => Box::new(chain_spec::development_config().unwrap()),
            path => Box::new(chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path)).unwrap()),
        })
    }
}

impl CliConfiguration for StartCmd {
    fn shared_params(&self) -> &sc_cli::SharedParams {
        todo!()
    }
}

impl StartCmd {
    pub fn run(&self) {
        let runner = self.create_runner(self).unwrap();
    }
}
