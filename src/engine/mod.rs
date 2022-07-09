use rand::prelude::*;
use rand::thread_rng;
use rand::prelude::SliceRandom;

use std::collections::HashMap;
use std::hash::Hash;

/// Distanceable is trait of a struct that can have its distance measure with other
/// instance of the same struct
pub trait Distanceable {
    /// Other should usually be Self
    type Other: Distanceable;
    fn get_distance_f64(&self, other: &Self::Other) -> f64{unimplemented!()}
    fn get_distance_u32(&self, other: &Self::Other) -> u32{unimplemented!()}
}

impl Distanceable for [u32; 3] {
    type Other = Self;
    fn get_distance_u32(&self, other: &Self::Other) -> u32 {
        return f32::sqrt(
            (other[0] as f32 - self[0] as f32).powf(2.0)
            + (other[1] as f32 - self[1] as f32).powf(2.0)
            + (other[2] as f32 - self[2] as f32).powf(2.0)
        ) as u32
    }
}



/* 
    suffle
    it 1:
        encontrar centroides: fn
        acomodar centroide: closest: fn
*/

/// first, with data cloning, bad performance maybe
#[derive(Debug)]
pub struct ClusterMaker {
    pub k: u32,
    pub data: Vec<[u32; 3]>,
    pub centroids: Vec<[u32; 3]>,
    pub clusters: HashMap<[u32; 3], Vec<[u32; 3]>>
}

impl ClusterMaker {
    pub fn new(k: u32) -> Self {
        ClusterMaker { k: k,
            data: Vec::new(),
            centroids: Vec::new(),
            clusters: HashMap::new() }
    }
    /// Adds the data to cluster with,
    pub fn with_data(mut self, data: &Vec<[u32;3]>) -> Self {
        self.data = data.clone(); //store original
        let mut clone = data.clone();
        clone.shuffle(&mut thread_rng()); //shuffled
        self

        //call functions to calculate first centroid and clusters
    }

    pub fn cluster(&mut self) {
        self.centroids = self.calculate_first_centroids();
        self.clusters = self.compute_clusters();
        self.get_cluster_avg(&self.centroids[0]);
    }

    /// First centroids are just random
    pub fn calculate_first_centroids(&self) -> Vec<[u32; 3]> {
        let mut i: usize = 0;
        let mut result = Vec::new();
        while i < self.k as usize{ //first k clusters
            let centroid = self.data[rand::thread_rng().gen_range(0..self.data.len())];
            if result.contains(&centroid) { //cant have repeating ones
                continue;
            }
            result.push(centroid);
            i += 1;
        }
        result
    }

    /// any except first iteration
    pub fn calculate_centroids(&self) -> Vec<[u32; 3]> {
        unimplemented!()
    }

    pub fn get_cluster_avg(&self, centroid: &[u32; 3]) -> [u32; 3] {
        let mut sum = [0, 0, 0];
        let mut count = 0; //wont hurt anybody
        self.clusters.get(centroid)
            .expect("failed to get ref to clusters in get_cluster_avg")
            .iter().for_each(|data_point|{ //get every rgb component
                for i in 0..sum.len() {
                    sum[i] += data_point[i];
                }
                count += 1;
                /*sum.iter_mut().enumerate().for_each(|(i, _)|{
                    sum[i] = data_point[i];
                })*/
            });
        if count == 0 { //average of centroid is centroid i guess
            return centroid.clone();
        }
        for i in 0..sum.len() {
            sum[i] = (sum[i] as f32 / count as f32) as u32;
        }
        println!("centroid: {:?}, data: {:?}, avg: {:?}",
            centroid, 
            self.clusters.get(centroid).unwrap(),
            sum);
        sum
    }

    pub fn compute_clusters(&self) -> HashMap<[u32;3], Vec<[u32;3]>> {
        let mut result = HashMap::new();
        self.centroids.iter().for_each(|centroid| { //start with every centroid as key
            result.insert(*centroid, Vec::new());
        });
        self.data.iter().for_each(|data_point|{
            //println!("data: {:?}, distances: {:?}", data_point, self.calculate_centroid_distance(data_point));
            let distances = self.calculate_centroid_distance(data_point);
            let closest = distances.iter()
                .position(|dist| dist == distances.iter().min()
                    .expect("Couldnt find smallest distance"))
                .expect("Couldnt find position in vec");
            //println!("data: {:?}, closest: {:?}", data_point, self.centroids[closest]);
            result.get_mut(&self.centroids[closest])
                .expect("failed to get mut ref in compute_clusters")
                .push(*data_point);
        });
        result
    }

    pub fn calculate_centroid_distance(&self, data_point: &[u32; 3]) -> Vec<u32> {
        self.centroids.iter().map(|centroid| {
            /*println!("\tdata:{:?}, centroid: {:?}, dist: {:?}",
                data_point,
                centroid,
                data_point.get_distance_u32(centroid)
            );*/
            data_point.get_distance_u32(centroid)
        }).collect()
    }
}


#[cfg(test)]
mod test {
    use rand::thread_rng;
    use rand::prelude::SliceRandom;

    use super::ClusterMaker;

    #[test]
    fn test_engine() {
        let mut test_data = Vec::from([
            [0, 0, 0],
            [100, 100, 100],
            [120, 0, 255],
            [255, 255, 255],
            [0, 100, 0]
        ]);
        let mut cluster = ClusterMaker::new(3)
            .with_data(&test_data);
        cluster.cluster();
    }
}