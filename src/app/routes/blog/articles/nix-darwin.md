---
title: Nix on macOS
draft: true
date: 2024-10-20
---

Install Nix (from: https://nixos.org/download/):
```bash
$ sh <(curl -L https://nixos.org/nix/install)
```

Create a directory for your nix config:
```bash
mkdir ~/.config/nix
cd ~/.config/nix
```

Scaffold a nix-darwin flake:
```bash
nix flake init -t nix-darwin --extra-experimental-features "nix-command flakes"
```


__Sources__:
- ()[https://www.youtube.com/watch?v=Z8BL8mdzWHI&t=282s]
