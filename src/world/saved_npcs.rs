use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SavedNPCs {
    pub saved_goblin_tinkerer: bool,
    pub saved_wizard: bool,
    pub saved_mechanic: bool,
    pub saved_angler: bool,
    pub saved_stylist: bool,
    pub saved_tax_collector: bool,
    pub saved_golfer: bool,
    pub saved_bartender: bool,
    pub saved_slime_nerdy: bool,
    pub saved_merchant: bool,
    pub saved_demolitionist: bool,
    pub saved_party_girl: bool,
    pub saved_dye_trader: bool,
    pub saved_truffle: bool,
    pub saved_arms_dealer: bool,
    pub saved_nurse: bool,
    pub saved_princess: bool,
    pub saved_slime_cool: bool,
    pub saved_slime_elder: bool,
    pub saved_slime_clumsy: bool,
    pub saved_slime_diva: bool,
    pub saved_slime_surly: bool,
    pub saved_slime_mystic: bool,
    pub saved_slime_squire: bool,
}

// write a function to maximize the saved NPCs
impl SavedNPCs {
    pub fn maximize(&mut self) {
        self.saved_goblin_tinkerer = true;
        self.saved_wizard = true;
        self.saved_mechanic = true;
        self.saved_angler = true;
        self.saved_stylist = true;
        self.saved_tax_collector = true;
        self.saved_golfer = true;
        self.saved_bartender = true;
        self.saved_slime_nerdy = true;
        self.saved_merchant = true;
        self.saved_demolitionist = true;
        self.saved_party_girl = true;
        self.saved_dye_trader = true;
        self.saved_truffle = true;
        self.saved_arms_dealer = true;
        self.saved_nurse = true;
        self.saved_princess = true;
        self.saved_slime_cool = true;
        self.saved_slime_elder = true;
        self.saved_slime_clumsy = true;
        self.saved_slime_diva = true;
        self.saved_slime_surly = true;
        self.saved_slime_mystic = true;
        self.saved_slime_squire = true;
    }
}
