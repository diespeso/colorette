extern crate rand;
use rand::prelude::*;
use rocket::http::ext::IntoCollection;

use std::{collections::HashMap, hash::Hash};

use std::io::Cursor;
use std::fs::File;
use std::io::Write;
use image::io::Reader;
use image::Pixel;
use image::GenericImageView;

fn process_image() {
    println!("{:?}", std::env::current_dir());
    let img = Reader::open("test.jpg")
        .expect("failed to load img")
        .decode()
        .expect("failed to decode img");
    let pixels: Vec<[i32; 3]> = img.pixels().map(|(x, y, pixel)| {
        [pixel.channels()[0] as i32
        , pixel.channels()[1] as i32
        , pixel.channels()[2] as i32]
        /*pixel.channels().iter()
            .map(|f| {
                *f
            }).collect()*/
    }).collect();
    let mut clusterer = ClusterMaker::new(6, &pixels);
    clusterer.cluster_with_iter(1);
    //println!("{:?}", clusterer.get_clusters());
    let mut file = std::fs::File::create("dump.txt").unwrap();
    file.write_all(
        format!("{:?}", clusterer.get_clusters()).as_bytes()
        /*clusterer.get_clusters()).as_bytes()*/
    );
    /*let pixel = img.to_rgb32f();
    let pixel: Vec<u32> = pixel.get_pixel(0, 0).channels()
        .iter().map(|f| {
            (f * 255.0) as u32
        }).collect();
    println!("{:?}", pixel);*/
}

//image processing lol



/// With defined type for now
/// 
#[derive(Debug)]
pub struct ClusterMaker<'a> {
    pub k: i32,
    //centroids: Vec<&'a[i32; 3]>, //should be distanceable
    pub centroids: Vec<[i32;3]>,
    pub clusters: HashMap<[i32;3], Vec<[i32;3]>>,
    pub data: &'a Vec<[i32; 3]>
}

impl<'a> ClusterMaker<'a> {
    pub fn new(k: i32, data: &'a Vec<[i32; 3]>) -> Self {
        ClusterMaker {
            k,
            centroids: Vec::new(),
            clusters: HashMap::new(),
            data: data }
    }

    pub fn cluster_with_iter(&mut self, iters: i32) {
        if iters <= 0 {
            panic!("wont cluster with less than 1 iteration");
        }
        //set centroids, iter 1
        self.centroids = self.calculate_centroids_first();
        self.calculate_clusters();

        //for
        if (iters == 1) { //first iteration already done, sorta
            return;
        }
        for i in 0..(iters - 1) {
            self.centroids = self.calculate_centroids();
            self.calculate_clusters();
            //println!("inner test: {:?}, centroids: {:?}", self.get_clusters(), self.centroids);
        }

    }

    pub fn get_clusters(&self) -> HashMap<[i32;3], Vec<&[i32;3]>> {
        let mut clusters_back = HashMap::new();
        self.centroids.iter().for_each(|centroid| {
            clusters_back.insert(*centroid, Vec::new());
        });
        self.clusters.iter().for_each(|(data, cluster)| {
            clusters_back
                .get_mut(&cluster[0]).expect("failed to unwrap get_mut at get_clusters") //really gotta get rid of t hat useless vec wrapper
                .push(data) //lol
        });
        clusters_back

    }

    fn calculate_centroids(&self) -> Vec<[i32; 3]>  {
        /// analogous to self.clusters. self.clusters has data -> cluster tag
        /// while clusters_back will have cluster tag -> vec<data>
        let mut clusters_back = HashMap::new();
        let mut centroid_avg = HashMap::new();
        if(self.clusters.len() < 1) {
            panic!("cant calculate centroids without cluster data first");
        }
        self.centroids.iter().for_each(|centroid| {
            clusters_back.insert(centroid, Vec::new());
            centroid_avg.insert(centroid, [0, 0, 0]);
        });
        //println!("clusts: {:?}", clusters_back);
        
        self.clusters.iter().for_each(|(data, cluster)| { //
            clusters_back
                .get_mut(&cluster[0]) //when i change clusters to have a vec to just the array, change this
                .expect("failed to get mutable ref to cluster at calculate_centroids")
                .push(data)
        });
        let mut sum = [0, 0, 0];
        let mut count = 1;
        clusters_back.iter().for_each(|(cluster, data)| { //for each cluster -> data set
            data.iter().for_each(|data_point| { //for every data point y data set
                for i in 0..sum.len() { //for every number in  the data array
                    sum[i] += data_point[i];
                }
                count += 1;
            });
            *centroid_avg
                .get_mut(cluster)
                .expect("failed to get mut ref in centroid_avg")
                = [sum[0]/count, sum[1]/count, sum[2]/count]; //avg
            //restart
            sum = [0, 0, 0];
            count = 1;
        });

        centroid_avg.iter().map(|(original, average)| {
            *average
        }).collect()
    }

