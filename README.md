

A kubectl plugin to mount Persistent Volume Claim (PVC) on a new and interactive Pod.



## Installation

### Cargo

```
$ cargo install --path ./
```



## Usage

```bash
$ kubectl mount -h
kubectl-mount 0.1.0


USAGE:
    kubectl-mount [FLAGS] [OPTIONS] --pvc <pvc>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -w, --write      by default, mount PVC in readonly mode

OPTIONS:
        --image <image>              the Docker Image to use for the agent Pod [default: nicolaka/netshoot]
    -m, --mount-path <mount-path>    mount path in Pod [default: /opt]
    -n, --namespace <namespace>      namespace for the Agent Pod to run [default: default]
        --pod-name <pod-name>        override the default Agent Pod name [default: mount-agent]
        --pvc <pvc>                  Persistent Volume Claim (PVC) name
        --timeout <timeout>          the network timeout (seconds) [default: 100]
```



### example

Assume there's a pvc data-mysql-0 in default namespace.

```bash
$ kubectl mount --pvc data-mysql-0
[2022-01-11T09:32:45Z INFO  kubectl_mount] Creating Pod mount-agent ...
[2022-01-11T09:32:45Z INFO  kubectl_mount] Waiting Pod mount-agent ready ...
[2022-01-11T09:33:06Z INFO  kubectl_mount] Attaching to Pod mount-agent ...
~ #
~ # ls /opt/
data        lost+found
[2022-01-11T09:35:03Z INFO  kubectl_mount] Session ended
[2022-01-11T09:35:03Z INFO  kubectl_mount] Deleting Pod mount-agent ...
```



### readwrite

use `-w`

```bash
$ kubectl mount --pvc data-mysql-0 -w
[2022-01-11T09:39:16Z INFO  kubectl_mount] Creating Pod mount-agent ...
[2022-01-11T09:39:17Z INFO  kubectl_mount] Waiting Pod mount-agent ready ...
[2022-01-11T09:39:42Z INFO  kubectl_mount] Attaching to Pod mount-agent ...
~ # cd /opt
/opt # touch a.txt
/opt # ls
a.txt       data        lost+found
/opt #
[2022-01-11T09:40:04Z INFO  kubectl_mount] Session ended
[2022-01-11T09:40:04Z INFO  kubectl_mount] Deleting Pod mount-agent ...
```

