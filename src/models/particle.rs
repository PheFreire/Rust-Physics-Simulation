// Kinematics Variables
pub struct Particle {
    pub space: f32,
    pub velocity: f32,
    pub mass: f32,
    pub force: f32,
}

impl Particle {
    fn new(
        space: f32,
        velocity: f32,
        mass: f32,
        force: f32,
    ) -> Particle {
        Particle { 
            space,
            velocity,
            mass,
            force,
        }
    }

    fn update_particle(&mut self, acceleration: f32, del_time: f32) {
        // Second newton law
        let new_force = self.mass * acceleration;

        // Euler integration
        // TODO
        // Discover del_time parameter
        self.velocity += (new_force*del_time)/self.mass;
        self.space += self.velocity * del_time;
    }
}