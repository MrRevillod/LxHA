
use mongodb::bson;
use std::sync::Arc;
use futures::TryStreamExt;

use std::vec;
use serde_json::Value;

use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId, Document},
};

use axum_responses::{extra::ToJson, AxumResult, HttpResponse};

use crate::app::DatabaseReference;

use crate::models::user::*;

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
        
        self.collection
            .insert_one(user, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        Ok(())
    }

    pub async fn find_one_by_id(&self, id: &ObjectId) -> AxumResult<Option<User>> {

        let user = self.collection
            .find_one(doc! { "_id": id }, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        Ok(user)
    }

    pub async fn find_one(&self, filter: Document) -> AxumResult<Option<User>> {

        let user = self.collection
            .find_one(filter, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        Ok(user)
    }

    pub async fn find(&self) -> AxumResult<Vec<Value>> {

        let mut profiles: Vec<Value> = vec![];
        let mut cursor = self.collection.find(None, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        while let Some(result) = cursor.try_next().await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)? {

            profiles.push(result.into_user_data().to_json())
        }

        Ok(profiles)
    }

    pub async fn update(&self, id: &ObjectId, update: Document) -> AxumResult<()> {

        self.collection
            .update_one(doc! { "_id": id }, update, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        Ok(())
    }

    pub async fn delete(&self, id: &ObjectId) -> AxumResult<()> {

        self.collection
            .delete_one(doc! { "_id": id }, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        Ok(())
    }
}


