# shippr
A simple binary to manage your helmcharts.
Often you dont write a helmchart simply to deploy it but often you re-use it for multiple deployments or you slightly adjust it differently on different machines. shippr wants to support this by adding the concept of a new file: `deployment`. For your applications, split helmcharts and deployments. In a given directory you will find at minimum 2 things:
1. A deployment file, that specifies the helmchart and version
2. A `values-default.yaml`.

If you use multiple profiles, you can also add multiple files like `values-dev.yaml` or `values-prod.yaml`.

Read the [Rust CLI Book](https://rust-cli.github.io/book/index.html) if you're interested in creating your own CLI

## Deployment file
The deployment file supports the following file formats: YAML, TOML, JSON, INI, RON, JSON. The primary file format for shippr is YAML. \
Why YAML by default?? - Helm works with YAML. Having different file formats is confusing than just using these very simple Yamls. Fully enabling something like TOML would require mapping between the formats which I wanted to avoid. \
However if you dont mind, feel free to switch to any other of the listed file formats for your deployment.yaml

In there you simply need the following structure. While local works for the location, it is mainly meant for testing purposes. While relative paths also work, it is recommended to use absolut paths, as it takes the exeuction directory as its base path
```yaml
name: ingress-nginx                                     # Required / Name under which the Chart is being released
version: 1.12.0                                         # Optional / Version of the Chart
namespace: ingress-nginx                                # Required / Namespace in which the Chart is being released
location:                                               # Required - Exactly one / Location where to find the Chart
  repo: https://kubernetes.github.io/ingress-nginx
  local: /home/user/charts/ingress-nginx
```

## Usage
**shippr**
```
A simple binary to manage your helmcharts

Usage: shippr [OPTIONS] <COMMAND>

Commands:
  cluster  Configures the cluster
  check    Verifies that the chart can be deployed
  deploy   Deploys helm chart by its deployment file
  cleanup  Cleans up any releases that are deployed but not defined
  help     Print this message or the help of the given subcommand(s)

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
