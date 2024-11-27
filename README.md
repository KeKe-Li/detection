#### detection 

server system resource monitoring probe memory tool is a tool used to monitor system resources (such as CPU, memory, disk, etc.)


#### build & run

```bash
> cargo build 

> cargo run 
```


61.51.111.41:26  (panwei-1)
61.51.111.41:27  (panwei-2)
61.51.111.41:28  (panwei-3)
>
root 密码都是 panwei-2025
虚机配置都是 4c/8g 

ssh -l root 61.51.111.41 -p 26 

ssh -l root 61.51.111.41 -p 27

ssh -l root 61.51.111.41 -p 28



> lscpu


scp -P 2222 file.txt username@remote_host:~/destination/
