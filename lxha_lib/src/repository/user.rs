
use std::vec;
use std::sync::Arc;
use futures::TryStreamExt;
use axum_responses::AxumResult;
use mongodb::{Collection, bson::{doc, oid::ObjectId, Document}};

use crate::app::DatabaseReference;
use crate::{models::user::*, utils::dbg::handle_internal_sv_error};

#[derive(Debug, Clone)]
pub struct UserRespository {
    collection: Collection<User>,
}

impl UserRespository {
    
    pub fn new(db: DatabaseReference) -> Self {
        UserRespository {
            collection: Arc::clone(&db).collection::<User>("users"),
        }
    }

    pub async fn create(&self, user: &User) -> AxumResult<()> {
        
        self.collection.insert_one(user, None).await
            .map_err(|e| handle_internal_sv_error(e))?
        ;

        Ok(())
    }

    pub async fn find_one_by_id(&self, id: &ObjectId) -> AxumResult<Option<User>> {

        let user = self.collection.find_one(doc! { "_id": id }, None).await
            .map_err(|e| handle_internal_sv_error(e))?
        ;

        Ok(user)
    }

    pub async fn find_one(&self, filter: Document) -> AxumResult<Option<User>> {

        let user = self.collection.find_one(filter, None).await
            .map_err(|e| handle_internal_sv_error(e))?
        ;

        Ok(user)
    }

    pub async fn find(&self) -> AxumResult<Vec<PrivateProfile>> {

        let mut profiles: Vec<PrivateProfile> = vec![];
        let mut cursor = self.collection.find(None, None).await
            .map_err(|e| handle_internal_sv_error(e))?
        ;

        while let Some(user) = cursor.try_next().await
            .map_err(|e| handle_internal_sv_error(e))? {

            profiles.push(user.into_priv_profile())
        }

        Ok(profiles)
    }

    pub async fn update(&self, id: &ObjectId, update: Document) -> AxumResult<()> {

        self.collection.update_one(doc! { "_id": id }, update, None).await
            .map_err(|e| handle_internal_sv_error(e))?
        ;

        Ok(())
    }

    pub async fn delete(&self, id: &ObjectId) -> AxumResult<()> {

        self.collection.delete_one(doc! { "_id": id }, None).await
            .map_err(|e| handle_internal_sv_error(e))?
        ;

        Ok(())
    }
}

