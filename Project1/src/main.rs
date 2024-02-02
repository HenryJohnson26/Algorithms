use core::num;
use std::{env, string};
use std::fs;

use petgraph::graph::Graph;

fn main() {
    let filePath: &str = "in.txt";
    let mut gameGraph = Graph::<String, &str>::new();

    let contents = fs::read_to_string(filePath)
        .expect("Should have been able to read the file");
    let inputLines: Vec<&str> = contents.split('\n').collect();

    let firstLine: Vec<&str> = inputLines.get(0).unwrap()
        .get(0..inputLines.get(0).unwrap().len()-1).unwrap().split(' ').collect();
    //print!("{:?}",firstLine.get(1).unwrap().get(0..firstLine.get(1).unwrap().len()-1));
    let numEdges = firstLine.get(1).unwrap().parse::<i32>().unwrap();
    let numVertices = firstLine.get(0).unwrap().parse::<i32>().unwrap();
    //println!("{:?}",numEdges);

    let colorMap: Vec<&str> = inputLines.get(1).unwrap()
        .get(0..inputLines.get(1).unwrap().len()-1).unwrap().split(' ').collect();
    //println!("{:?}",colorMap);

    let thirdLine: Vec<&str> = inputLines.get(2).unwrap()
        .get(0..inputLines.get(2).unwrap().len()-1).unwrap().split(' ').collect();
    let startingState = format!("{:?},{:?}",thirdLine.get(0).unwrap().parse::<i32>().unwrap(),thirdLine.get(1).unwrap().parse::<i32>().unwrap());
    //print!("{:?}",startingState);

    //generate vertices
    for i in 1..numVertices+1{
        for j in 1..numVertices+1{
            //println!("{:?},{:?}",i,j);
            gameGraph.add_node(format!("{:?},{:?}",i,j));
        }
    }

    //create goal vertice
    gameGraph.add_node("end".to_string());

    //calculate edges
    let mut index = 0;
    for line in inputLines{
        if index > 2 && line != ""{
            let input: Vec<&str> = line.get(0..line.len()-1).unwrap().split(' ').collect();
            let start = input.get(0).unwrap().parse::<i32>().unwrap();
            let end = input.get(1).unwrap().parse::<i32>().unwrap();
            let color = input.get(2).unwrap();
            //println!("{:?},{:?} color:{:?}",start, end, color);
            let mut verticeNum = 1;
            for vertexColor in &colorMap{
                if vertexColor.to_string() == color.to_string(){
                    println!("Found available edges: {:?},{:?} to {:?},{:?} and {:?},{:?} to {:?},{:?}", start,verticeNum,end,verticeNum,verticeNum,start,verticeNum,end);
                }
                verticeNum+=1;
            }
        }
        index+=1;
    }
}
