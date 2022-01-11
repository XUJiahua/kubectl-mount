use std::process::Command;

use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, DeleteParams, ListParams, PostParams, ResourceExt, WatchEvent},
    Client,
};
use log::{debug, info};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = env ! ("CARGO_PKG_NAME"), about = env ! ("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    /// namespace for the Agent Pod to run
    #[structopt(default_value = "default", global = true, long, short)]
    namespace: String,
    /// override the default Agent Pod name
    #[structopt(default_value = "mount-agent", global = true, long)]
    pod_name: String,
    /// the Docker Image to use for the agent Pod
    #[structopt(default_value = "nicolaka/netshoot", long)]
    image: String,
    /// the network timeout (seconds)
    #[structopt(default_value = "100", long)]
    timeout: u32,
    /// Persistent Volume Claim (PVC) name
    #[structopt(long)]
    pvc: String,
    /// mount path in Pod
    #[structopt(default_value = "/opt", long, short)]
    mount_path: String,
    /// by default, mount PVC in readonly mode
    #[structopt(long, short)]
    write: bool,
    // #[structopt(subcommand)]
    // cmd: Option<SubCommand>,
}

// #[derive(Debug, StructOpt)]
// enum SubCommand {
//     Attach,
//     Clean,
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init log
    // std::env::set_var("RUST_LOG", "info,kube=debug");
    env_logger::init();

    // parse flags
    let opt = Opt::from_args();
    debug!("{:?}", opt);
    let namespace = opt.namespace;
    let pod_name = opt.pod_name;
    let image_name = opt.image;
    let pvc = opt.pvc;
    let mount_path = opt.mount_path;
    let readonly = !opt.write;

    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::namespaced(client, &namespace);

    let p: Pod = serde_json::from_value(serde_json::json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": { "name": pod_name },
        "spec": {
            "containers": [{
                "name": pod_name,
                "image": image_name,
                "command": ["tail", "-f", "/dev/null"],
                "volumeMounts":[{
                    "name": "mount-agent-pv",
                    "mountPath": mount_path,
                    "readOnly": readonly,
                }],
            }],
            "volumes": [{
                "name": "mount-agent-pv",
                "persistentVolumeClaim": {
                    "claimName": pvc,
                }
            }],
        }
    }))?;

    debug!("{:?}", p);
    info!("Creating Pod {} ...", &pod_name);
    // Stop on error including a pod already exists or is still being deleted.
    pods.create(&PostParams::default(), &p).await?;
    debug!("pod created");

    info!("Waiting Pod {} ready ...", &pod_name);
    // Wait until the pod is running, otherwise we get 500 error.
    let lp = ListParams::default()
        .fields(format!("metadata.name={}", &pod_name).as_str())
        .timeout(opt.timeout);
    let mut stream = pods.watch(&lp, "0").await?.boxed();
    while let Some(status) = stream.try_next().await? {
        match status {
            WatchEvent::Added(o) => {
                debug!("Added {}", o.name());
            }
            WatchEvent::Modified(o) => {
                let s = o.status.as_ref().expect("status exists on pod");
                if s.phase.clone().unwrap_or_default() == "Running" {
                    debug!("Ready to attach to {}", o.name());
                    break;
                }
            }
            _ => {}
        }
    }

    info!("Attaching to Pod {} ...", &pod_name);
    // kubectl has a better way to handle tty (search TTY.Safe in source), so we just use kubectl.
    // https://sourcegraph.com/github.com/kubernetes/kubernetes@master/-/blob/staging/src/k8s.io/kubectl/pkg/util/term/term.go?L106:14#tab=references

    let command = format!("kubectl exec -it {} -- sh", &pod_name,);
    // TODO: have to compat with multiple platforms
    let mut child = Command::new("sh").arg("-c").arg(&command).spawn()?;

    child.wait()?;

    // // Do an interactive exec to a blog pod with the `sh` command
    // let ap = AttachParams::interactive_tty();
    // let mut attached = pods.exec(&pod_name, vec!["sh"], &ap).await?;
    //
    // // The received streams from `AttachedProcess`
    // let mut stdin_writer = attached.stdin().unwrap();
    // let mut stdout_reader = attached.stdout().unwrap();
    //
    // // > For interactive uses, it is recommended to spawn a thread dedicated to user input and use blocking IO directly in that thread.
    // // > https://docs.rs/tokio/0.2.24/tokio/io/fn.stdin.html
    // let mut stdin = tokio::io::stdin();
    // let mut stdout = tokio::io::stdout();
    // // pipe current stdin to the stdin writer from ws
    // tokio::spawn(async move {
    //     tokio::io::copy(&mut stdin, &mut stdin_writer)
    //         .await
    //         .unwrap();
    // });
    // // pipe stdout from ws to current stdout
    // tokio::spawn(async move {
    //     tokio::io::copy(&mut stdout_reader, &mut stdout)
    //         .await
    //         .unwrap();
    // });
    // // When done, type `exit\n` to end it, so the pod is deleted.
    // let status = attached.await;
    // debug!("{:?}", status);

    info!("Session ended");
    info!("Deleting Pod {} ...", &pod_name);
    pods.delete(&pod_name, &DeleteParams::default())
        .await?
        .map_left(|pdel| {
            assert_eq!(pdel.name(), pod_name.as_str());
        });

    Ok(())
}
