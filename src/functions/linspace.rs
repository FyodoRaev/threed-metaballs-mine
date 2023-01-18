use crate::functions::marching_cubes;
use crate::functions::floatIterator::FloatIterator;

static triTable: &[&[usize]] = marching_cubes::TABLE;
static cornerIndexAFromEdge: &[usize] = marching_cubes::cornerIndexAFromEdge;
static cornerIndexBFromEdge: &[usize] = marching_cubes::cornerIndexBFromEdge;


pub struct Linspace {
    step: f64,
    len: f64,
    cubes: Vec<[(f64, f64, f64); 8]>,
}

impl Linspace {
    pub fn new(step: f64, len: f64) -> Linspace {
        //creating cubes
        let mut cubes: Vec<[(f64, f64, f64); 8]> = Vec::new();
        for i in FloatIterator::new_with_step(-len/2.0, len/2.0, step) {
            let x: f64 = i;
            for j in FloatIterator::new_with_step(-len/2.0, len/2.0, step) {
                let y: f64 = j;
                for l in FloatIterator::new_with_step(-len/2.0, len/2.0, step) {
                    let z: f64 = l;
                    let cube = [
                        (x, y, z),
                        (x + step, y, z),
                        (x + step, y, z + step),
                        (x, y, z + step),
                        (x, y + step, z),
                        (x + step, y + step, z),
                        (x + step, y + step, z + step),
                        (x, y + step, z + step),
                    ];
                    cubes.push(cube);
                }
            }
        }
        return Linspace {
            step,
            len,
            cubes,
        };
    }
}

impl Linspace {
    pub fn getVerticesCoordsIndexes(&self, circleCenters: &Vec<(f64, f64, f64)>, circleRad: &f64) -> Vec<((f64, f64, f64), i32)> {
        let mut vertexPositions: Vec<((f64, f64, f64), i32)> = Vec::new();
        let mut index = 1;
        let cubes = &self.cubes;
        for cube in cubes {
            let mut cubeIndex = 0;
            for i in 0..8 {
                let power = i as u32;
                if cube[i].2 == 0.0 {
                    cubeIndex += u32::pow(2, power) as usize;
                }
            }
        println!("{}", cubeIndex);
        let triangulation = triTable[cubeIndex]; 
        for edgeIndex in triangulation {
                // Ищу координаты ребер формирующих ребро, которое должно быть закрашено согласно триангуляции
                let indexA = cornerIndexAFromEdge[*edgeIndex];
                let indexB = cornerIndexBFromEdge[*edgeIndex];

                // точка на ребре которая должна быть включена в треугольник который нужно закрасить
                let mean_x = (cube[indexA].0 + cube[indexB].0) /2.0;
                let mean_y = (cube[indexA].1 + cube[indexB].1) /2.0;
                let mean_z = (cube[indexA].2 + cube[indexB].2) /2.0;
                let vertexPos: (f64, f64, f64) = (mean_x, mean_y, mean_z);
                vertexPositions.push((vertexPos, index)); // I have to store indexes of vertices
                index+=1; 
            }
        }
        return vertexPositions;
    }
}

pub fn sphere(point: (f64, f64, f64), circleCenters: &Vec<(f64, f64, f64)>) -> f64 {
    let mut sphereFunc: f64 = 0.0;
    for i in 0..circleCenters.len() {
    sphereFunc += f64::powf((point.0 - circleCenters[i].0), 2.0) +
                  f64::powf((point.1 - circleCenters[i].1), 2.0) +
                  f64::powf((point.2 - circleCenters[i].2), 2.0);}
    return sphereFunc;
}