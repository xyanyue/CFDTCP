 Short text clustering for determining center points and correlation judgment [Jenks Natural Breaks]

 这是一个简单判断多个短文本相关性的库。【需提供一个中心点，也就是需要选择一个短文本作为参照】

 主要适用于比如：相关文章推荐之后的，各个推荐标题是否过于紧密的判断

 ## 流程：
 1. 使用AC自动机，去除停用词。【当然更好的方式是 分词，但是为了提高运行效率，未使用分词】
 2. 使用 one-hot 方式生成各个句子和中心句子的句向量，存储到Bitmap。【更好的是：使用语言模型生成向量】
 3. 计算各个句子向量到中心句之间的距离。
 4. 对计算的距离 统计离散系数，可以看出其 聚合程度
 5. 或者使用Jenks Natural Breaks 聚类，并查找最优的【标准差最小】的簇，簇越多表示越离散，越少越聚合 【更好的是：使用密度聚类算法】
 ## Example
 ```rust
    let centor =  "感冒第二天了，嗓子完全沙哑了，怎么办";
     let list = [
                "感冒咳嗽引起嗓子沙哑",
                "我是感冒引起的嗓子沙哑",
                "感冒咳嗽流鼻涕嗓子沙哑",
                "因感冒引起的嗓子沙哑",
                "感冒引起了嗓子沙哑。完全说不出话来",
                "前几天感冒嗓子有点沙哑",
                "年前感冒引起的嗓子沙哑",
                "我是感冒引起的嗓子沙哑",
                "感冒四天了，嗓子沙哑",
     ];
     let mut cfdtcp = CFDTCP::CFDTCP::new();
     cfdtcp.centor(centor.to_owned()).list(list.to_vec());
     // 获取众位数 距离相同最多的
     // @return (mode,count)
     let mode = cfdtcp.mode().unwrap();
     // 离散系数
     // @return f64
     let distribution = cfdtcp.distribution();
     // 聚类 参数-表示最多计算的簇，比如有9个句子，最多只能分成9簇，越少计算越快，准确度越低
     // @return (usize,Vec)
     let class = cfdtcp.jenks_classify(9);
 ```
### test
```JSON
(
            "感冒第二天了，嗓子完全沙哑了，怎么办",
            [
                "感冒咳嗽引起嗓子沙哑",
                "我是感冒引起的嗓子沙哑",
                "感冒咳嗽流鼻涕嗓子沙哑",
                "因感冒引起的嗓子沙哑",
                "感冒引起了嗓子沙哑。完全说不出话来",
                "前几天感冒嗓子有点沙哑",
                "年前感冒引起的嗓子沙哑",
                "我是感冒引起的嗓子沙哑",
                "感冒四天了，嗓子沙哑",
            ],
        ),
        (
            "鼻炎，咽炎严重，特别是鼻炎",
            [
                "鼻炎，咽炎，鼻炎引起的咽炎",
                "鼻炎严重了会引起咽炎吗",
                "这是鼻炎还是咽炎呢严重吗",
                "有鼻炎，鼻炎比较严重",
                "我闺女有鼻炎，冬天鼻炎特别严重",
                "咽炎鼻炎七八年了，去年咽炎严重声带受损",
                "鼻炎咽喉炎严重鼻炎咽喉炎",
                "鼻炎的症状有哪些？鼻炎严不严重？",
                "鼻炎的症状有哪些？鼻炎严不严重？",
            ],
        ),
        (
            "得脚气的原因脚气怎么办",
            [
                "小孩得脚气怎么办什么治疗脚气",
                "脚气怎么办有脚气怎么办",
                "脚气难受怎么办？因为有脚气",
                "脚气是怎么得的为什么会得脚气",
                "得了脚气的症状脚气怎么办",
                "脚气的早期症状脚气怎么办",
                "脚气怎么办？脚气怎么才能好啊",
                "脚气不好怎么办？脚上有脚气",
                "有脚气怎么办脚气用什么",
            ],
        ),
        (
            "得脚气的原因脚气怎么办？",
            [
                "小孩得脚气怎么办什么治疗脚气",
                "脚气怎么办有脚气怎么办",
                "脚气难受怎么办？因为有脚气",
                "特别痒脱皮，是脚癣吗？还是脚气",
                "脚癣的病因",
                "脚癣根治方法",
                "香港脚治疗方法",
                "香港脚怎么办",
                "香港脚怎么才能根治啊",
            ],
        ),
```

```bash
//运行结果
 running 1 test
 得脚气的原因脚气怎么办离散系数：Some(0.19338161676886978) 众位数：距离-7 => 数量:3
 距离：[5, 5, 7, 7, 7, 8, 8, 9, 9]
 分类4:
 ["start:5 end:7 count:2", "start:7 end:8 count:3", "start:8 end:9 count:3"]
 ----------------------------------------------
 感冒第二天了，嗓子完全沙哑了，怎么办离散系数：Some(0.15596031940172347) 众位数：距离-15 => 数量:5
 距离：[8, 13, 14, 14, 15, 15, 15, 15, 15]
 分类2:
 ["start:8 end:15 count:5"]
 ----------------------------------------------
 鼻炎，咽炎严重，特别是鼻炎离散系数：Some(0.29921571088744625) 众位数：距离-13 => 数量:2
 距离：[5, 7, 7, 8, 8, 9, 12, 13, 13]
 分类5:
 ["start:5 end:7 count:1", "start:7 end:8 count:2", "start:8 end:12 count:3", "start:12 end:13 count:2"]
 ----------------------------------------------
 得脚气的原因脚气怎么办？离散系数：Some(0.34283965148438666) 众位数：距离-9 => 数量:2
 距离：[6, 7, 8, 9, 9, 14, 14, 15, 17]
 分类7:
 ["start:6 end:7 count:1", "start:7 end:9 count:2", "start:9 end:14 count:2", "start:14 end:15 count:2", "start:15 end:17 count:1",    "start:17 end:17 count:1"]
 ----------------------------------------------
 test test ... ok

 test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.72s
```


