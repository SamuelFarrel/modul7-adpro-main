use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, Subscriber> = DashMap::new();
}

pub struct SubscribersRepository;

impl SubscribersRepository {
}