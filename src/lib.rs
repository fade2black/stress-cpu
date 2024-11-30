use rand::{thread_rng, Rng};
use tokio::{
    sync::oneshot::{self, error::TryRecvError, Receiver},
    time::Duration,
};

fn compute(mut rx: Receiver<()>) {
    let mut rng = thread_rng();
    let num = rng.gen_range(1975..2005) as f64;

    while let Err(TryRecvError::Empty) = rx.try_recv() {
        let _ = f64::sqrt(num);
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
