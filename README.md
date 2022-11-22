# 通过测试用例学习rust

[Rust 语言圣经](https://course.rs/)

看书学习，第一个看的就是[自动化测试](https://course.rs/test/write-tests.html)部分,之后就按照上面的示例敲代码


### 自己搭建《Rust语言圣经》服务

由于github访问的速度太慢所以在自己购买的云服务器主机上搭建了个《Rust语言圣经》服务，用云主机是因为我希望在手机、平板上也能访问到。没有使用本地搭建是因为我没有不关机的电脑，我不想每次重启电脑都操作一遍，设置成服务或守护进程的方法我没有get到，简单起见直接与服务器吧。以下方法使用与mac也适用于 windows，只是要自行修改相关命令与步骤。

环境:
1. 系统: centos 7
2. 版控: git
3. 其他: rust, screen, mdbook, vim, curl, systemctl, firewalld


参考:
1. [rust安装](https://doc.rust-lang.org/book/ch01-01-installation.html)
2. [screen使用](https://www.gnu.org/software/screen/manual/screen.html)
3. [git使用](https://git-scm.com/book/zh/v2)
4. [vim使用(这里未使用，用echo代替了)](https://vimhelp.org/)
5. [mdbook使用](https://rust-lang.github.io/mdBook/)
6. [cargo使用清华源(国内环境)](https://course.rs/first-try/slowly-downloading.html)
7. [curl文档](https://curl.se/docs/)
8. [systemctl](https://www.freedesktop.org/software/systemd/man/systemctl.html#)
9. [systemd (太高深没弄懂)](https://systemd.io/)
10. [firewalld (太高深没弄懂)](https://firewalld.org/documentation/)

```bash
# 1. 更新系统
yum update -y
# 2. 安装 git screen vim curl
yum install -y vim screen git curl
# 3. 安装 rust
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
# 4. 设置cargo使用清华源(国内环境) 加快cargo下载包的速度
cat > $HOME/.cargo/config.toml <<EOF
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
EOF
# 5. 安装mdbook
cargo install mdbook

# 6. clone《Rust语言圣经》 项目
git clone https://github.com/sunface/rust-course.git ~/rust-course

# 7. 查看端口是否被占用

# 8. 启动项目
screen -S rust-course
cd ~/rust-course
mdbook serve -n 0.0.0.0 -p 2022
# 快捷键 ctrl a, d (按住ctrl键依次按a和d 不是同时按三个键)

# 9. 本地访问测试
curl localhost:2022

# 10. 关闭并停用防火墙 (不一定正确)
systemctl stop firewalld
systemctl disable firewalld

# 11. 远程访问测试 
curl http://43.139.144.58:2022
```









