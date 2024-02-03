use core::num;
use std::borrow::Borrow;
use std::{env, string};
use std::fs;
use std::collections::HashMap;

use petgraph::data::Build;
use petgraph::graph::Graph;
use petgraph::Directed;

fn main() {
    let filePath: &str = "in.txt";
    let mut gameGraph = Graph::<String, i32>::new();

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
    let mut nodeMap: HashMap<String, petgraph::prelude::NodeIndex> = HashMap::new();
    //let mut nodeMap: HashMap<String, graph.nodeMap> = HashMap::new();
    let end = gameGraph.add_node("end".to_string());
    for i in 1..numVertices+1{
        for j in 1..numVertices+1{
            let node = gameGraph.add_node(format!("{:?},{:?}",i,j));
            nodeMap.insert(format!("{:?},{:?}",i,j), node);
            if i == numVertices || j == numVertices{
                gameGraph.add_edge(node, end, 0);
            }
        }
    }


    //create goal vertice
    //gameGraph.add_node("end");

    //calculate edges
    let mut index = 0;
    //let mut startNode = "".to_string();
    let mut endNode = "".to_string();
    let mut startNode = format!("balls");
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
                    //println!("Found available edges: {:?},{:?} to {:?},{:?} and {:?},{:?} to {:?},{:?}", start,verticeNum,end,verticeNum,verticeNum,start,verticeNum,end);
                    //startNode = format!("{:?},{:?}",start,verticeNum);
                    gameGraph.add_edge(*nodeMap.get(&format!("{:?},{:?}",start,verticeNum)).unwrap(), *nodeMap.get(&format!("{:?},{:?}",end,verticeNum)).unwrap(), 1);
                    gameGraph.add_edge(*nodeMap.get(&format!("{:?},{:?}",verticeNum,start)).unwrap(), *nodeMap.get(&format!("{:?},{:?}",verticeNum,end)).unwrap(), 1);
                }
                verticeNum+=1;
            }
        }
        index+=1;
    }
}
