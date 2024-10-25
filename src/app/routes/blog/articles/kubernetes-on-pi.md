---
title: Kubernetes on Raspberry Pi
---

## Setting up the Raspberry Pi 4/5

Download the [Raspberry Pi Imager](https://www.raspberrypi.com/software/) to flash the microSD card or USB disk with the Raspberry Pi OS.
- Change the advanced settings, specifically don't forget to change the following:
    - Wifi
    - Username and password

## Install Kubernetes (K3s)

### Step 1 (cgroup)

Standard Raspbian Buster installations do not start with cgroups enabled. K3S needs cgroups to start the systemd service. cgroups can be enabled by appending `cgroup_memory=1 cgroup_enable=memory` to `/boot/cmdline.txt`

### Step 2 (k3s master)

```bash
curl -sfL https://get.k3s.io | sh -
```

### Step 3 (k3s node)

Source: https://sahansera.dev/building-your-own-private-kubernetes-cluster-on-a-raspberry-pi-4-with-k3s/

On the master to get security token:
```bash
sudo cat /var/lib/rancher/k3s/server/token
```

On the agent:
```bash
curl -sfL https://get.k3s.io | K3S_NODE_NAME="node01" K3S_URL="https://10.0.0.100:6443" K3S_TOKEN="token from above step" sh -
```

On the master to verify:
```bash
kubectl get nodes
```

## Kubernetes Remote Console (kubectl)

On the master node:

```bash
sudo cat /etc/rancher/k3s/k3s.yaml
```

On the PC edit .kube/config

```bash
kubectl config use-context pi
```

## Kubernetes Monitoring (Prometheus)

Install Lens

Right click on cluster > Settings > Lens Monitoring

Check prometheus
Check kube-state-metrics
Check node-exporter

## Kubernetes Storage (Longhorn)

Source: https://docs.k3s.io/storage

Install pre-requisites on every node:
```bash
sudo apt-get install open-iscsi
```

Apply manifest:
```bash
kubectl apply -f https://raw.githubusercontent.com/longhorn/longhorn/master/deploy/longhorn.yaml
```

Dashboard: port-forward 

## Kubernetes CI/CD (ArgoCD)

```bash
kubectl apply -f https://raw.githubusercontent.com/argoproj/argo-cd/master/manifests/install.yaml --namespace argocd
```

## Fan Control

### Enable PWM

__Source:__ https://github.com/dotnet/iot/blob/main/Documentation/raspi-pwm.md

Open `/boot/config.txt`
```bash
sudo nano /boot/config.txt
```
And add a line with `dtoverlay=pwm,pin=18,func=2`. 
Reboot.


### Wire the fan

__Source:__ https://blog.driftking.tw/en/2019/11/Using-Raspberry-Pi-to-Control-a-PWM-Fan-and-Monitor-its-Speed/

<img src="/images/blog/kubernetes-on-pi-fan-wiring.webp" width="800px">

### Fan Control

__Source:__ https://github.com/treydempsey/fan_control/tree/main

- Install rustup
- Clone the git repository
    ```bash
    git clone https://github.com/treydempsey/fan_control
    ```
- Build the code
    ```bash
    cd fan_control/
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    cargo build --release
    ```
- Install the binary
    ```bash
    sudo cp target/release/fan_control /usr/local/bin
    ```
- Install the systemd unit file
    ```bash
    sudo cp fan_control.service /lib/systemd/system
    ```
- Fix service:
    ```bash
    sudo nano /lib/systemd/system/fan_control.service
    ```
    And change `[Service]` to `ExecStart=/usr/local/bin/fan_control`

- Start the service:
    ```bash
    sudo systemctl enable fan_control
    sudo systemctl start fan_control
    systemctl status fan_control
    ```

## Firewall

```bash
sudo apt-get install ufw
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw limit 22

# allow k3s ports: https://docs.k3s.io/installation/requirements#networking
sudo ufw allow 6443
sudo ufw allow 2376/tcp
sudo ufw allow 2379/tcp
sudo ufw allow 2380/tcp
```

## Automatic System Updates

Source: https://haydenjames.io/how-to-enable-unattended-upgrades-on-ubuntu-debian/

Install:

```bash
sudo apt update && sudo apt upgrade
sudo apt install unattended-upgrades
sudo apt install apt-listchanges
```

Change settings at:

```bash
sudo nano /etc/apt/apt.conf.d/50unattended-upgrades
```

## Automatic Kubernetes Updates

https://docs.k3s.io/upgrades/automated

## Dynamic DNS

Script: https://github.com/K0p1-Git/cloudflare-ddns-updater
```bash
crontab -e
```

```
*/5 * * * * /bin/bash /home/pi/cloudflare-ddns-updater/cloudflare.sh
```

## Router Port Forwarding

## GitHub Actions Automated Deployment
Source: https://nicwortel.nl/blog/2022/05/27/continuous-deployment-to-kubernetes-with-github-actions


## Configure Private Docker Registry

JetBrains Containers: 

```bash
kubectl create namespace <NAMESPACE>
kubectl create secret docker-registry regcred --docker-server=<SPACE>.registry.jetbrains.space/p/<PROJECT>/containers --docker-username=<USERNAME> --docker-password=<PASSWORD> --namespace <NAMESPACE>
```

Github Container Registry:

```bash 
kubectl create namespace <NAMESPACE>
kubectl create secret docker-registry regcred --docker-server=ghcr.io --docker-username=<USERNAME> --docker-password=<PASSWORD> --namespace <NAMESPACE>
```
