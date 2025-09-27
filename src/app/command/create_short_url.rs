use crate::{app::command::create_short_url, id_provider::IdProvider};
pub trait CreateShortUrlReposoitory {
    fn save(&self, id: String, full_url: String, id: String) -> Result<(), String>;
}
pub struct CreateShortUrl<I, R>
where
    I: IdProvider,
    R: CreateShortUrlReposoitory,
{
    id_provider: I,
    repo: R,
}

impl<I, R> CreateShortUrl<I, R>
where
    I: IdProvider,
    R: CreateShortUrlReposoitory,
{
    pub fn new(id_provider: I, repo: R) -> Self {
        Self { id_provider, repo }
    }

    pub async fn execute(&self, full_url: String) -> Result<String, String> {
        let id = self.id_provider.provide();
        self.repo.save(id.clone(), full_url, id.clone())?;
        //Ok("1".to_owned()) // Placeholder implementation
        //format!("short.ly/{}", original_url) // Placeholder implementation
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use crate::id_provider;

    use super::*;

    #[tokio::test]
    async fn get_short_url() {
        //given
        let id_provider = id_provider::FakeIdProvider::new("abc123".to_owned());
        let command = CreateShortUrl::new(id_provider);
        //when
        let result = command.execute("https://google.com".to_owned()).await;
        //then
        //assert!(result.is_ok());
        assert_ne!(result, Ok("".to_owned()));
    }
    #[tokio::test]
    async fn get_two_different_short_url() {
        //given

        let idp = crate::id_provider::NanoIdProvider;
        let command = CreateShortUrl::new(idp);
        //when
        let result = command.execute("https://google.com".to_owned()).await;
        let result2 = command.execute("https://yandex.ru".to_owned()).await;
        //then
        assert_ne!(result, result2);
    }
}
