use sc_cli::{CliConfiguration, SubstrateCli};

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

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_cli::ChainSpec>, String> {
        todo!()
    }
}

impl CliConfiguration for StartCmd {
    fn shared_params(&self) -> &sc_cli::SharedParams {
        todo!()
    }
}

impl StartCmd {
    pub fn run(&self) {
        // let run_cmd = RunCmd{};
        let runner = self.create_runner(self).unwrap();
        // runner.run_node_until_exit(|config| async move {

        // });
    }
}
