

A kubectl plugin to mount Persistent Volume Claim (PVC) on a new and interactive Pod.

```bash
$ ./kubectl-mount -h
kubectl-mount 0.1.0


USAGE:
    kubectl-mount [FLAGS] [OPTIONS] --pvc <pvc>

FLAGS:
    -h, --help         Prints help information
        --readwrite    By default, Readonly mode
    -V, --version      Prints version information

OPTIONS:
        --image <image>            the Image to use for the agent Pod [default: nicolaka/netshoot]
    -n, --namespace <namespace>    namespace for the Agent Pod to run [default: default]
        --pod-name <pod-name>      override the default Agent Pod name [default: mount-agent]
        --pvc <pvc>                Persistent Volume Claim (PVC) name
        --timeout <timeout>         [default: 100]
```



in readonly mode:

```bash
$ ./kubectl-mount --pvc data-mysql-0
```



in readwrite mode:

```bash
$ ./kubectl-mount --pvc data-mysql-0 --readwrite
```



### TODO

1. Error: failed to upgrade to a WebSocket connection: failed to switch protocol: 400 Bad Request (in Lens console)
2. ~ # ^[[21;5R ugly display
3. Ctrl-C break the program (kubectl run: Ctrl-C doesn't break, catch Ctrl-D to exit the program)

