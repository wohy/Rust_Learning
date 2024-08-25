# 陈天 · Rust 编程第一课
  
## 专栏交流群

[戳此加入](https://jinshuju.net/f/mJxhwJ)， 陈天的 Rust 交流群  
另外，陈天的《必知必会：3 小时 Rust 从入门到进阶》视频课，[戳此免费观看](http://u.geekbang.org/subject/intro/1007543?channel=65d1a6195d136)

  

## 你将获得

*   系统高效的 Rust 学习路径
*   攻克 Rust 编程十大难点
*   即学即练，Rust 四大项目实战
*   200+原理图，详解 Rust 设计理念

  

## 讲师介绍

陈天，现任北美最大的免费流媒体服务 TubiTV 的高级研发副总裁。他始终致力于高性能系统的研发，活跃在网络协议、网络安全、服务端架构、区块链以及云服务等诸多热门领域，已经积累了 18 年工作经验。

因为始终坚持自己的工作理念——“以合适的工具解决合适的问题”，所以在职业生涯的不同阶段，面对不同的工作需求，他坚持使用形态和机制都不同的开发语言，比如说：

*   用 C 和汇编打造过各种网络协议，维护过在网络安全领域非常知名的嵌入式操作系统 ScreenOS；
*   用 Python/JavaScript 撰写过曾经的创业项目途客圈；
*   用 Node.js/Elixir 打造过 TubiTV 高并发的后端核心；
*   用 Elixir 打造过区块链框架 Forge；
*   也研究过 Haskell/F#、Clojure/Racket、Swift、Golang 和 C# 等其他语言。

  

## 课程介绍

Rust 集表达力、高性能、内存安全于一身，在 1.0 版本发布后不久，口碑就一路高飞猛进，从 16 年起，连续 6 年成为 Stack Overflow 用户评选出来的最受喜爱的语言（2021/3/15评选结果）。

近几年，国外很多厂商宣布用 Rust 重写自己的技术栈，国内也有用 Rust 重写后端中间件的趋势。在可预见的未来，Rust 或在大多数领域代替 C/C++，甚至 Java/Golang，成为开发者的主力语言。

但是，Rust 对初学者似乎并不友好，被公认难入门，学习曲线相当陡峭。

比如变量的所有权和生命周期，作为 Rust 的创新概念，是其他编程语言都未涉及的领域。除此之外，不同语言背景，学习也各有难点：

*   C 开发者，难点是类型系统和泛型编程；
*   C++ 开发者，难点主要在类型系统；
*   Python/Ruby/JavaScript 开发者，难点在并发处理、类型系统及泛型编程；
*   Golang 开发者，难点在类型系统和泛型编程；
*   Java 开发者，难点在异步处理和并发安全的理解上。

该如何跨越这些门槛，不断攀登新高峰呢？

为此，我们邀请到 Rust 专家陈天老师，结合其十八年深度使用多种语言的经验，以先行者的身份输出学习心法，**从新手命令行到项目落地，带你知识与实战两手抓，真正掌握 Rust 的概念本质和设计理念，把 Rust 引入你的技术栈**。  
![](https://static001.geekbang.org/resource/image/1c/8d/1cf03ee698cyy875e8fac45b8ed5f88d.jpg)

### 课程模块设计

整个专栏分为五个模块：

**前置知识篇**  
回顾软件开发的基础概念：堆、栈、函数、闭包、虚表、泛型、同步和异步等。这是学好任意一门编程语言都要吃透的概念，因为编程语言不过是这些概念的具体表述和载体。

**基础知识篇**  
首先通过get hands dirty小项目周，感性体验Rust语言的魅力，然后回归理性，结合第一性原理，深入浅出地探讨 Rust 变量的所有权和生命周期，以及对比几种主流的内存管理方式。之后围绕所有权和生命周期，来讨论 Rust 的几大语言特性：函数式编程特性、类型系统、泛型编程以及错误处理。

**进阶篇**  
Pascal 之父，图灵奖得主尼古拉斯·沃斯有一个著名的公式：算法+数据结构=程序，想随心所欲地使用 Rust 为你的系统构建数据结构，深度掌握类型系统必不可少。

所以这个模块将重点介绍 trait、trait object、泛型、unsafe rust，最后还会讲到 FFI，让你用 Rust 为自己的主力语言在关键路径上提供更高的性能，或者引入 Rust 生态中特定的库。

**并发篇**  
学会用最合适的工具解决最合适的问题很重要。这个模块将带你从底层的 atomics 一路向上，历经 Mutex、Semaphore、Channel，直至 actor model，探索不同的并发手段。你会深度了解到，其他语言中被奉为圭臬的并发手段，在 Rust 里，只不过是一种并发工具。

**实战篇**  
单纯掌握语言特性，能应用这些特性写出解决一些小问题的代码，算是初窥门径，就像在游泳池里练习冲浪，想要真正把语言融会贯通，还要靠大风大浪中的磨炼。这个模块会带你学习如何把 Rust 应用在生产环境中，以及如何使用 Rust 的编程思想解决实际问题。

  

## 课程目录

![](https://static001.geekbang.org/resource/image/7b/1f/7b6f448a4d4de7279a82949332a3c21f.jpg)

  

## 适合人群

*   掌握任何一门编程语言
*   了解 Rust 基础语法

特别说明：如果你是 Rust 0 基础，推荐结合官方的 Rust book 大致了解语言概貌、基础语法，以减轻入门压力、平滑学习曲线。

  

## 特别放送

#### 免费领取福利

[![](https://static001.geekbang.org/resource/image/67/d3/6735de107a26897100a125272d1948d3.png?wh=1035x360)](http://time.geekbang.org/serv/v4/misc/jump?uri=https%3A%2F%2Ftime.geekbang.org%2Fhybrid%2Fmp%2Fjump%3Furl%3Dhttps%253A%252F%252Fstatic001.geekbang.org%252Fresource%252Fimage%252F30%252F8e%252F307017550c76b088bdd74b9ec86f4d8e.png)

  

#### 限时活动推荐

[![](https://static001.geekbang.org/resource/image/e7/88/e787834ec0927ca4bf8d6eb703c5e188.jpg)](https://time.geekbang.org/activity/promo?page_name=page_37&utm_source=tebiefangsong)

  

## 订阅须知

1.  订阅成功后，推荐通过“极客时间”App端、Web端学习。
2.  本专栏为虚拟商品，交付形式为图文+音频，一经订阅，概不退款。
3.  订阅后分享海报，每邀一位好友订阅有现金返现。
4.  戳此[先充值再购课更划算](https://shop18793264.m.youzan.com/wscgoods/detail/366k8bbsnpjzcdp?dc_ps=3288312595060320259.200001)，还有最新课表、超值赠品福利等。
5.  企业采购推荐使用“[极客时间企业版](https://b.geekbang.org/?utm_source=geektime&utm_medium=columnintro&utm_campaign=newregister&gk_source=2021020901_gkcolumnintro_newregister)”便捷安排员工学习计划，掌握团队学习仪表盘。
6.  戳此[申请学生认证](https://time.geekbang.com/activity/promo?page_name=page_3471768093)，订阅课程享受原价5折优惠。
7.  价格说明：划线价、订阅价为商品或服务的参考价，并非原价，该价格仅供参考。未划线价格为商品或服务的实时标价，具体成交价格根据商品或服务参加优惠活动，或使用优惠券、礼券、赠币等不同情形发生变化，最终实际成交价格以订单结算页价格为准。