    pub fn calculate_centroids_first(&self) -> Vec<[i32; 3]> {
        let mut centroids = Vec::new();
        let n = self.data.len() as i32;
        if self.k > n {
            panic!("not enough data to cluster, k is too big")
        }
        let mut rng = rand::thread_rng();
        let mut i = 0;
        while i < self.k { //first iteration: get random centroids
            let gen = self.data[rng.gen_range(0..n) as usize];
            if centroids.len() > 0 {
                if centroids.contains(&gen) {
                    continue; //do it until its a different one
                }
            }
            centroids.push(
                gen
            );
            i += 1;
        }
        centroids
    }

    fn calculate_clusters(&mut self) {
        let n = self.data.len() as i32;

        //algo2 starts
        let mut closest_distance: Option<i32> = None;
        let mut closest_centroid = None;
        for i in 0..n { //for every data point
            let cur = self.data[i as usize];
            self.centroids.iter().for_each(|centroid|{ //for every centroid
                if let Some(mut c_dist) = closest_distance {
                    let cur_dist = centroid.get_distance_i32(&cur); //get distance relative to this data point
                    if cur_dist < c_dist {
                        c_dist = cur_dist;
                        closest_centroid = Some(*centroid);
                    }
                } else {
                    let cur_centroid = self.centroids[0];
                    closest_distance = Some(cur_centroid.get_distance_i32(&cur));
                    closest_centroid = Some(cur_centroid);
                }
            });
            //println!("closest:{:?}, dist: {:?}", closest_centroid, closest_distance);
            //at this point we should have the closest centroid, ready to store
            //disadvantage: repeated values are ignored in clustering
            self.clusters.insert(
                cur,
                Vec::from([
                    closest_centroid.expect("Failed to calculate closest centroid at ClusterMaker")
                ])
            );

            closest_distance = None;
            closest_centroid = None;
        }
        //algo 2 ends

       
    }
}


/// Distanceable is trait of a struct that can have its distance measure with other
/// instance of the same struct
pub trait Distanceable {
    /// Other should usually be Self
    type Other: Distanceable;
    fn get_distance_f64(&self, other: &Self::Other) -> f64{unimplemented!()}
    fn get_distance_i32(&self, other: &Self::Other) -> i32{unimplemented!()}
}

impl Distanceable for [i32; 3] {
    type Other = Self;
    fn get_distance_i32(&self, other: &Self::Other) -> i32 {
        return f32::sqrt(
            (other[0] - self[0]).pow(2) as f32
            + (other[1] - self[1]).pow(2) as f32
            + (other[2] - self[2]).pow(2) as f32
        ) as i32
    }
}

#[cfg(test)]
mod tests_engine {
    use crate::engine::ClusterMaker;

    use super::{Distanceable, process_image};

    #[test]
    fn test_processing() {
        process_image();
    }

    #[test]
    fn test_engine() {
        let dataset = Vec::from([[0, 0, 0], [1, 2, 3], [255, 255, 255], [160, 0, 0],
        [1, 1, 1], [250, 250, 250], [0, 255, 0]]);
        
        let mut cluster = ClusterMaker::new(2, &dataset);
        //println!("{:?}", cluster.get_clusters());
        cluster.cluster_with_iter(10);
        //println!("{:?}", cluster.get_clusters());
    }
}
