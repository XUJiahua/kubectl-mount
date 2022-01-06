use k8s_openapi::api::core::v1::Pod;
use futures::{StreamExt, TryStreamExt};

use kube::{
    api::{Api, AttachParams, DeleteParams, ListParams, PostParams, ResourceExt, WatchEvent},
    Client,
};
use log::{debug, info};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = env ! ("CARGO_PKG_NAME"), about = env ! ("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    #[structopt(default_value = "default", long, short)]
    namespace: String,
    #[structopt(default_value = "mount-agent", long)]
    pod_name: String,
    #[structopt(default_value = "alpine", long)]
    image_name: String,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init log
    std::env::set_var("RUST_LOG", "info,kube=debug");
    env_logger::init();

    // parse flags
    let opt = Opt::from_args();
    debug!("{:?}", opt);

    info!("version: {}", env!("CARGO_PKG_VERSION"));

    let client = Client::try_default().await?;
    let namespace = opt.namespace;
    let pod_name = opt.pod_name;
    let image_name = opt.image_name;

    let p: Pod = serde_json::from_value(serde_json::json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": { "name": pod_name },
        "spec": {
            "containers": [{
                "name": pod_name,
                "image": image_name,
                // Do nothing
                "command": ["tail", "-f", "/dev/null"],
            }],
        }
    }))?;

    debug!("{:?}", p);

    let pods: Api<Pod> = Api::namespaced(client, &namespace);
    // Stop on error including a pod already exists or is still being deleted.
    pods.create(&PostParams::default(), &p).await?;

    info!("pod created");

    // Wait until the pod is running, otherwise we get 500 error.
    let lp = ListParams::default().fields(format!("metadata.name={}",&pod_name).as_str()).timeout(100);
    let mut stream = pods.watch(&lp, "0").await?.boxed();
    while let Some(status) = stream.try_next().await? {
        match status {
            WatchEvent::Added(o) => {
                info!("Added {}", o.name());
            }
            WatchEvent::Modified(o) => {
                let s = o.status.as_ref().expect("status exists on pod");
                if s.phase.clone().unwrap_or_default() == "Running" {
                    info!("Ready to attach to {}", o.name());
                    break;
                }
            }
            _ => {}
        }
    }

    info!("attaching to pod");

    // Do an interactive exec to a blog pod with the `sh` command
    let ap = AttachParams::interactive_tty();
    let mut attached = pods.exec(&pod_name, vec!["sh"], &ap).await?;

    // The received streams from `AttachedProcess`
    let mut stdin_writer = attached.stdin().unwrap();
    let mut stdout_reader = attached.stdout().unwrap();

    // > For interactive uses, it is recommended to spawn a thread dedicated to user input and use blocking IO directly in that thread.
    // > https://docs.rs/tokio/0.2.24/tokio/io/fn.stdin.html
    let mut stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    // pipe current stdin to the stdin writer from ws
    tokio::spawn(async move {
        tokio::io::copy(&mut stdin, &mut stdin_writer).await.unwrap();
    });
    // pipe stdout from ws to current stdout
    tokio::spawn(async move {
        tokio::io::copy(&mut stdout_reader, &mut stdout).await.unwrap();
    });
    // When done, type `exit\n` to end it, so the pod is deleted.
    let status = attached.await;
    println!("{:?}", status);

    // Delete it
    println!("deleting");
    pods.delete(&pod_name, &DeleteParams::default())
        .await?
        .map_left(|pdel| {
            assert_eq!(pdel.name(), pod_name.as_str());
        });

    Ok(())
}
