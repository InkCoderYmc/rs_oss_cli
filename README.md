# rs_oss_cli
基于aws的s3封装了一个通用的oss cli工具

参数解释：

--action(-a) 包括upload,download,delete,list

--enable-dir(-e) 是否启动dir模式,dir模式默认输入输出路径均为dir(目前版本未实现强校验)

--local-path(-l) 本地路径

--oss-path(-o) oss端路径

--config-path 配置文件路径(目前仅支持yaml), 支持从环境变量读取，命令行传递优先级高于环境变量

--config-name 支持配置文件中配置项的选择(默认为default)，同上
