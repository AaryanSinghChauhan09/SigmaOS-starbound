//! starbound model — Starbound dedicated server.

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub uuid: String,
    pub account: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub players: Vec<Player>,
    pub universe: String,
    pub running: bool,
}

impl Default for Model {
    fn default() -> Self { Self { players: vec![], universe: "Starbound".into(), running: false } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn start(&mut self) -> bool {
        if !self.running { self.running = true; true } else { false }
    }

    pub fn join(&mut self, player: Player) -> bool {
        if self.running { self.players.push(player); true } else { false }
    }

    pub fn list_players(&self) -> &[Player] { &self.players }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn start_join() {
        let mut s = Model::new();
        assert!(s.start());
        assert!(s.join(Player { name: "P1".into(), uuid: "u1".into(), account: "a1".into() }));
        assert_eq!(s.list_players().len(), 1);
    }
}
