## v0.3.2

2023-09-17

1. 修复raft节点变更需要等更新配置后才同步到 naming node manage的问题

## v0.3.1


## v0.3.0

2023-09-16

1. rnacos支持集群部署，具备用于生产环境的条件。 集群起始版本更新为0.3.x
2. 配置中心使用raft协议+本地存储支持集群部署，不依赖mysql。
3. 注册中心使用类distro协议支持集群部署。
4. 修复非默认命名问题的配置中心下载配置内容为空的问题。

## v0.2.2

2023-09-16

1. rnacos支持集群部署，具备用于生产环境的条件。
2. 配置中心使用raft协议+本地存储支持集群部署，不依赖mysql。
3. 注册中心使用类distro协议支持集群部署。

## v0.2.2-beta.1

2023-08-26

1. 配置中心使用raft协议支持集群部署,合并v0.2.1后更新版本。


## v0.2.1

2023-08-26

1. grpc协议注册中心查询服务列表问题修复

## v0.2.1-beta.1

2023-08-07

1. 配置中心使用raft协议支持集群部署
   + 基于async-raft库实现rnacos的raft协议服务。
   + 配置中心储存层接入raft,支持集群部署。
   + 通过转发请求的方式支持raft从节点写入功能.
   + 配置中心集群功能测试通过,输出集群部署说明与测试脚本。
   + 初步完成配置中心的压测: 单节点的查询与单机一致，可水平扩容提升集群整体的qps；集群写入rps为一千左右，降幅较大，后继再做性能优化。

## v0.2.0

2023-07-03

1. 配置中心数据存储由sqlite切换到sled (因为计划使用sled+raft 支持配置中心的集群,所以本地存储统一切换成sled)
   + 写入qps比sqlite版本增二十多倍，达1.5万左右；
   + 内存比sqlite版本多占用20M左右；
   + 如果从0.1.x 版本升级到0.2.x，需要把老配置先导出再导入；
2. 统一处理`cargo clippy --all` 告警，使用 `cargo fmt --all` 对代码做格式化，方便多人协作；


## v0.1.10

2023-06-18

1. 更新grpc协议ServiceInfo对象的checksum字段类型


## v0.1.9

2023-06-10

1. 控制台配置变更前支持配置内容差异比较
2. 配置中心删除配置时也需要持久化

## v0.1.8

2023-06-01

1.  更新实例时，如果新实例来自http,旧实例来自grpc,则保持grpc的实例信息



## v0.1.7

2023-05-31

1. 调整对grpc连接判活机制,确保链接超时后会清空链接对应的对象信息;

## v0.1.6

2023-05-28

1. 完成历史配置历史变更记录页面开发。主要包含查询配置历史记录，与历史记录内容恢复两个功能。
2. 修复grpc处理日志中时长一直为0的问题。
3. 修复注册中心部分场景下的删除不存在服务实例的问题。

## v0.1.5

2023-05-19

1. 配置中心支持按条件导出配置文件，导出的文件兼容nacos格式。
2. 调整rnacos-web-dist-wrap引入方式,不通过build做二次处理
3. 区分维护docker稳定版本镜像 qingpan/rnacos:stable

## v0.1.4

1. 修复2.0版本注册心跳的问题，注册中心支持grpc统一维持心跳。
2. 配置中心支持导入配置文件，配置文件兼容 nacos 格式。（导出计划后继版本支持）

