---
title: Permanent Admin on MacOS
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

Add your user to the admin group using the following command: ([source](https://superuser.com/questions/214004/how-to-add-user-to-a-group-from-mac-os-x-command-line)).


```bash
sudo dseditgroup -o edit -a john -t user admin
```

