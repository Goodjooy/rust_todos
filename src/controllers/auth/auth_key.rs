use rocket::http::{Cookie, CookieJar};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthKey<T>
where
    T: Sized,
    T: Serialize,
{
    death_time: SystemTime,
    life_time: Duration,
    death: bool,
    #[serde(bound(deserialize = " T: Deserialize<'de>"))]
    data: T,
}

impl<T> AuthKey<T>
where
    T: Serialize + DeserializeOwned + Sized,
{
    pub fn load_death_time(life_time: Duration) -> SystemTime {
        let now = SystemTime::now();
        now + life_time
    }

    pub fn new(data: T, life_time: Duration) -> Self {
        let death_time = Self::load_death_time(life_time);

        Self {
            death_time,
            data,
            death: false,
            life_time,
        }
    }

    pub fn new_cookie(name: &str, data: T, life_time: Duration) -> Cookie {
        let d = Self::new(data, life_time);
        let value = serde_json::to_string(&d).expect("Failure Prase To Json");
        Cookie::new(name, value)
    }

    pub fn from_cookie<'r>(cookie: Cookie<'r>, name: &'static str, jar: &CookieJar<'r>) -> Option<T>
    where
        T: Clone,
    {
        let value = cookie.value();
        let ad = serde_json::from_str::<AuthKey<T>>(value).ok()?;
        let data = ad.data.clone();

        if SystemTime::now() > ad.death_time || ad.death {
            None
        } else {
            // refeash cookie
            let ck = Self::new_cookie(name, ad.data, ad.life_time);
            jar.add_private(ck);

            Some(data)
        }
    }
    pub fn kill(&mut self) {
        self.death = true;
    }
}
