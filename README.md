```
vagrant up
vagrant ssh-config >> ~/.ssh/config
scp -r . vagrant@yourvm:~/minios/

vagrant ssh -- -X
cd ~/minios

make run
```