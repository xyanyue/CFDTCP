//! 这是一个简单判断多个短文本相关性的库。【需提供一个中心点，也就是需要选择一个短文本作为参照】
//!
//! 主要适用于比如：相关文章推荐之后的，各个推荐标题是否过于紧密的判断
//!
//! ## 流程：
//! 1. 使用AC自动机，去除停用词。【当然更好的方式是 分词，但是为了提高运行效率，未使用分词】
//! 2. 使用 one-hot 方式生成各个句子和中心句子的句向量，存储到Bitmap。【更好的是：使用语言模型生成向量】
//! 3. 计算各个句子向量到中心句之间的距离。
//! 4. 对计算的距离 统计离散系数，可以看出其 聚合程度
//! 5. 或者使用Jenks Natural Breaks 聚类，并查找最优的【标准差最小】的簇，簇越多表示越离散，越少越聚合 【更好的是：使用密度聚类算法】
//! ## Example
//! ```no run
//!     let centor =  "感冒第二天了，嗓子完全沙哑了，怎么办";
//!     let list = [
//!                "感冒咳嗽引起嗓子沙哑",
//!                "我是感冒引起的嗓子沙哑",
//!                "感冒咳嗽流鼻涕嗓子沙哑",
//!                "因感冒引起的嗓子沙哑",
//!                "感冒引起了嗓子沙哑。完全说不出话来",
//!                "前几天感冒嗓子有点沙哑",
//!                "年前感冒引起的嗓子沙哑",
//!                "我是感冒引起的嗓子沙哑",
//!                "感冒四天了，嗓子沙哑",
//!     ];
//!     let mut cfdtcp = CFDTCP::CFDTCP::new();
//!     cfdtcp.centor(centor.to_owned()).list(list.to_vec());
//!     // 获取众位数 距离相同最多的
//!     // @return (mode,count)
//!     let mode = cfdtcp.mode().unwrap();
//!     // 离散系数
//!     // @return f64
//!     let distribution = cfdtcp.distribution();
//!     // 聚类 参数-表示最多计算的簇，比如有9个句子，最多只能分成9簇，越少计算越快，准确度越低
//!     // @return (usize,Vec)
//!     let class = cfdtcp.jenks_classify(9);
//! ```
use std::{collections::HashMap, fmt::Display, path::Path};

use jenks::get_jenks_classification;
use text_processing::Processing;
use utilities::Classification;
use vectorization::Vectorization;

mod jenks;
pub mod text_processing;
mod utilities;
pub mod vectorization;

// 众位数
pub fn mode(v: &Vec<u64>) -> Option<(u64, i32)> {
    let frequencies = v.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });
    frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, count)| (*value, count))
}
// 平均值
pub fn mean(v: &Vec<u64>) -> Option<f64> {
    let sum = v.iter().sum::<u64>() as f64;
    let count = v.len();

    match count {
        positive if positive > 0 => {
            Some(sum / count as f64)
            // println!("mean:{:?}", m);
            // m
        }
        _ => None,
    }
}
// 标准差
pub fn std_deviation(v: &Vec<u64>) -> Option<f64> {
    match (mean(v), v.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = v
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value as f64);

                    diff * diff
                })
                .sum::<f64>()
                / count as f64;
            Some(variance.sqrt())
        }
        _ => None,
    }
}
// 离散系数
fn discrete_coefficient(v: &Vec<u64>) -> Option<f64> {
    match (std_deviation(v), mean(v)) {
        (Some(dev), Some(m)) => Some(dev / m),
        _ => None,
    }
}
pub struct CFDTCP<'a> {
    v: Vectorization<'a>,
    p: Processing,
}

impl<'a> CFDTCP<'a> {
    pub fn new() -> CFDTCP<'a> {
        let mut P = text_processing::Processing::new_file(Path::new(".stop_word.txt"));
        P.set_ac();
        let mut V = Vectorization::new();
        Self { v: V, p: P }
    }
    pub fn centor(&mut self, c: String) -> &mut Self {
        self.v.centor(self.p.parse(c));
        self
    }

    pub fn list(&mut self, list: Vec<&'a str>) -> &mut Self {
        self.v.list(list);
        self
    }

    pub fn distribution(&mut self) -> Option<f64> {
        discrete_coefficient(&self.v.get_dt())
    }

    pub fn mode(&mut self) -> Option<(u64, i32)> {
        mode(&self.v.get_dt())
    }

    pub fn jenks_classify(&mut self, num_bins: usize) -> (usize, Classification) {
        let mut c = Classification::new();
        let mut n: usize = 0;
        let mut min = f64::MAX;
        let data = self.v.get_dt();
        for num_bin in 1..num_bins {
            let class = get_jenks_classification(num_bin, data);
            let max_dev = CFDTCP::one_jenks_max_std_deviation(data, &class);
            // println!("max:{} min:{} n:{}", max_dev, min, num_bin);
            if max_dev < min {
                min = max_dev;
                n = num_bin;
                c = class;
            }
        }
        (n + 1, c)
    }
    fn one_jenks_max_std_deviation(dt: &Vec<u64>, res: &Classification) -> f64 {
        let mut max = 0.0;
        let start = 0;
        for bin in res {
            match std_deviation(&dt[start as usize..start + bin.count as usize].to_vec()) {
                Some(v) => {
                    if v > max {
                        max = v
                    }
                }
                None => {}
            }
        }
        max
    }

    pub fn get_dt(&mut self) -> &Vec<u64> {
        self.v.get_dt()
    }

    pub fn clear(&mut self) {}
}

pub struct MyClassification(pub Classification);

impl Display for MyClassification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vec = Vec::new();
        for bin in &self.0 {
            vec.push(format!(
                "start:{} end:{} count:{}",
                bin.bin_start, bin.bin_end, bin.count
            ));
        }
        write!(f, "{:?}", vec)
    }
}
