# 版本说明

|版本|对nacos兼容说明|版本特点说明|适用场景|
|--|--|--|--
|v0.1.10|支持1.x服务接口;除了查询服务中心服务列表外，支持2.x大部分接口|单机部署版本,配置中心数据库使用sqlite，内存占用比较低，但配置写入tps不高，7百左右|不使用2.x的注册中心服务，对内存占用敏感的小应用可以考虑使用|
|v0.2.3|支持2.x新最版nacos面向sdk的协议|单机部署版本,配置中心数据库使用sled, 初始内存多占用25M左右,配置中心单机写入tps在1.5万以上|对配置中心有高频写入需求的应用可以考虑使用|
|v0.3.0以上|支持2.x新最版nacos面向sdk的协议|支持集群部署版本，使用1个节点的集群方式当做单机模式,配置中心raft 协议没有充分优化，配置中心集群写入tps在1.5千左右|对配置中心没有太高频写入需求的应用都推荐使用这个版本|