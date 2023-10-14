use rand::prelude::*;
use crate::ai::ec::aco::ant::{Ant, AntColony};

/// This is a simple example of the use of Ant Colony Optimization

pub struct Ant {
  pub id: usize,
  pub path: Vec<usize>,
  pub path_length: f64,
}

pub struct AntColony {
  pub ants: Vec<Ant>,
  pub pheromones: Vec<Vec<f64>>,
  pub distances: Vec<Vec<f64>>,
  pub num_ants: usize,
  pub num_nodes: usize,
  pub alpha: f64,
  pub beta: f64,
  pub rho: f64,
  pub q: f64,
  pub init_pheromone: f64,
  pub max_iterations: usize,
  pub best_path: Vec<usize>,
  pub best_path_length: f64,
}

impl AntColony {
  pub fn new(
    num_ants: usize,
    num_nodes: usize,
    alpha: f64,
    beta: f64,
    rho: f64,
    q: f64,
    init_pheromone: f64,
    max_iterations: usize,
  ) -> AntColony {
    let mut ants = Vec::with_capacity(num_ants);
    for i in 0..num_ants {
      ants.push(Ant {
        id: i,
        path: Vec::with_capacity(num_nodes),
        path_length: 0.0,
      });
    }
    let mut pheromones = Vec::with_capacity(num_nodes);
    for _ in 0..num_nodes {
      pheromones.push(vec![init_pheromone; num_nodes]);
    }
    let mut distances = Vec::with_capacity(num_nodes);
    for _ in 0..num_nodes {
      distances.push(vec![0.0; num_nodes]);
    }
    AntColony {
      ants,
      pheromones,
      distances,
      num_ants,
      num_nodes,
      alpha,
      beta,
      rho,
      q,
      init_pheromone,
      max_iterations,
      best_path: Vec::with_capacity(num_nodes),
      best_path_length: 0.0,
    }
  }

  pub fn run(&mut self) {
    let mut rng = thread_rng();
    for i in 0..self.max_iterations {
      self.clear_ants();
      self.move_ants();
      self.update_best();
      self.update_pheromones();
      println!("Iteration: {}, Best Path Length: {}", i, self.best_path_length);
    }
  }

  fn clear_ants(&mut self) {
    for ant in &mut self.ants {
      ant.path.clear();
      ant.path_length = 0.0;
    }
  }

  fn move_ants(&mut self) {
    for _ in 1..self.num_nodes {
      for ant in &mut self.ants {
        let next = self.select_next(ant);
        ant.path.push(next);
        ant.path_length += self.distances[ant.path[ant.path.len() - 2]][next];
      }
    }
  }

  fn select_next(&self, ant: &Ant) -> usize {
    let mut rng = thread_rng();
    let mut denom = 0.0;
    for i in 0..self.num_nodes {
      if !ant.path.contains(&i) {
        denom += self.pheromones[ant.path[ant.path.len() - 1]][i].powf(self.alpha) * self.distances[ant.path[ant.path.len() - 1]][i].powf(self.beta);
      }
    }
    let mut probs = Vec::with_capacity(self.num_nodes);
    for i in 0..self.num_nodes {
      if ant.path.contains(&i) {
        probs.push(0.0);
      } else {
        let numerator = self.pheromones[ant.path[ant.path.len() - 1]][i].powf(self.alpha) * self.distances[ant.path[ant.path.len() - 1]][i].powf(self.beta);
        probs.push(numerator / denom);
      }
    }
    let mut cum = Vec::with_capacity(self.num_nodes);
    let mut cum_sum = 0.0;
    for i in 0..self.num_nodes {
      cum_sum += probs[i];
      cum.push(cum_sum);
    }
    let rand = rng.gen::<f64>();
    for i in 0..self.num_nodes {
      if rand < cum[i] {
        return i;
      }
    }
    return 0;
  }
}