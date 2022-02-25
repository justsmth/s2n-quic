use netbench::{scenario, Result, Timer};
use s2n_quic::{
    provider::{
        io,
        tls::default::certificate::{IntoCertificate, IntoPrivateKey},
    },
    Connection,
};
use std::{collections::HashSet, sync::Arc};
use structopt::StructOpt;
use tokio::spawn;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    Server::from_args().run().await
}

#[derive(Debug, StructOpt)]
pub struct Server {
    #[structopt(flatten)]
    opts: netbench_driver::Server,

    #[structopt(long)]
    disable_gso: bool,
}

impl Server {
    pub async fn run(&self) -> Result<()> {
        let scenario = self.opts.scenario();
        let trace = self.opts.trace();

        let mut server = self.server()?;

        while let Some(connection) = server.accept().await {
            let scenario = scenario.clone();
            let trace = trace.clone();
            spawn(async move {
                if let Err(error) = handle_connection(connection, scenario, trace).await {
                    eprintln!("error: {:#}", error);
                }
            });
        }

        return Err("server shut down unexpectedly".into());

        async fn handle_connection(
            connection: Connection,
            scenario: Arc<scenario::Server>,
            mut trace: impl netbench::Trace,
        ) -> Result<()> {
            let conn_id = connection.id();
            let server_name = connection.server_name()?.ok_or("missing server name")?;
            let scenario = scenario.on_server_name(&server_name)?;
            let conn =
                netbench::Driver::new(scenario, netbench::s2n_quic::Connection::new(connection));

            let mut checkpoints = HashSet::new();
            let mut timer = netbench::timer::Tokio::default();

            trace.enter(timer.now(), conn_id, 0);
            conn.run(&mut trace, &mut checkpoints, &mut timer).await?;
            trace.exit(timer.now());

            Ok(())
        }
    }

    fn server(&self) -> Result<s2n_quic::Server> {
        let (certificate, private_key) = self.opts.certificate();
        let certificate = certificate.pem.as_str().into_certificate()?;
        let private_key = private_key.pem.as_str().into_private_key()?;

        let tls = s2n_quic::provider::tls::default::Server::builder()
            .with_certificate(certificate, private_key)?
            .with_application_protocols(
                self.opts.application_protocols.iter().map(String::as_bytes),
            )?
            .with_key_logging()?
            .build()?;

        let mut io_builder =
            io::Default::builder().with_receive_address((self.opts.ip, self.opts.port).into())?;

        if self.disable_gso {
            io_builder = io_builder.with_gso_disabled()?;
        }

        let io = io_builder.build()?;

        let server = s2n_quic::Server::builder()
            .with_io(io)?
            .with_tls(tls)?
            .start()
            .unwrap();

        Ok(server)
    }
}