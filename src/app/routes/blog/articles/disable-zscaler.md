---
title: Disable Zscaler on macOS
draft: true
date: 2024-20-10
---


```bash
sudo launchctl load /Library/LaunchDaemons/com.zscaler.service.plist && sudo launchctl load /Library/LaunchDaemons/com.zscaler.tunnel.plist
```

```bash
sudo launchctl unload /Library/LaunchDaemons/com.zscaler.service.plist && sudo launchctl unload /Library/LaunchDaemons/com.zscaler.tunnel.plist
```

__Sources__:
- https://www.atpeaz.com/disable-zscaler-temporarily-on-your-mac/