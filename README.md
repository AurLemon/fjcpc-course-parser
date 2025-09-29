# fjcpc-course-parser

一个用 Rust + Vue 写的小项目，用来解析学校课程表信息，带 API。

因为我想把课表数据放到第三方课程表 APP 用，但学校的教务系统是基于 WeLink 的 H5 应用，网页版教务系统又看不到课程表。

所以单开一个仓库试图把船政的课程表解析成通用格式作为 API 以供其它平台使用（比如第三方课表）。

## FAQ

- Q1: 为什么项目英文名不是 FJCCC 而是 FJCPC？
- A1: FJCPC 是船政老域名（Fujian Chuanzheng Political College），就跟着这个懒得改了，虽然我知道现在是 FJCCC（Fujian Chuanzheng Communications College）。

- Q2: 为什么项目内的 API 请求都强制 IPv4 访问？
- A2: 因为学校的 DNS 解析服务器有问题，IPv6 虽然有解析但是无法连接。这意味着，如果你的设备支持 IPv6，那么每次连接学校服务器的时候就会优先选择 IPv6 连接，可 IPv6 的服务器是无法连接的，所以只能等 IPv6 超时后（10 秒）自动切换到 IPv4，学校 WeLink 卡顿也是这个原因。为了提高项目内学校的 API 访问速度，所以强制 IPv4。
