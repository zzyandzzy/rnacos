# 简介

rnacos是一个用rust实现的nacos服务。

rnacos是一个轻量、 快速、稳定、高性能的服务；包含注册中心、配置中心、web管理控制台功能，支持单机、集群部署。

rnacos设计上完全兼容最新版本nacos面向client sdk 的协议（包含1.x的http OpenApi，和2.x的grpc协议）, 支持使用nacos服务的应用平迁到 rnacos。

rnacos相较于java nacos来说，是一个提供相同功能，启动更快、占用系统资源更小、性能更高、运行更稳定的服务。

## 适用场景

1. 开发测试环境使用nacos，nacos服务可以换成rnacos。启动更快，秒启动。
2. 个人资源云服务部署的 nacos，可以考虑换成rnacos。资源占用率低: 包10M 出头，不依赖 JDK；运行时 cpu 小于0.5% ，小于5M（具体和实例有关）。
3. 使用非订制nacos服务 ，希望能提升服务性能与稳定性，可以考虑迁移到 rnacos。


## 功能说明

这里把 nacos 服务的功能分为三块
1、面向 SDK 的功能
2、面向控制台的功能
3、面向部署、集群的功能

每一块做一个对nacos服务的对比说明。

### 一、面向 SDK 的功能

访问认证：

1. 有提供获取认证token的接口
2. 实际请求暂不支持认证，都算认证通过。

配置中心：

1. 支持配置中心的基础功能、支持维护配置历史记录
2. 兼容配置中心的SDK协议
3. 暂不支持灰度发布、暂不支持tag隔离

注册中心：

1. 支持注册中心的基础功能
2. 兼容配置中心的SDK协议
3. 暂不支持1.x的 udp 实例变更实时通知，只支持 2.x 版本grpc实例变更实时通知 。最开始的版本也有支持过udp实例变更 通知，后面因支持 grpc 的两者不统一，就暂时去掉，后继可以考虑加回去。

### 二、面向控制台的功能

访问认证：
暂时不开启认证

命名空间：

1. 支持管理命名空间列表
2. 支持切换命名空间查询配置、服务数据。

配置中心：

1. 支持配置中心信息管理
1. 支持配置导入、导出,其文件格式与nacos兼容
2. 支持配置历史记录查看与恢复
3. 暂不支持tag的高级查询
4. 暂不支持查询配置监听记录

服务中心：

1. 支持注册中心的服务、服务实例管理
2. 暂不支持查询监听记录

### 三、面向部署、集群的功能

1. 支持单机部署
2. 支持集群部署。集群部署配置中心数据使用raft+节点本地存储组成的分布式存储，不需要依赖mysql。具体参考 [集群部署说明](./cluster_deploy.md)
