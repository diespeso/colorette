extern crate rand;
use rand::prelude::*;

use std::{collections::HashMap, hash::Hash};


/// With defined type for now
/// 
#[derive(Debug)]
pub struct ClusterMaker<'a> {
    k: i32,
    centroids: Vec<&'a[i32; 3]>, //should be distanceable
    clusters: HashMap<&'a[i32;3], Vec<&'a[i32;3]>>,
    data: Vec<&'a[i32; 3]>
}

impl<'a> ClusterMaker<'a> {
    pub fn new(k: i32, data: Vec<&'a[i32; 3]>) -> Self {
        ClusterMaker {
            k,
            centroids: Vec::new(),
            clusters: HashMap::new(),
            data: data }
    }

    pub fn cluster_with_iter(&mut self, iters: i32) {
        //set centroids, iter 1
        self.centroids = self.calculate_centroids_first();
        self.calculate_clusters(iters);

        //for
        println!("centroids(new): {:?}", self.calculate_centroids());

    }

    fn calculate_centroids(&self) -> Vec<&'a[i32; 3]>  {
        let mut centroids = Vec::new();

        /// analogous to self.clusters. self.clusters has data -> cluster tag
        /// while clusters_back will have cluster tag -> vec<data>
        let mut clusters_back = HashMap::new();
        if(self.clusters.len() < 1) {
            panic!("cant calculate centroids without cluster data first");
        }
        self.centroids.iter().for_each(|centroid| {
            clusters_back.insert(centroid, Vec::new());
        });
        self.clusters.iter().for_each(|(data, cluster)| {
            clusters_back
                .get_mut(&cluster[0]) //when i change clusters to have a vec to just the array, change this
                .expect("failed to get mutable ref to cluster at calculate_centroids")
                .push(data)
        });

        println!("reverse: {:?}", clusters_back);
        centroids
    }

    fn calculate_centroids_first(&self) -> Vec<&'a[i32; 3]> {
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

    fn calculate_clusters(&mut self, iters: i32) {
        let n = self.data.len() as i32;
        /*if self.k > n {
            panic!("not enough data to cluster, k is too big")
        }
        let mut rng = rand::thread_rng();*/
        //let rando = rng.gen_range(0..n);
        // in a real world setting it wont matter if the choosen are repeated
        // there will be too much test data
        /*let mut i = 0;
        while i < self.k { //first iteration: get random centroids
            let gen = self.data[rng.gen_range(0..n) as usize];
            if self.centroids.len() > 0 {
                if self.centroids.contains(&gen) {
                    continue; //do it until its a different one
                }
            }
            self.centroids.push(
                gen
            );
            i += 1;
        }*/

        //algo2 starts
        /*self.centroids.iter().for_each( |centroid| {
            self.clusters.insert(centroid, Vec::new());
        });*/
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
            /*self.clusters
                .insert(cur, 
                    Vec::new());
            println!("final status: {:?}, {:?}", cur, closest_centroid.unwrap());
            self.clusters
                .get_mut(cur)
                .expect("Failed to get mutable clusters at ClusterMaker")
                .push(closest_centroid.expect("Failed to calculate closest centroid at ClusterMaker"));
            */
                /*self.clusters
                .get_mut(cur)
                .expect("Failed to get mutable clusters at ClusterMaker")
                .push(closest_centroid.expect("Failed to calculate closest centroid at ClusterMaker"));
            */
            //restart

            //println!("clusters: {:?}", self.clusters);

            closest_distance = None;
            closest_centroid = None;
        }
        //algo 2 ends

        //set centroid keys in cluster map
        /*self.centroids.iter().for_each(
            |centroid| {
                self.clusters.insert(*centroid, Vec::new());
            }
        );*/
        /*
        //let mut distances = Vec::new();
        //calculate distance between every datapoint and every centroid
        let mut deltas: HashMap<&[i32;3], HashMap<&[i32;3], i32>> = HashMap::new();
        for i in 0..n {
            deltas.insert(
                self.data[i as usize],
                self.centroids
                .iter().map(
                    |centroid| {
                        (*centroid, self.data[i as usize].get_distance_i32(&centroid))
                    }
                ).collect()
            );
        }   

        //set clusters using deltas, for every delta find the smallest distance
        //and assign the deltas datapoint to the cluster
        //TODO: OPTIMIZATION: Find a way to put this inside the original for
        let mut closest_distance = i32::MAX;
        let mut closest_centroid = [0, 0, 0]; //might be risky, maybe use option
        deltas.iter().for_each( //iter every data point
            |(data_point, distances)| {
                distances.iter().for_each( //iter distances relative to every centroid
                    |(centroid, distance)| {
                        if *distance < closest_distance {
                            closest_distance = *distance;
                            closest_centroid = **centroid; //xd
                        }
                    }
                );
                self.clusters.get_mut(&closest_centroid)
                    .expect("Failed to get mut referece to clusters")
                    .push(*data_point);
                //println!("{:?}", distances)
                closest_distance = i32::MAX;
                closest_centroid = [0, 0, 0];
            }
        );
        */
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

    use super::Distanceable;

    #[test]
    fn test_engine() {
        let dataset = [[0, 0, 0], [1, 2, 3], [255, 255, 255], [160, 0, 0]];
        println!("distance: {}", dataset[0].get_distance_i32(&dataset[2]));
        let mut cluster = ClusterMaker::new(2, 
            vec![&dataset[0], &dataset[1],
            &dataset[2], &dataset[3]]);
        println!("{:?}", cluster);
        cluster.cluster_with_iter(10);
        println!("{:?}", cluster);
    }
}
