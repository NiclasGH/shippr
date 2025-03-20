# shippr
A simple binary to manage your helmcharts.
Often you dont write a helmchart simply to deploy it but often you re-use it for multiple deployments or you slightly adjust it differently on different machines. shippr wants to support this by adding the concept of a new file: `deployment`. For your applications, split helmcharts and deployments. In a given directory you will find at minimum 2 things:
1. A deployment file, that specifies the helmchart and version
2. A `values-default.yaml`.

If you use multiple profiles, you can also add multiple files like `values-dev.yaml` or `values-prod.yaml`.

Read the [Rust CLI Book](https://rust-cli.github.io/book/index.html) if you're interested in creating your own CLI

## Deployment file
The deployment file supports the following file formats: TOML, JSON, YAML, INI, RON, JSON.

In there you simply need the following structure (example in yaml):
```yaml
name: ingress-nginx                                     # Required
version: 1.12.0                                         # Optional
namespace: ingress-nginx                                # Optional
location:                                               # Required - Exactly one
  repo: https://kubernetes.github.io/ingress-nginx
  local: /home/user/charts/ingress-nginx
```

## Usage
**shippr**
```
Usage: shippr [OPTIONS] <COMMAND>

Commands:
  cluster <NAME>    Configures the cluster
  check <DIR>       Verifies that the chart can be deployed
  deploy <DIR>      Deploys helm chart by its deployment file
  cleanup <DIR>     Cleans up any releases that are deployed but not defined
    -n <NAMESPACE>  Namespace to cleanup

  help              Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Enables verbose logging. [Default: ERROR logs]
  -h, --help        Print help
  -V, --version     Print version
```

## Docker
The docker image is made to be used in CI/CD pipelines. It contains helm, kubectl and shippr

## Developing
It is recommended to enable the githooks to prevent the CI failing after you push.
```bash
git config --local core.hooksPath .githooks/
```
