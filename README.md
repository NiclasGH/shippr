# shippr
A simple binary to manage your helmcharts.
Often you dont write a helmchart simply to deploy it but often you re-use it for multiple deployments or you slightly adjust it different on different machines. shippr wants to support this by adding the concept of a new file: `deployment`. For your applications, split helmcharts and deployments. In a given directory you will find at minimum 2 things:
1. A deployment file, that specifies the helmchart and version
2. A `values-default.yaml`.

If you use multiple profiles, you can also add multiple files like `values-dev.yaml` or `values-prod.yaml`.

Read the [Rust CLI Book](https://rust-cli.github.io/book/index.html) if you're interested in creating your own CLI

## Deployment file
The deployment file supports the following file formats: TOML, JSON, YAML, INI, RON, JSON.

In there you simply need the following structure (example in yaml):
```yaml
chart:
    name: ingress-nginx                                     # Required
    version: 1.12.0                                         # Optional
    namespace: ingress-nginx                                # Optional
    location:                                               # Required - At least one
        repo: https://kubernetes.github.io/ingress-nginx
        local: /home/user/charts/ingress-nginx
```
For the location, repo will be prioritized

## Usage
**shippr**
```
shippr[EXE] [OPTIONS]
    install [kube|helm]     Deploys kubectl or helm
    
    cluster <CLUSTER>       Sets the current cluster
    check <DIR>             Performs a dry run to check that everything works
    deploy <DIR>            Deploys the application in the specified directory [default: "."]
    cleanup <DIR>           Compares the deployed releases with the applications in the current directory. 
                            Undeploys if the directory doesn't contain the release.
        -n, --namespace     Only cleans up a specified namespace
    
    -y, --no-verify         Deploys/Undeploys without confirming the action
    -p, --profile <NAME>    Deploys in a specific profile (e.g. dev/prod etc.)
    -h, --help              Print help
    -V, --version           Print version
```

## Docker
**TODO** here you can find a leightweight docker image for use in pipelines. It has kubectl, helm and shippr pre-installed


