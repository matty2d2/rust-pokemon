macro_rules! endpoint {
    ($type:ty; for $name:literal) => {
        use cached::proc_macro::cached;
        use reqwest::Url;

        const ENDPOINT: &str = concat!("https:///pokeapi.co/api/v2/", $name, "/");

        #[cached(result = true, time = 259200)]
        pub async fn get_by_id(id: i64) -> Result<$type, reqwest::Error> {
            let url = Url::parse(ENDPOINT).unwrap().join(&id.to_string()).unwrap();
            reqwest::get(url).await?.json::<$type>().await
        }

        #[cached(result = true, time = 259200)]
        pub async fn get_by_name(name: &'static str) -> Result<$type, reqwest::Error> {
            let url = Url::parse(ENDPOINT).unwrap().join(name).unwrap();
            reqwest::get(url).await?.json::<$type>().await
        }
    };

    ($type:ty; for $name:literal; with $(($sub:ident, $sub_type:ty))+) => {

        crate::endpoint!($type; for $name);

        $(
            pub mod $sub {

                use cached::proc_macro::cached;
                use reqwest::Url;

                use super::ENDPOINT;

                const SUB_ENDPOINT: &str = stringify!($sub);

                #[cached(result = true, time = 259200)]
                pub async fn get_by_id(id: i64) -> Result<$type, reqwest::Error> {
                    let sub_path = format!("{}/{}", id, SUB_ENDPOINT);
                    let url = Url::parse(ENDPOINT).unwrap().join(&sub_path).unwrap();
                    reqwest::get(url).await?.json::<$type>().await
                }

                #[cached(result = true, time = 259200)]
                pub async fn get_by_name(name: &'static str) -> Result<$type, reqwest::Error> {
                    let sub_path = format!("{}/{}", name, SUB_ENDPOINT);
                    let url = Url::parse(ENDPOINT).unwrap().join(&sub_path).unwrap();
                    reqwest::get(url).await?.json::<$type>().await
                }
            }
        )+
    };
}

pub(crate) use endpoint;