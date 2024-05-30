
use axum_responses::AxumResult;
use mongodb::{Collection, bson::Document};

use crate::models::token::Token;
use crate::app::DatabaseReference;
use crate::utils::dbg::handle_internal_sv_error;

#[derive(Debug, Clone)]
pub struct TokenRepository {
    collection: Collection<Token>
}

impl TokenRepository {

    pub fn new(db: DatabaseReference) -> Self {
        TokenRepository {
            collection: db.collection::<Token>("expired_token")
        }
    }

    pub async fn save(&self, token: &Token) -> AxumResult<()> {
        
        self.collection.insert_one(token, None).await
            .map_err(|e| handle_internal_sv_error(e))?
        ;

        Ok(())
    }

    pub async fn find_one(&self, filter: Document) -> AxumResult<Option<Token>> {

        let user = self.collection.find_one(filter, None).await
            .map_err(|e| handle_internal_sv_error(e))?
        ;

        Ok(user)
    }
}
