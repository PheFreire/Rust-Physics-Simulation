use super::Particle;

// Spring Mass Model
pub struct Spring {
    particle_a: Particle,
    particle_b: Particle,
    stiffiness: f32,
    rest_lenght: f32,
    damping_factor: f32,
}

impl Spring {
    fn new(
        &self,
        particle_a: Particle,
        particle_b: Particle,
        stiffiness: f32,
        rest_lenght: f32,
        damping_factor: f32,
    ) -> Spring {
        Spring {
            particle_a,
            particle_b,
            stiffiness,
            rest_lenght,
            damping_factor,
        } 
    }

    
    fn get_spring_deformation_force(&self) -> f32 {
        // Hooke's law

        let x = {
            let particles_distance = self.particle_b.space - self.particle_a.space;  
            particles_distance.abs() - self.rest_lenght
        };
        
        x * self.stiffiness 
    }

    fn get_spring_damping_force(&self) -> f32 {
        // normalize_points_direction_vectors = (B_s - A_s / |B_s - A_s|)
        // vectors_difference = (B_v - A_b)
        // vectors_dot_product = normalize_points_direction_vectors * vectors_difference
        // damping_force = vectors_dot_product * damping_factor 

        let vectors_dot_product = {
            let particles_distance = self.particle_b.space - self.particle_a.space;
            let normalize_points_direction_vectors = particles_distance / particles_distance.abs();
            let vectors_difference = self.particle_b.velocity - self.particle_a.velocity;

            normalize_points_direction_vectors * vectors_difference
        };

        vectors_dot_product * self.damping_factor
    }

    fn get_total_spring_force(&self) -> f32 {
        // Sum between spring_deformation_force and spring_damping_force

        self.get_spring_deformation_force() + self.get_spring_damping_force()
    }

    fn get_particle_vector_force(&self) -> f32 {
        // normalize_points_direction_vectors = (B_s - A_s / |B_s - A_s|)
        // Fb = Ft * normalize_points_direction_vectors
        // if the results is positive the particles moving towards each other
        // if the results is negative the particles moving away each other 
        let particles_distance = self.particle_b.space - self.particle_a.space;
        let normalize_points_direction_vectors = particles_distance / particles_distance.abs();

        self.get_total_spring_force() * normalize_points_direction_vectors 
    }
}

