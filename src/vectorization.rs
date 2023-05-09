use std::{
    collections::HashMap,
    ops::{BitOr, BitXor},
};

use roaring::RoaringBitmap;

use crate::discrete_coefficient;
#[derive(Debug)]
pub struct Vectorization<'a> {
    list: Vec<&'a str>,
    centor: String,
    words_hash: HashMap<char, usize>,
    words_vec: Vec<char>,
    pub dt: Vec<u64>,
}

impl<'a> Vectorization<'a> {
    pub fn new() -> Self {
        Vectorization {
            list: Vec::new(),
            centor: "".to_owned(),
            words_hash: HashMap::new(),
            words_vec: Vec::new(),
            dt: Vec::new(),
            // words_hash_index: HashMap::new(),
        }
    }
    pub fn get_dt(&mut self) -> &Vec<u64> {
        self.set_dt();
        &self.dt
    }
    pub fn get_word_len(&self) -> usize {
        self.words_vec.len()
    }
    pub fn add(&mut self, s: &'a str) {
        self.list.push(s);
    }
    pub fn list(&mut self, list: Vec<&'a str>) {
        self.list = list;
    }
    pub fn centor(&mut self, s: String) {
        self.centor = s;
    }

    fn words(&mut self) {
        self.words_vec = Vec::new();
        self.words_hash = HashMap::new();
        for w in &self.list {
            w.chars().for_each(|ch| {
                self.words_hash
                    .entry(ch)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            });
        }
        self.centor.chars().for_each(|ch| {
            self.words_hash
                .entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        });
        let mut vec: Vec<(&char, &usize)> = self.words_hash.iter().collect();
        // 出现次数多的字，排到前面，在存储进bitmap中时其存储在低位
        vec.sort_by(|a, b| b.1.cmp(a.1));
        // let mut words_vec = Vec::new();
        for (k, _) in vec {
            self.words_vec.push(*k);
            // self.words_hash.entry(k)
        }
    }

    fn word_set_index(&mut self) {
        for (i, c) in self.words_vec.iter().enumerate() {
            self.words_hash.entry(*c).and_modify(|index| *index = i);
        }
    }

    fn get_word_index(&self, c: &char) -> usize {
        match self.words_hash.get(c) {
            Some(index) => *index,
            None => 0,
        }
    }

    fn get_bitmap(&self, s: String) -> RoaringBitmap {
        let mut rb = RoaringBitmap::new();
        s.chars().for_each(|c| {
            let index = self.get_word_index(&c);
            rb.insert(index.try_into().unwrap());
        });
        rb
    }

    fn set_dt(&mut self) {
        self.words();
        self.dt = Vec::new();
        self.word_set_index();
        // let mut vec_list = Vec::new();q
        let centor_bit = self.get_bitmap(self.centor.clone());

        for l in &self.list {
            // let
            // vec_list.push();
            self.dt
                .push(self.distance(&centor_bit, &self.get_bitmap(l.to_string())));
        }
        self.dt.sort()
    }

    fn distance(&self, rb1: &RoaringBitmap, rb2: &RoaringBitmap) -> u64 {
        rb1.bitxor(rb2).len()
    }

    // pub fn distribution(&mut self) -> Option<f64> {
    //     self.set_dt();
    //     discrete_coefficient(self.get_dt())
    // }
}
