---
title: Permanent Admin on MacOS
date: 2024-10-13
---

## Sudoers

Add the user to the sudoers file:
```bash
sudo hx /etc/sudoers
```

Edit the following section and add your user below `root` and `%admin`. 
```
# root and users in group wheel can run anything on any machine as any user
root		ALL = (ALL) ALL
%admin		ALL = (ALL) ALL
john        ALL = (ALL) ALL
```


## Admin Group

Check if user is an admin:
```bash
dseditgroup -o checkmember -m john admin
```

Add your user to the admin group ([source](https://superuser.com/questions/214004/how-to-add-user-to-a-group-from-mac-os-x-command-line)):


```bash
sudo dseditgroup -o edit -a john -t user admin
```

