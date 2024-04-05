pub struct TurnResult {
    turn_nr: i64,
    blue_units: i64,
    red_units: i64,
    blue_casualties: i64,
    red_casualties: i64,
}

impl TurnResult {
    pub fn new(
        turn_nr: i64,
        blue_units: i64,
        red_units: i64,
        blue_casualties: i64,
        red_casualties: i64,
    ) -> Self {
        Self {
            turn_nr,
            blue_units,
            red_units,
            blue_casualties,
            red_casualties,
        }
    }

    pub fn turn_nr(&self) -> i64 {
        self.turn_nr
    }

    pub fn blue_units(&self) -> i64 {
        self.blue_units
    }

    pub fn red_units(&self) -> i64 {
        self.red_units
    }

    pub fn blue_casualties(&self) -> i64 {
        self.blue_casualties
    }

    pub fn red_casualties(&self) -> i64 {
        self.red_casualties
    }
}

pub struct SimulationResult {
    blue_attrition_coefficient: f32,
    red_attrition_coefficient: f32,
    engagements: i64,
    blue_units: i64,
    red_units: i64,
    results: Vec<TurnResult>,
}

impl SimulationResult {
    pub fn new(
        blue_attrition_coefficient: f32,
        red_attrition_coefficient: f32,
        engagements: i64,
        blue_units: i64,
        red_units: i64,
        results: Vec<TurnResult>,
    ) -> Self {
        Self {
            blue_attrition_coefficient,
            red_attrition_coefficient,
            engagements,
            blue_units,
            red_units,
            results,
        }
    }

    pub fn blue_attrition_coefficient(&self) -> f32 {
        self.blue_attrition_coefficient
    }

    pub fn red_attrition_coefficient(&self) -> f32 {
        self.red_attrition_coefficient
    }

    pub fn engagements(&self) -> i64 {
        self.engagements
    }

    pub fn blue_units(&self) -> i64 {
        self.blue_units
    }

    pub fn red_units(&self) -> i64 {
        self.red_units
    }

    pub fn turn_results(&self) -> &Vec<TurnResult> {
        &self.results
    }
}

pub struct Battlefield {
    blue_units: i64,
    red_units: i64,
    blue_ac: f32,
    red_ac: f32,
}

impl Battlefield {
    pub fn new(blue_units: i64, red_units: i64, blue_ac: f32, red_ac: f32) -> Self {
        Self {
            blue_units,
            red_units,
            blue_ac,
            red_ac,
        }
    }

    pub fn blue_units(&self) -> i64 {
        self.blue_units
    }

    pub fn red_units(&self) -> i64 {
        self.red_units
    }

    pub fn simulate_turn(&mut self) -> (i64, i64) {
        let blue_casualties = (self.red_units as f32 * self.red_ac).ceil() as i64;
        let red_casualties = (self.blue_units as f32 * self.blue_ac).ceil() as i64;

        self.blue_units -= blue_casualties;
        self.red_units -= red_casualties;

        if self.blue_units < 0 {
            self.blue_units = 0;
        }

        if self.red_units < 0 {
            self.red_units = 0;
        }

        (blue_casualties, red_casualties)
    }
}
