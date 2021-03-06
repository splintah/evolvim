#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OutputType {
    MouthHue,
    Eating,
    Turning,
    Accelerating,
    Fight,
}

impl OutputType {
    pub fn use_output<B>(
        &self,
        value: f64,
        env: &mut crate::brain::EnvironmentMut<B>,
        time_step: f64,
    ) {
        use OutputType::*;

        match self {
            MouthHue => env.this_body.set_mouth_hue(value),
            Eating => {
                let tile_pos = env.this_body.get_random_covered_tile(env.board_size);
                let tile = env.terrain.get_tile_at_mut(tile_pos);
                env.this_body
                    .eat(value, time_step, env.time, env.climate, tile);
            }
            Turning => env.this_body.turn(value, time_step),
            Accelerating => env.this_body.accelerate(value, time_step),
            Fight => env.this_body.fight(
                value,
                env.time,
                time_step,
                env.sbip,
                env.self_pointer.clone(),
            ),
        };
    }
}
