use crate::agent::SpinAgent;
use rand::Rng;

pub struct IsingEnvironment {
    pub lattice: Vec<Vec<SpinAgent>>,
    pub size: usize,
    pub temperature: f64,
    pub j_coupling: f64,
    pub energy_history: Vec<f64>,
    pub magnetization_history: Vec<f64>,
}

impl IsingEnvironment {
    pub fn new(size: usize, temperature: f64, j_coupling: f64) -> Self {
        let lattice = (0..size)
            .map(|_| (0..size).map(|_| SpinAgent::new()).collect())
            .collect();

        Self {
            lattice,
            size,
            temperature,
            j_coupling,
            energy_history: Vec::new(),
            magnetization_history: Vec::new(),
        }
    }

    pub fn calculate_site_energy(&self, i: usize, j: usize) -> f64 {
        let current_spin = self.lattice[i][j].spin;
        let mut energy = 0.0;

        // 计算与邻居的相互作用
        let neighbors = [
            ((i + self.size - 1) % self.size, j),  // 上
            ((i + 1) % self.size, j),              // 下
            (i, (j + self.size - 1) % self.size),  // 左
            (i, (j + 1) % self.size),              // 右
        ];

        for (ni, nj) in neighbors {
            energy -= self.j_coupling as f64 * current_spin as f64 * self.lattice[ni][nj].spin as f64;
        }

        energy
    }

    pub fn calculate_total_energy(&self) -> f64 {
        let mut total_energy = 0.0;
        for i in 0..self.size {
            for j in 0..self.size {
                total_energy += self.calculate_site_energy(i, j);
            }
        }
        total_energy / 2.0  // 因为每个相互作用被计算了两次
    }

    pub fn calculate_magnetization(&self) -> f64 {
        let total_spin: i32 = self.lattice
            .iter()
            .flat_map(|row| row.iter())
            .map(|agent| agent.spin as i32)
            .sum();
        total_spin as f64 / (self.size * self.size) as f64
    }

    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();
        
        // 随机选择一个位置
        let i = rng.gen_range(0..self.size);
        let j = rng.gen_range(0..self.size);

        // 计算翻转前的能量
        let energy_before = self.calculate_site_energy(i, j);
        
        // 翻转自旋
        self.lattice[i][j].flip();
        
        // 计算翻转后的能量
        let energy_after = self.calculate_site_energy(i, j);
        
        // 计算能量差
        let delta_e = energy_after - energy_before;
        
        // Metropolis算法
        if delta_e > 0.0 && rng.gen::<f64>() > (-delta_e / self.temperature).exp() {
            // 如果不接受翻转，就改回来
            self.lattice[i][j].flip();
        }

        // 记录能量和磁化强度历史
        self.energy_history.push(self.calculate_total_energy());
        self.magnetization_history.push(self.calculate_magnetization());
    }
} 