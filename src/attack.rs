use std::sync::mpsc::Sender;

pub trait Attack {
    fn attack_on_curve(tx: Sender<String>);
    fn attack_on_prime(tx: Sender<String>);
}

pub struct CPA;

impl Attack for CPA {
    fn attack_on_curve(tx: Sender<String>) {
        todo!()
    }

    fn attack_on_prime(tx: Sender<String>) {
        todo!()
    }
}

pub struct CCA;

impl Attack for CCA {
    fn attack_on_curve(tx: Sender<String>) {
        todo!()
    }

    fn attack_on_prime(tx: Sender<String>) {
        todo!()
    }
}
