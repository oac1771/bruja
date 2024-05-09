use crate::chain_spec;

use sc_cli::{ChainSpec, CliConfiguration, SharedParams, SubstrateCli};

#[derive(Debug, clap::Args)]
pub struct StartCmd {

    #[command(flatten)]
    shared_params: SharedParams
}

impl SubstrateCli for StartCmd {
    fn impl_name() -> String {
        "Node".into()
    }

	fn impl_version() -> String {
		"0.1.0".into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"".into()
	}

	fn copyright_start_year() -> i32 {
		2024
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
        &self.shared_params
    }
}

impl StartCmd {
    pub fn run(&self) {
        let runner = self.create_runner(self).unwrap();
        println!("foo!")
    }
}
