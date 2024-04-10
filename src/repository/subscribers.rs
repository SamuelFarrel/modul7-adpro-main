use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, Subscriber> = DashMap::new();
}

pub struct SubscribersRepository;

impl SubscribersRepository {
    pub fn add(profuct_type: &str, subscriber: Subscriber) -> Subscriber {
        let subscriber_value = subscriber.clone();
        if SUBSCRIBERS.get(product_type).is_none(){
            SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
        };

        SUBSCRIBERS.get(product_type).unwrap()
            .insert(subscriber_value.url.clone(), subscriber_value);
        return subscriber;
    }
}