use rand::prelude::*;
use rand::thread_rng;
use rand::prelude::SliceRandom;

use std::collections::HashMap;
use std::hash::Hash;

use std::io::Cursor;
use std::fs::File;
use std::io::Write;
use image::io::Reader;
use image::Pixel;
use image::GenericImageView;
use image::DynamicImage;

fn process_image() {
    let img = Reader::open("test.jpg")
        .expect("failed to load image test.jpg")
        .decode()
        .expect("failed to decode image");
        let pixels: Vec<[u32; 3]> = img.pixels().map(|(x, y, pixel)| {
            [pixel.channels()[0] as u32
            , pixel.channels()[1] as u32
            , pixel.channels()[2] as u32]
            //ignore alpha
        }).collect();
        println!("pixels: {:?}", pixels.len());
        let mut c_maker = ClusterMaker::new(5)
            .with_data(&pixels);
        
        c_maker.cluster(5);
        let results: Vec<[u32; 3]> = c_maker.clusters.iter().map(|cluster| {
            cluster.centroid
        }).collect();
        let mut file = File::create("image_dump.txt").expect("failed to create dump file");
        file.write_all(
            //format!("{:?}", c_maker.clusters.keys()).as_bytes()
            //format!("{:?}", c_maker.clusters).as_bytes()
            format!("{:?}", results).as_bytes()
        ).expect("Failed to write to file");
}

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

/// first, with data cloning, bad performance maybe
#[derive(Debug)]
pub struct ClusterMaker {
    pub k: u32,
    pub data: Vec<[u32; 3]>,
    pub clusters: Vec<Cluster>
}

impl ClusterMaker {
    pub fn new(k: u32) -> Self {
        ClusterMaker { k: k,
            data: Vec::new(),
            clusters: Vec::new()}
    }
    /// Adds the data to cluster with,
    pub fn with_data(mut self, data: &Vec<[u32;3]>) -> Self {
        self.data = data.clone(); //store original
        /*let mut clone = data.clone();
        clone.shuffle(&mut thread_rng()); //shuffled*/
        self
    }

    pub fn cluster(&mut self, i: u32) {
        if i <= 0 {
            panic!("cant cluster in 0 iterations");
        }
        self.init();
        self.compute_clusters();
        if i == 1 {
            return
        }
        for i in 0..(i - 1) {
            self.update_centroids();
            self.compute_clusters();
        }        
    }

    /// First centroids are just random
    /*pub fn calculate_first_centroids(&self) -> Vec<[u32; 3]> {
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
    }*/

    pub fn init(&mut self) {
        println!("{:?}", "init!");
        let mut i: usize = 0;
        while i < self.k as usize {
            let centroid = self.data[rand::thread_rng().gen_range(0..self.data.len())];
            let mut repeated = false;
            self.clusters.iter().for_each(|cluster| {
                if cluster.centroid == centroid { //repeated
                    repeated = true;
                }
            });
            if repeated {
                continue;
            }
            println!("k: {:?}, i: {:?}, {:?}", self.k, i, centroid);
            self.clusters.push(Cluster::new(centroid));
            i += 1;
        }
    }

    /// any except first iteration
    /*pub fn calculate_centroids(&self) -> Vec<[u32; 3]> {
        self.centroids.iter().map(|centroid|{
            self.get_cluster_avg(centroid)
        }).collect()
    }*/
    /*/
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
            });
        if count == 0 { //average of centroid is centroid i guess
            return centroid.clone();
        }
        for i in 0..sum.len() {
            sum[i] = (sum[i] as f32 / count as f32) as u32;
        }
        sum
    }*/

    pub fn compute_clusters(&mut self) {
        //for each data point, calculate each distance to each cluster
        //get the smallest distance, its the closest cluster
        //assign datapoint to cluster
        //assign new centroids
        self.data.iter().for_each(|data_point| {
            let mut closest_distance = u32::MAX;
            let mut closest_centroid = self.clusters[0].centroid;
            self.clusters.iter().for_each(|cluster| {
                let distance = data_point.get_distance_u32(&cluster.centroid);
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_centroid = cluster.centroid;
                }
            });
            //find cluster with given centroid
            self.clusters.iter_mut().for_each(|cluster| {
                if cluster.centroid == closest_centroid {
                    cluster.add(*data_point);
                }
            })
        }); 
        // at the end the centroids are ready for an update
    }

    pub fn update_centroids(&mut self) {
        self.clusters.iter_mut().for_each(|centroid| {
            centroid.update_centroid();
            centroid.clean();
        })
    }

    /*fn add_to_avg(&mut self, centroid: &[u32; 3], data_point: &[u32; 3]) -> [u32;3]{
        let (mut a, mut b) = self.current_avgs[centroid];
        a[0] += data_point[0];//rgb
        a[1] += data_point[1];
        a[2] += data_point[2];
        b += 1;
        data_point
    }*/

    pub fn calculate_centroid_distance(&self, data_point: &[u32; 3]) -> Vec<u32> {
        /*self.centroids.iter().map(|centroid| {
            data_point.get_distance_u32(centroid)
        }).collect()*/
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Cluster {
    pub centroid: [u32; 3],
    pub elements: Vec<[u32; 3]>,
    pub suma: [u32; 3]
}

impl Cluster {
    /// Cluster ignores its centroid as an element
    pub fn new(centroid: [u32; 3]) -> Self {
        Self{centroid: centroid, elements: Vec::new(), suma: [0, 0, 0]}
    }

    /// aggregate to this cluster
    pub fn add(&mut self, element: [u32; 3]) {
        self.suma[0] += element[0];
        self.suma[1] += element[1];
        self.suma[2] += element[2];
        self.elements.push(element);
    }

    /// average color for this cluster, ignoring centroid
    pub fn get_next_centroid(&self) -> [u32;3] {
        let count = self.elements.len() as u32;
        [self.suma[0] / count, self.suma[1] / count, self.suma[2] / count]
    }

    pub fn clean(&mut self) {
        self.elements = Vec::new();
        self.suma = [0, 0, 0];
    }

    pub fn update_centroid(&mut self) {
        self.centroid = self.get_next_centroid();
    }
}


#[cfg(test)]
mod test {
    use rand::thread_rng;
    use rand::prelude::SliceRandom;

    use super::{ClusterMaker, process_image};
    use super::Cluster;

    #[test]
    fn test_engine() {
        let mut test_data = Vec::from([
            [0, 0, 0],
            [100, 100, 100],
            [120, 0, 255],
            [255, 255, 255],
            [0, 100, 0]
        ]);
        let mut cluster = ClusterMaker::new(2)
            .with_data(&test_data);
        println!("{:?}", cluster);
        cluster.cluster(3);
        println!("{:?}", cluster);
    }

    #[test]
    fn test_processing() {
        process_image();
    }

    #[test]
    fn test_cluster() {
        let mut cluster = Cluster::new([0, 100, 200]);
        cluster.add([100, 0, 0]);
        cluster.add([0, 0, 100]);

        println!("cluster: {:?}, {:?}", cluster, cluster.get_next_centroid());
    }
}