---
layout: ../../layouts/BlogLayout.astro
title: GitHub Actions Self Hosted Runners on Kubernetes
---

Source:
https://docs.github.com/en/actions/hosting-your-own-runners/managing-self-hosted-runners-with-actions-runner-controller/quickstart-for-actions-runner-controller

## 1. Install a  Actions Runner Controller (ARC)

```bash
helm install arc --namespace "arc-systems" --create-namespace oci://ghcr.io/actions/actions-runner-controller-charts/gha-runner-scale-set-controller
```

## 2. Create a PAT (Classic)

https://docs.github.com/en/actions/hosting-your-own-runners/managing-self-hosted-runners-with-actions-runner-controller/authenticating-to-the-github-api#deploying-using-personal-access-token-classic-authentication

Scope: 
- Repository runners: repo
- Organization runners: admin:org

## 3. Install a Runner Set

```bash
helm install "self-hosted" --namespace "arc-runners" --create-namespace --set githubConfigUrl="https://github.com/uxsoft-rs" --set githubConfigSecret.github_token="${GITHUB_PAT}" oci://ghcr.io/actions/actions-runner-controller-charts/gha-runner-scale-set
```

## 4. 
https://some-natalie.dev/blog/kaniko-in-arc/